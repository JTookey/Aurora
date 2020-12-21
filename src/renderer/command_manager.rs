use crate::Colour;

use super::{InstanceManager, RenderCommand, LineInstance};

// These are the internally stored commands that allow us batch renderpasses together
pub enum InternalCommands {
    Clear {
        colour: Colour,
    },
    DrawLinesBatch{
        line_instance_start: usize,
        line_instance_end: usize,
    },
    None,
}

pub struct CommandManager {
    command_list: Vec<InternalCommands>,
}

impl CommandManager {
    pub fn new() -> Self {
        Self { command_list: Vec::new() }
    }

    pub fn commands(&self) -> &Vec<InternalCommands> {
        &self.command_list
    }

    pub fn process_cmd(&mut self, new_cmd: RenderCommand, im: &mut InstanceManager) {
        match new_cmd {
            RenderCommand::Clear(colour) => {
                self.command_list.push(InternalCommands::Clear{
                    colour,
                });
            },
            RenderCommand::DrawLine(line_desc) => {
                // Create Line Instance
                let new_line_instance = LineInstance {
                    position_1: [line_desc.start.x, line_desc.start.y],
                    position_2: [line_desc.end.x, line_desc.end.y],
                    line_colour: [
                        line_desc.colour.r as f32, 
                        line_desc.colour.g as f32, 
                        line_desc.colour.b as f32, 
                        line_desc.colour.a as f32
                        ],
                    line_width: line_desc.width,
                };

                // Push new instance
                let line_index = im.push_line_instance(new_line_instance);

                // Check if can be batched with last command
                if let Some(InternalCommands::DrawLinesBatch{line_instance_end, ..}) = self.command_list.last_mut() {
                    *line_instance_end += 1;

                // If not then create a new batch command
                } else {
                    let new_line_batch = InternalCommands::DrawLinesBatch {
                        line_instance_start: line_index,
                        line_instance_end: line_index + 1,
                    };

                    self.command_list.push(new_line_batch);
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.command_list.clear();
    }
}