use clap::{App, Arg};
use rust_bindings::WasmtimeInference;
use std::path::PathBuf;

fn main() -> wasmtime::Result<()> {
    let matches = App::new("Inference CLI")
        .arg(
            Arg::with_name("component_path")
                .long("component-path")
                .value_name("COMPONENT_PATH")
                .help("Sets the component wasm file path")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("model_dir")
                .long("model-dir")
                .value_name("MODEL_DIR")
                .help("Sets the model directory path")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let component_path = PathBuf::from(matches.value_of("component_path").unwrap());
    let model_dir = PathBuf::from(matches.value_of("model_dir").unwrap());
    let inference = WasmtimeInference::new(component_path, model_dir)?;
    let result = inference.compute(None);
    println!("Result: {}", result);
    Ok(())
}
