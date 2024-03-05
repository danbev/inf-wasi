use crate::inf::wasi::config;
use crate::inf::wasi::engine;

wit_bindgen::generate!({
    path: "../wit/inf.wit",
    world: "inference-world",
    exports: {
        world: Export
    },
});

struct Export;

impl Guest for Export {
    fn compute() -> String {
        println!("Running inference");
        let config = config::get_config();
        let engine = engine::Engine::new(&config);
        let result = engine.inference(&config.prompt);
        println!("Result: {:?}", result);
        result
    }
}
