use crate::crux::util;

#[derive(Clone, Debug)]
pub struct CallFrame {
    pub function_name: String,
    pub location: String,
}

impl CallFrame {
    pub fn new(function_name: String, location: String) -> Self {
        CallFrame {
            function_name,
            location,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ExecContext {
    pub call_stack: Vec<CallFrame>,
}

impl ExecContext {
    pub fn new() -> Self {
        let call_stack = vec![];
        ExecContext { call_stack }
    }

    pub fn format_stack_trace(&self) -> String {
        let mut report = String::new();

        report.push_str("Stack trace -->");
        if self.call_stack.is_empty() {
            report.push_str("\n\t[exec_ctx empty]");
        }
        for call in self.call_stack.iter().rev() {
            let position = &format!("\n\tat {} ({})", &call.function_name, &call.location);
            report.push_str(&position);
        }

        util::red_colored(&report)
    }

    pub fn push_call(&mut self, frame: CallFrame) {
        self.call_stack.push(frame);
    }

    pub fn pop_call(&mut self) {
        if self.call_stack.pop().is_none() {
            eprintln!("[exec_ctx] Warning: tried to pop from empty stack trace 🤡");
        }
    }
}
