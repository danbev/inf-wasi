use serde_json::json;
use wasi_nn::{self, GraphExecutionContext};

wit_bindgen::generate!({
    world: "llm",
    exports: {
        world: Exports,
    },
});

struct Exports;

impl Guest for Exports {
    fn version() -> String {
        crate::version().to_string()
    }

    fn inference() -> String {
        // TODO: Figure out how to create a component adapter for wasi-nn
        /*
        let model_name = "llama-chat";
        let model_options = json!({
            "stream-stdout": true,
            "enable-log": true,
            "ctx-size": 1024,
            "n-predict": 512,
            "n-gpu-layers": 33
        });
        let graph =
            wasi_nn::GraphBuilder::new(wasi_nn::GraphEncoding::Ggml, wasi_nn::ExecutionTarget::GPU)
                //.config(model_options.to_string())
                .build_from_cache(model_name)
                .expect("Failed to build graph from cache");
        */
        //let mut context: GraphExecutionContext = graph.init_execution_context().unwrap();
        //println!("context: {}", context);
        "inference result".to_string()
    }
}
