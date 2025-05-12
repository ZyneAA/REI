use cranelift::prelude::*;

use crate::crux::token::Object;

pub fn to_clif_type(ty: &Object) -> Option<Type> {

    match ty {
        Object::Number(_) => Some(types::F32),
        Object::Str(_) => Some(types::I64),
        Object::Bool(_) => Some(types::I8),
        Object::Null => None,
    }

}

pub fn create_block_with_seal(builder: &mut FunctionBuilder) -> Block {

    let block = builder.create_block();
    builder.seal_block(block);
    block

}
