use binaryen::ffi::{
    BinaryenAddDataSegment, BinaryenAddFunction, BinaryenConst, BinaryenExpression,
    BinaryenExpressionPrint, BinaryenFunctionGetBody, BinaryenFunctionGetName,
    BinaryenFunctionHasLocalName, BinaryenGetFunction, BinaryenIndex, BinaryenLiteralInt32,
    BinaryenModuleCreate, BinaryenModuleDispose, BinaryenModulePrint, BinaryenModuleRead,
    BinaryenModuleRef, BinaryenModuleWrite, BinaryenSetMemory, BinaryenTypeCreate,
    BinaryenTypeInt32,
};
use clap::Parser;
use std::ffi::CStr;
use std::ffi::{c_void, CString};
use std::fs::File;
use std::io::{Read, Write};
use std::os::raw::c_char;
use std::ptr;

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

fn read_module(filename: &str) -> BinaryenModuleRef {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = Vec::new();
    f.read_to_end(&mut contents)
        .expect("something went wrong reading the file");

    unsafe { BinaryenModuleRead(contents.as_ptr() as *mut i8, contents.len()) }
}

fn main() {
    unsafe {
        let args = Args::parse();
        let config_wasm = read_module(&args.config_module_path);
        let get_module_path = CString::new("inf:wasi/config#get-model-path").unwrap();
        let get_module_path_func = BinaryenGetFunction(config_wasm, get_module_path.as_ptr());
        println!("get_module_path_func: {:?}", get_module_path_func);
        println!(
            "get_module_path_func: has local name: {:?}",
            BinaryenFunctionHasLocalName(get_module_path_func, 0)
        );
        println!(
            "get_module_path_func: name: {:?}",
            CStr::from_ptr(BinaryenFunctionGetName(get_module_path_func))
                .to_string_lossy()
                .into_owned()
        );

        let body = BinaryenFunctionGetBody(get_module_path_func);
        BinaryenExpressionPrint(body);

        let module_ref = BinaryenModuleCreate();

        let mem_name = CString::new("mem").unwrap();
        let name = CString::new("0").unwrap();
        let segment_names = vec![
            CString::new("inf-config").unwrap(),
            CString::new("1").unwrap(),
        ];
        let segment_names_ptrs: Vec<*const c_char> =
            segment_names.iter().map(|name| name.as_ptr()).collect();

        let segment_data = CString::new("hello, world").unwrap();
        let segment_size: BinaryenIndex = segment_data.to_bytes_with_nul().len() as BinaryenIndex;
        let segment_datas = vec![segment_data, CString::new("I am passive").unwrap()];
        let segment_datas_ptrs: Vec<*const c_char> =
            segment_datas.iter().map(|name| name.as_ptr()).collect();

        let first_seg_off = BinaryenConst(module_ref, BinaryenLiteralInt32(10));
        let first_seg_off_mut: *mut BinaryenExpression = first_seg_off;

        // passive segments don't have offsets as they are not used to initialize memory
        // but instead used with operations like memory.init and memory.copy
        // which will specify the offsets to the destination.
        //let segment_offsets: Vec<*const c_void> = vec![first_seg_off, ptr::null()];
        let mut segment_offsets: Vec<*mut BinaryenExpression> =
            vec![first_seg_off_mut, ptr::null_mut()];

        let segment_sizes: Vec<BinaryenIndex> = vec![segment_size, 0];

        let mem = BinaryenSetMemory(
            module_ref,
            1,                                     // initial memory (pages?)
            256,                                   // maximum memory (pages?)
            mem_name.as_ptr(),                     // export name
            segment_names_ptrs.as_ptr() as *mut _, // segments names
            segment_datas_ptrs.as_ptr() as *mut _, // segment datas
            ptr::null_mut(),                       // segmentPassive which are segments that are not
            segment_offsets.as_mut_ptr(),          // segmentOffset
            segment_sizes.as_ptr() as *mut _,      // segment sizes
            0,
            false,         // shared
            false,         // memory64 (interesting!)
            name.as_ptr(), // name
        );

        let params = BinaryenTypeCreate(ptr::null_mut(), 0);
        let results = BinaryenTypeInt32();

        let memory_offset = BinaryenConst(module_ref, BinaryenLiteralInt32(10));
        let const18 = BinaryenConst(module_ref, BinaryenLiteralInt32(18));
        let function_name = CString::new("added_function").expect("CString::new failed");

        let _ = BinaryenAddFunction(
            module_ref,
            function_name.as_ptr(),
            params,
            results,
            ptr::null_mut(),
            0,
            memory_offset,
        );

        BinaryenModulePrint(module_ref);

        let output_file = "sample.wasm";
        let mut buffer: Vec<u8> = Vec::new();
        buffer.resize(1024 * 1024, 0);

        let written_size =
            BinaryenModuleWrite(module_ref, buffer.as_mut_ptr() as *mut i8, buffer.len());
        buffer.truncate(written_size);
        let mut file = File::create(output_file).expect("Unable to create file");
        file.write(&buffer).expect("Unable to write data to file");
        println!("Wrote {} bytes to {}", written_size, output_file);

        println!("We should now be able to run wasm2wat {}", output_file);

        BinaryenModuleDispose(module_ref);
    }
}
