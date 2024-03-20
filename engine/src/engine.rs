use crate::engine::exports::inf::wasi::engine::Guest;
use crate::engine::exports::inf::wasi::engine::GuestEngine;
use crate::engine::inf::wasi::config_types::Config;
use serde_json::json;
use std::path::PathBuf;
use wasi_nn::graph;
use wasi_nn::tensor;

wit_bindgen::generate!({
    path: "../wit",
    world: "engine-world",
});

pub struct EngineImpl {
    pub model_path: String,
    pub prompt: String,
}

impl Guest for EngineImpl {
    type Engine = EngineImpl;
}

impl GuestEngine for EngineImpl {
    fn new(config: Config) -> Self {
        Self {
            model_path: config.model_path,
            prompt: config.prompt,
        }
    }

    fn version() -> String {
        crate::version().to_string()
    }

    fn inference(&self, prompt: String) -> String {
        let model_path = PathBuf::from(&self.model_path);

        let graph_builder: graph::GraphBuilder = model_path.to_str().unwrap().as_bytes().to_vec();
        let builders = vec![graph_builder];

        let graph = graph::load(
            &builders,
            graph::GraphEncoding::Ggml,
            graph::ExecutionTarget::Cpu,
        )
        .unwrap();

        let context = wasi_nn::inference::init_execution_context(graph).unwrap();

        println!("Engine model_path: {}", &self.model_path);
        println!("Engine prompt: {}", &self.prompt);

        // TODO: all these options should be part of the configuration object
        // is some way. This needs to be figured out.
        let options = json!({
            "stream-stdout": true,
            "enable-log": true,
            "ctx-size": 1024,
            "n-predict": 80,
            "n-gpu-layers": 25
        });

        let options_tensor = wasi_nn::tensor::Tensor {
            dimensions: vec![1_u32],
            tensor_type: tensor::TensorType::U8,
            data: options.to_string().as_bytes().to_vec(),
        };

        wasi_nn::inference::set_input(context, 0, &options_tensor).unwrap();
        //context.set_input("options", options_tensor).unwrap();

        let prompt = &self.prompt;
        let prompt_tensor = tensor::Tensor {
            dimensions: vec![1_u32],
            tensor_type: tensor::TensorType::U8,
            data: prompt.as_bytes().to_vec(),
        };
        //context.set_input("prompt", prompt_tensor).unwrap();
        wasi_nn::inference::set_input(context, 1, &prompt_tensor).unwrap();

        //context.compute().unwrap();
        wasi_nn::inference::compute(context).unwrap();
        let output = wasi_nn::inference::get_output(context, 0).unwrap();
        //let output = context.get_output("outut").unwrap();
        String::from_utf8(output).unwrap()
    }
}

export!(EngineImpl);
