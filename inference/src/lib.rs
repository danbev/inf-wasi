use crate::exports::inf::wasi::inference::Guest;
use crate::inf::wasi::config;
use crate::inf::wasi::engine;

wit_bindgen::generate!({
    path: "../wit",
    world: "inference-world",
    exports: {
        world: Export,
         "inf:wasi/inference": Export,
    },
});

struct Export;

impl Guest for Export {
    fn compute() -> String {
        println!("Running inference");
        let config = config::get_config();
        let model_path = config.model_path;
        println!("Model path: {:?}", model_path);

        let result = engine::inference();
        println!("Result: {:?}", result);
        "testing...".to_string()
    }
}
