use serde_json::json;
use std::env;
use wasi_nn::{self, GraphExecutionContext};

fn main() {
    let args: Vec<String> = env::args().collect();
    let model_name: &str = &args[1];

    let model_options = json!({
        "stream-stdout": true,
        "enable-log": true,
        "ctx-size": 1024,
        "n-predict": 512,
        "n-gpu-layers": 33
    });
    println!("Model options: {:?}", model_options);

    let graph =
        wasi_nn::GraphBuilder::new(wasi_nn::GraphEncoding::Ggml, wasi_nn::ExecutionTarget::GPU)
            .build_from_cache(model_name)
            .expect("Failed to build graph from cache");
    let mut context: GraphExecutionContext = graph.init_execution_context().unwrap();
    println!("context: {}", context);

    // Set options to input with index 1
    let options = json!({
        "stream-stdout": true,
        "enable-log": true,
        "ctx-size": 1024,
        "n-predict": 512,
        "n-gpu-layers": 33
    });
    println!("Options: {}", options);

    context
        .set_input(
            1,
            wasi_nn::TensorType::U8,
            &[1],
            &options.to_string().as_bytes().to_vec(),
        )
        .unwrap();

    let prompt = &args[2];
    println!("Prompt: {}", prompt);
    let tensor_data = prompt.as_bytes().to_vec();
    context
        .set_input(0, wasi_nn::TensorType::U8, &[1], &tensor_data)
        .unwrap();

    println!("Response:");
    context.compute().unwrap();
    let max_output_size = 4096 * 6;
    let mut output_buffer = vec![0u8; max_output_size];
    let mut output_size = context.get_output(0, &mut output_buffer).unwrap();
    output_size = std::cmp::min(max_output_size, output_size);
    let output = String::from_utf8_lossy(&output_buffer[..output_size]).to_string();
    println!("{}", output.trim());
}
