use crate::inf::wasi::config;
use crate::inf::wasi::engine;

wit_bindgen::generate!({
    path: "../wit",
    world: "inference-world",
});

struct Export;

impl Guest for Export {
    fn compute(prompt: Option<String>) -> String {
        println!("Inference Component Running inference");
        let config = config::get_config();
        let engine = engine::Engine::new(&config);
        let result = match prompt {
            Some(prompt) => engine.inference(&prompt),
            None => engine.inference(&config.prompt.clone()),
        };
        result
    }
}

export!(Export);
