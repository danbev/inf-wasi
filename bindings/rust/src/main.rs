use rust_bindings::Inference;
use std::path::PathBuf;

fn main() -> wasmtime::Result<()> {
    let component_path = PathBuf::from("./target/composed.wasm");
    let model_dir = PathBuf::from("./models");
    let inference = Inference::new(component_path, model_dir)?;
    let result = inference.compute();
    println!("result: {:?}", result);
    Ok(())
}
