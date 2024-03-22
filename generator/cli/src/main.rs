use clap::Parser;
use generator::{self, BuildType, GenConfig};
use std::path::PathBuf;

#[derive(clap::Parser, Debug)]
#[command(
    author ,
    about = "inf-wasi component generator",
    long_about = None
)]
pub struct Args {
    #[arg(short = 'n', long = "name", value_name = "String")]
    pub(crate) name: String,

    #[arg(long = "model-path", value_name = "FILE")]
    pub(crate) model_path: PathBuf,

    #[arg(short = 'p', long = "prompt", value_name = "String")]
    pub(crate) prompt: String,

    #[arg(
        short = 'w',
        long = "work-dir",
        value_name = "DIR",
        default_value = "working"
    )]
    pub(crate) work_dir: PathBuf,

    #[arg(
        short = 'm',
        long = "modules-dir",
        value_name = "DIR",
        default_value = "modules"
    )]
    pub(crate) modules_dir: PathBuf,

    #[arg(
        short = 'o',
        long = "output-dir",
        value_name = "DIR",
        default_value = "working/target"
    )]
    pub(crate) output_dir: PathBuf,
}

fn main() {
    let args = Args::parse();
    let config_name = args.name;
    let prompt = args.prompt;
    let model_path = args.model_path;

    let config = GenConfig {
        name: config_name,
        model_path,
        prompt,
        work_dir: args.work_dir,
        build_type: BuildType::Debug,
        modules_dir: args.modules_dir,
        output_dir: args.output_dir,
    };
    match generator::generate(&config) {
        Ok(composed_path) => println!("Composed into webassembly component:\n{composed_path:?}"),
        Err(e) => eprintln!("Error while generating: {}", e),
    }
}
