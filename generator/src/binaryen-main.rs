use binaryen::ffi::BinaryenLiteralInt32;
use clap::Parser;
use std::ffi::CString;
use std::fs::File;
use std::io::{Read, Write};

#[derive(clap::Parser, Debug)]
#[command(
    author ,
    about = "inf-wasi component generator",
    long_about = None
)]
pub struct Args {
    #[arg(short = 'c', long = "config-module-path", value_name = "String")]
    pub(crate) config_module_path: String,
}

fn read_module(filename: &str) -> binaryen::ffi::BinaryenModuleRef {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = Vec::new();
    f.read_to_end(&mut contents)
        .expect("something went wrong reading the file");

    let raw = unsafe {
        //binaryen::ffi::BinaryenModuleSafeRead(contents.as_ptr() as *const c_char, contents.len())
        binaryen::ffi::BinaryenModuleCreate()
    };
    raw
}

fn main() {
    let args = Args::parse();
    let module_ref = read_module(&args.config_module_path);

    let params = unsafe { binaryen::ffi::BinaryenTypeCreate(std::ptr::null_mut(), 0) };
    let results = unsafe { binaryen::ffi::BinaryenTypeInt32() };

    let const18 = unsafe { binaryen::ffi::BinaryenConst(module_ref, BinaryenLiteralInt32(18)) };
    let function_name = CString::new("added_function").expect("CString::new failed");

    let _ = unsafe {
        binaryen::ffi::BinaryenAddFunction(
            module_ref,
            function_name.as_ptr(),
            params,
            results,
            std::ptr::null_mut(),
            0,
            const18,
        )
    };

    unsafe { binaryen::ffi::BinaryenModulePrint(module_ref) };

    let output_file = "sample.wasm";
    let mut buffer: Vec<u8> = Vec::new();
    buffer.resize(1024 * 1024, 0);

    let written_size = unsafe {
        binaryen::ffi::BinaryenModuleWrite(module_ref, buffer.as_mut_ptr() as *mut i8, buffer.len())
    };

    buffer.truncate(written_size);
    let mut file = File::create(output_file).expect("Unable to create file");
    file.write(&buffer).expect("Unable to write data to file");
    println!("Wrote {} bytes to {}", written_size, output_file);
    println!("We should now be able to run wasm2wat {}", output_file);

    unsafe { binaryen::ffi::BinaryenModuleDispose(module_ref) };
}
