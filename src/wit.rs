use serde_json::json;
use wasi_nn::graph;
use wasi_nn::inference;
use wasi_nn::tensor;

wit_bindgen::generate!({
    path: "wit/inf.wit",
    world: "inf",
    exports: {
        world: Exports,
    },
});

use std::path::PathBuf;

struct Exports;

impl Guest for Exports {
    fn version() -> String {
        crate::version().to_string()
    }

    fn inference() -> String {
        // TODO: Figure out how to create a component adapter for wasi-nn
        let model_name = "llama-chat";
        let _model_options = json!({
            "stream-stdout": true,
            "enable-log": true,
            "ctx-size": 1024,
            "n-predict": 512,
            "n-gpu-layers": 33
        });

        let model_path = PathBuf::from("../models/llama-2-7b-chat.Q5_K_M.gguf");
        let graph_builder: graph::GraphBuilder = model_path.to_str().unwrap().as_bytes().to_vec();
        let builders = vec![graph_builder];

        let graph = graph::load(
            &builders,
            graph::GraphEncoding::Ggml,
            graph::ExecutionTarget::Cpu,
        )
        .unwrap();

        let context: inference::GraphExecutionContext =
            inference::init_execution_context(graph).unwrap();

        println!("Context: {}", context);

        let options = json!({
            "stream-stdout": true,
            "enable-log": true,
            "ctx-size": 1024,
            "n-predict": 512,
            "n-gpu-layers": 25
        });
        println!("Options: {}", options);

        let options_tensor = tensor::Tensor {
            dimensions: vec![1_u32],
            tensor_type: tensor::TensorType::U8,
            data: options.to_string().as_bytes().to_vec(),
        };

        inference::set_input(context, 1, &options_tensor).unwrap();

        let prompt = "What is LoRA?";
        let prompt_tensor = tensor::Tensor {
            dimensions: vec![1_u32],
            tensor_type: tensor::TensorType::U8,
            data: prompt.as_bytes().to_vec(),
        };
        inference::set_input(context, 2, &prompt_tensor).unwrap();

        inference::compute(context).unwrap();
        let output = inference::get_output(context, 3).unwrap();

        /*
        context
            .set_input(
                1,
                wasi_nn::tensor::TensorType::U8,
                &[1],
                &options.to_string().as_bytes().to_vec(),
            )
            .unwrap();
        */

        /*
        let graph = GraphBuilder::new(GraphEncoding::Ggml, ExecutionTarget::GPU)
            //.config(model_options.to_string())
            .build_from_cache(model_name)
            .expect("Failed to build graph from cache");
        */
        //let mut context: GraphExecutionContext = graph.init_execution_context().unwrap();
        //println!("context: {}", context);
        "inference result".to_string()
    }
}
