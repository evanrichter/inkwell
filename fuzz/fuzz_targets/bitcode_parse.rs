#![no_main]
use libfuzzer_sys::fuzz_target;

use inkwell::context::Context;
use inkwell::memory_buffer::MemoryBuffer;
use inkwell::module::Module;

fuzz_target!(|data: &[u8]| {
    let buf = MemoryBuffer::create_from_memory_range_copy(data, "fuzz-data");
    let ctx = Context::create();
    let module = match Module::parse_bitcode_from_buffer(&buf, &ctx) {
        Ok(m) => m,
        Err(_) => return,
    };

    let _ = module.get_data_layout();
    let _ = module.get_first_function();
    let _ = module.get_last_function();
    let _ = module.get_function("function");
    let _ = module.get_struct_type("struct");
    let _ = module.get_triple();
    let _ = module.create_execution_engine();
    let _ = module.get_first_global();
    let _ = module.get_last_global();
    let _ = module.get_global("global");
    let _ = module.get_name();
    let _ = module.get_or_insert_comdat("comdat");
    let _ = module.get_flag("flag");
    let _ = module.get_debug_metadata_version();
    let _ = module.strip_debug_info();
    let _ = module.get_debug_metadata_version();
});
