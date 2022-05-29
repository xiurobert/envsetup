use crate::cli_utils::exec_stream_shell;

struct Plan {
    pub requires_elevator: bool,
    pub commands: Vec<String>
}


impl Plan {
    pub fn new() -> Plan {
        Plan {
            requires_elevator: false,
            commands: Vec::new()
        }
    }

    pub fn preview_plan(&self) {
        for cmd in &self.commands {
            println!( "+ {}", cmd);
        }
    }

    pub fn execute_plan(plan: &Plan) -> Vec<bool> {
        let mut results = Vec::<bool>::new();
        for cmd in &plan.commands {
            let shell_result = exec_stream_shell(cmd);
            results.push(shell_result);
        }
        results
    }
}