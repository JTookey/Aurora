pub struct CommandManager {
    frame_commands: Vec<InternalCommands>,
}

impl CommandManager {
    pub fn new() -> Self {
        Self {
            frame_commands: Vec::new(),
        }
    }
}

enum InternalCommands {
    Clear {
        r: f64,
        g: f64,
        b: f64,
        a: f64,
    },
    DrawLinesBatch{
        line_instance_start: usize,
        line_instance_end: usize,
    },
    None,
}
