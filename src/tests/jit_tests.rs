use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{Linkage, Module};

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
) -> Value {
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

    let builder = JITBuilder::new(cranelift_module::default_libcall_names());
    let mut module = JITModule::new(builder.unwrap());
    let mut ctx = module.make_context();
    let mut func_ctx = FunctionBuilderContext::new();

    let int_type = types::I64;
    let sig = module.make_signature();
    ctx.func.signature = sig;
    let mut builder = FunctionBuilder::new(&mut ctx.func, &mut func_ctx);
    let block = builder.create_block();
    builder.append_block_params_for_function_params(block);
    builder.switch_to_block(block);
    builder.seal_block(block);

    let ast = make_ast();
    let val = codegen_expr(&ast, &mut builder, &mut func_ctx, &mut module);

    builder.ins().return_(&[val]);
    builder.finalize();

    let id = module
        .declare_function("main", Linkage::Export, &ctx.func.signature)
        .unwrap();
    module.define_function(id, &mut ctx).unwrap();
    module.clear_context(&mut ctx);
    module.finalize_definitions();

    let code_ptr = module.get_finalized_function(id);

    let func = unsafe { std::mem::transmute::<_, fn() -> i64>(code_ptr) };
    let result = func();

    println!("JIT result: {}", result);

}
