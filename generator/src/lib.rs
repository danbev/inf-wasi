use anyhow::Result;
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::{Read, Seek, Write};
use std::path::PathBuf;
use std::process::Command;
use toml_edit::Document;
use wasm_compose::composer::ComponentComposer;
use wasm_compose::config::Config as ComposerConfig;
use wit_component::ComponentEncoder;

macro_rules! workspace_toml {
    ($config_name:expr) => {
        format!(
            "
[workspace]
resolver = \"2\"
members = [
    \"{}\",
]
[workspace.dependencies]
wit-bindgen = {{ version = \"0.19.2\", default-features = true, features = ['macros'] }}
",
            $config_name
        )
    };
}

macro_rules! workspace_member_toml {
    ($config_name:expr) => {
        format!(
            "
[package]
name = \"{}\"
version = \"0.1.0\"
edition = \"2021\"

[dependencies]
wit-bindgen = {{ workspace = true }}

[lib]
crate-type = [\"cdylib\"]
",
            $config_name
        )
    };
}

const SRC_HEADER: &str = r#"
wit_bindgen::generate!({
    inline: "
      package inf:wasi;

      world config-world {
        export config;
        export config-types;
      }

      interface config-types {
        record config {
          model-path: string,
          prompt: string,
        }
      }

      interface config {
        use config-types.{config};
        get-config: func() -> config;
      }
    ",
    world: "config-world",
    exports: {
        "inf:wasi/config": Exports
    },
});
"#;

macro_rules! config_lib {
    ($model_path:expr, $prompt:expr) => {
        format!(
            "{}
use crate::exports::inf::wasi::config::Guest;
use crate::exports::inf::wasi::config_types::Config;

struct Exports;

impl Guest for Exports {{
    fn get_config() -> Config {{
        let model_path = r#\"{}\"#;
        let prompt = r#\"{}\"#;
        Config {{
            model_path: model_path.to_string(),
            prompt: prompt.to_string(),
        }}
    }}
}}",
            SRC_HEADER, $model_path, $prompt
        )
    };
}

#[derive(Debug, Clone, Copy)]
pub enum BuildType {
    Debug,
    Release,
}

impl BuildType {
    fn as_string(&self) -> &'static str {
        match self {
            BuildType::Debug => "debug",
            BuildType::Release => "release",
        }
    }
}

impl From<&str> for BuildType {
    fn from(s: &str) -> Self {
        match s {
            "release" => BuildType::Release,
            _ => BuildType::Debug,
        }
    }
}

impl From<String> for BuildType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "release" => BuildType::Release,
            _ => BuildType::Debug,
        }
    }
}

#[derive(Debug)]
pub struct GenConfig {
    pub name: String,
    pub model_path: PathBuf,
    pub prompt: String,
    pub work_dir: PathBuf,
    pub build_type: BuildType,
    pub modules_dir: PathBuf,
    pub output_dir: PathBuf,
}

impl GenConfig {
    pub fn new(name: &str, model_path: PathBuf, prompt: &str, work_dir: PathBuf) -> Self {
        Self {
            name: name.to_string(),
            model_path,
            prompt: prompt.to_string(),
            build_type: BuildType::Debug,
            work_dir,
            modules_dir: PathBuf::from("modules"),
            output_dir: PathBuf::from("target"),
        }
    }
}

fn create_workspace(working_dir_path: &PathBuf, config_name: &str) -> Result<()> {
    let workspace_toml = format!("{}/Cargo.toml", working_dir_path.display());
    if !working_dir_path.exists() {
        fs::create_dir_all(working_dir_path)?;
        fs::write(workspace_toml, workspace_toml!(config_name))?;
    }
    Ok(())
}

fn add_workspace_member(working_dir_path: &PathBuf, config_name: &str) -> Result<()> {
    let workspace_toml = format!("{}/Cargo.toml", working_dir_path.display());
    let mut cargo_toml = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&workspace_toml)?;
    let mut content = String::new();
    cargo_toml.read_to_string(&mut content)?;

    let mut root_toml = content.parse::<Document>()?;
    let members = root_toml["workspace"]["members"].as_array_mut().unwrap();
    if members
        .iter()
        .find(|i| i.as_str() == Some(config_name))
        .is_some()
    {
        return Ok(());
    }
    members.push(config_name);

    cargo_toml.set_len(0)?;
    cargo_toml.seek(std::io::SeekFrom::Start(0))?;
    cargo_toml.write_all(root_toml.to_string().as_bytes())?;
    Ok(())
}

pub fn generate(config: &GenConfig) -> Result<PathBuf> {
    let config_name = &config.name.clone();
    let working_dir_path = config.work_dir.clone();
    let workspace_exists = working_dir_path.exists();
    if !workspace_exists {
        create_workspace(&working_dir_path, config_name)?;
        println!("Created workspace '{:?}'", working_dir_path);
    }
    add_workspace_member(&working_dir_path, config_name)?;

    let working_dir_path = working_dir_path.canonicalize().unwrap();

    let working_dir = working_dir_path.display();
    let config_src_dir = format!("{}/{}/src", working_dir, &config_name);
    let config_toml = format!("{}/{}/Cargo.toml", working_dir, &config_name);
    let _res = fs::create_dir_all(&config_src_dir);
    let _res = fs::write(&config_toml, workspace_member_toml!(&config_name));
    let _res = fs::write(
        format!("{}/lib.rs", &config_src_dir),
        config_lib!(
            &config.model_path.to_str().unwrap().to_string(),
            &config.prompt
        ),
    );
    if !workspace_exists {
        print!("Building workspace...");
    }
    let _ = std::io::stdout().flush();
    let output = match config.build_type {
        BuildType::Release => Command::new("cargo")
            // TODO: Currently when doing a release build there is an issue with wit.
            .args(&[
                "build",
                "--package",
                config_name,
                "--release",
                "--manifest-path",
                &config_toml,
                "--target",
                "wasm32-unknown-unknown",
            ])
            .output()?,
        BuildType::Debug => Command::new("cargo")
            .args(&[
                "build",
                "--package",
                config_name,
                "--manifest-path",
                &config_toml,
                "--target",
                "wasm32-unknown-unknown",
            ])
            .output()?,
    };
    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "Compilation error: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    if !workspace_exists {
        println!("done");
    }

    // Make a component out for the core webassembly module that we compiled
    // above.
    let config_module_path = format!(
        "{}/target/wasm32-unknown-unknown/{}/{}.wasm",
        working_dir,
        config.build_type.as_string(),
        config_name
    );

    let config_module = fs::read(config_module_path).unwrap();

    let component_encoder = ComponentEncoder::default()
        .module(config_module.as_slice())
        .unwrap();
    let component = component_encoder.encode().unwrap();
    let component_path = PathBuf::from(format!(
        "{}/target/{}-config-component.wasm",
        working_dir, config_name
    ));
    let ret = fs::write(&component_path, component);
    if ret.is_err() {
        return Err(anyhow::anyhow!(
            "Failed to write component to {component_path:?}",
            component_path = component_path
        ));
    }
    println!("Created webassembly component: {component_path:?}");

    let engine_component_path = PathBuf::from(format!(
        "{}/engine-component.wasm",
        config.modules_dir.display()
    ));
    let config_component_path = PathBuf::from(format!(
        "{}/target/{}-config-component.wasm",
        working_dir, config_name
    ));
    let definitions = vec![engine_component_path, config_component_path];

    let composer_config = ComposerConfig {
        search_paths: vec![
            config.modules_dir.clone(),
            PathBuf::from(format!("{}/target", working_dir)),
        ],
        instantiations: indexmap::IndexMap::new(),
        definitions,
        ..Default::default()
    };
    //println!("composer_config: {:?}", composer_config);
    let mut inference_component_path = config.modules_dir.clone();
    inference_component_path.push("inference-component.wasm");
    let composer = ComponentComposer::new(&inference_component_path, &composer_config);
    let composed = composer.compose();

    let mut composed_path = config.output_dir.clone();
    if !composed_path.is_absolute() {
        composed_path = env::current_dir()?.join(composed_path)
    }

    composed_path.push(format!("{}-composed.wasm", config_name));
    let res = fs::write(&composed_path, composed.unwrap());
    if res.is_err() {
        return Err(anyhow::anyhow!(
            "Failed to write composed component: {}",
            composed_path.to_string_lossy()
        ));
    }
    Ok(composed_path.to_path_buf())
}
