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
    pub fn execute_plan(plan: &Plan, with_terminal_output: bool) -> Vec<i8> {

    }
}