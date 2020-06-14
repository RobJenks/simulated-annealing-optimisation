const CMD_TERMINATE: i32 = 1;

pub struct Command {
    operation: i32
}

impl Command {
    pub fn terminate() -> Self { Self { operation: CMD_TERMINATE } }

    pub fn get_operation(&self) -> i32 { self.operation }

}