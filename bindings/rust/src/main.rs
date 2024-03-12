use clap::{App, Arg};
use rust_bindings::WasmtimeInference;
use std::path::PathBuf;

fn main() -> wasmtime::Result<()> {
    let matches = App::new("Wasmtime llama.cpp inference")
        .arg(
            Arg::with_name("component_path")
                .long("component-path")
                .value_name("COMPONENT_PATH")
                .help("Path to the component wasm")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("model_dir")
                .long("model-dir")
                .value_name("MODEL_DIR")
                .help("The model directory to preopen and make available to the wasm component")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let component_path = PathBuf::from(matches.value_of("component_path").unwrap());
    let model_dir = PathBuf::from(matches.value_of("model_dir").unwrap());

    let inference = WasmtimeInference::new(component_path, model_dir)?;
    let result = inference.run_inference(None);

    println!("Result: {}", result);
    Ok(())
}
