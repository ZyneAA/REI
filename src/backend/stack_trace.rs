#[derive(Clone)]
pub struct CallFrame {
    pub function_name: String,
    pub location: String
}

pub struct ExecContext {
    pub call_stack: Vec<CallFrame>,
}
