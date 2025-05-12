use cranelift::prelude::*;
use cranelift_module::Module;
use cranelift_jit::JITModule;

pub fn emit_print(
    builder: &mut FunctionBuilder,
    module: JITModule,
    value: Value,
    newline: bool
)
{
    let printf_id = module.declare_function(
        "printf",
        Linkage::Import,
        &{
            let mut sig = cranelift::codegen::ir::Signature::new(cranelift::codegen::isa::CallConv::SystemV);
            sig.params.push(AbiParam::new(types::I8X32)); // Format string
            sig.params.push(AbiParam::new(types::I32));   // Value
            sig
        },
    ).unwrap();

    let format_str = if newline { "%d\n\0" } else { "%d\0" };
    let format_ptr = builder.ins().iconst(types::I64, format_str.as_ptr() as i64);
    builder.ins().call(printf_id, &[format_ptr, value]);
}
