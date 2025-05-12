use cranelift::prelude::*;
use cranelift_jit::{ JITBuilder, JITModule };
use cranelift_module::{ Linkage, Module };

/// Simple AST
enum Expr {

    Num(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),

}

fn make_ast() -> Expr {

    Expr::Add(
        Box::new(Expr::Num(2)),
        Box::new(Expr::Mul(Box::new(Expr::Num(3)), Box::new(Expr::Num(4)))),
    )

}

fn codegen_expr<'a>(
    expr: &Expr,
    builder: &mut FunctionBuilder<'a>,
    ctx: &mut FunctionBuilderContext,
    module: &mut JITModule,
) -> Value
{

    let int_type = types::I64;

    match expr {
        Expr::Num(n) => builder.ins().iconst(int_type, *n),
        Expr::Add(lhs, rhs) => {
            let l = codegen_expr(lhs, builder, ctx, module);
            let r = codegen_expr(rhs, builder, ctx, module);
            builder.ins().iadd(l, r)
        }
        Expr::Mul(lhs, rhs) => {
            let l = codegen_expr(lhs, builder, ctx, module);
            let r = codegen_expr(rhs, builder, ctx, module);
            builder.ins().imul(l, r)
        }
    }
}

#[test]
pub fn jit_testing() {

    // Set up JIT module
    let builder = JITBuilder::new(cranelift_module::default_libcall_names());
    let mut module = JITModule::new(builder.unwrap());

    // Create a function context and signature
    let mut ctx = module.make_context();
    let mut func_ctx = FunctionBuilderContext::new();
    ctx.func.signature = module.make_signature(); // define signature if needed

    // Now build the IR
    {
        let mut builder = FunctionBuilder::new(&mut ctx.func, &mut func_ctx);

        let entry_block = builder.create_block();
        builder.append_block_params_for_function_params(entry_block);
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);

        // Just return 42 as an example
        let forty_two = builder.ins().iconst(types::I64, 42);
        builder.ins().return_(&[forty_two]);

        builder.finalize(); // Don't forget this
    }

    // Finalize and compile the function
    let func_id = module
        .declare_function("main", cranelift_module::Linkage::Export, &ctx.func.signature)
        .unwrap();

    module.define_function(func_id, &mut ctx).unwrap();
    module.clear_context(&mut ctx);
    module.finalize_definitions();

    // Get pointer to compiled code and call it
    let code_ptr = module.get_finalized_function(func_id);
    let func = unsafe { std::mem::transmute::<_, fn() -> i64>(code_ptr) };

    println!("JIT ran and returned: {}", func());

}
