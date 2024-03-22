use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};

use clap::Parser;
use generator::{self, BuildType, GenConfig};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone)]
struct GenAppData {
    work_dir: PathBuf,
    modules_dir: PathBuf,
    output_dir: PathBuf,
    build_type: BuildType,
}

impl GenAppData {
    fn new(
        work_dir: PathBuf,
        modules_dir: PathBuf,
        output_dir: PathBuf,
        build_type: BuildType,
    ) -> Self {
        GenAppData {
            work_dir,
            modules_dir,
            output_dir,
            build_type,
        }
    }
}

#[derive(clap::Parser, Debug)]
#[command(
    author ,
    about = "inf-wasi web component generator",
    long_about = None
)]
pub struct Args {
    #[arg(
        short = 'm',
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

    #[arg(
        short = 'b',
        long = "build-type",
        value_name = "BUILD_TYPE",
        default_value = "Release"
    )]
    pub(crate) build_type: String,
}

#[derive(Deserialize, Debug)]
struct InputData {
    config_name: String,
    model_path: String,
    prompt: String,
}

#[post("/generate")]
async fn generate_wasm(
    data: web::Data<GenAppData>,
    config: web::Json<InputData>,
) -> impl Responder {
    let gen_config = GenConfig {
        name: config.config_name.clone(),
        model_path: config.model_path.clone().into(),
        prompt: config.prompt.clone(),
        build_type: data.build_type,
        work_dir: data.work_dir.clone(),
        modules_dir: data.modules_dir.clone(),
        output_dir: data.output_dir.clone(),
    };

    match generator::generate(&gen_config) {
        Ok(composed_path) => match std::fs::read(composed_path) {
            Ok(wasm_binary) => HttpResponse::Ok()
                .content_type("application/wasm")
                .body(wasm_binary),
            Err(_) => HttpResponse::InternalServerError().body("Error reading wasm file"),
        },
        Err(_) => HttpResponse::InternalServerError().body("Error generating wasm file"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let config_app_data = web::Data::new(GenAppData::new(
        args.work_dir,
        args.modules_dir,
        args.output_dir,
        args.build_type.into(),
    ));
    println!("Starting server at http://127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(config_app_data.clone())
            .service(generate_wasm)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
