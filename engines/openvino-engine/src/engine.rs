use crate::engine::exports::inf::wasi::engine::Guest;
use crate::engine::exports::inf::wasi::engine::GuestEngine;
use crate::engine::inf::wasi::config_types::Config;
use std::path::PathBuf;
use wasi_nn::graph;
use wasi_nn::tensor;

wit_bindgen::generate!({
    path: "../../wit",
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

        let model: graph::GraphBuilder = model_path.to_str().unwrap().as_bytes().to_vec();
        let weights: graph::GraphBuilder = model_path.to_str().unwrap().as_bytes().to_vec();
        let builders = vec![model, weights];

        let graph = graph::load(
            &builders,
            graph::GraphEncoding::Openvino,
            graph::ExecutionTarget::Cpu,
        )
        .unwrap();

        let context = wasi_nn::inference::init_execution_context(graph).unwrap();

        println!("Engine model_path: {}", &self.model_path);
        println!("Engine prompt: {}", &self.prompt);

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
