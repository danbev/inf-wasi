use crate::exports::inf::wasi::config::Guest;
use crate::exports::inf::wasi::config_types::Config;
use std::path::PathBuf;

wit_bindgen::generate!({
    path: "../wit/inf.wit",
    world: "config-world",
    exports: {
        "inf:wasi/config": Exports
    },
});

struct Exports;

impl Guest for Exports {
    fn get_config() -> Config {
        let model_path = PathBuf::from("models/llama-2-7b-chat.Q5_K_M.gguf")
            .to_str()
            .unwrap()
            .to_string();
        Config {
            model_path,
            prompt: "What is LoRA?".to_string(),
        }
    }
}
