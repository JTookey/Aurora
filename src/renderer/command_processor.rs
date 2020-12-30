use super::{
    LineInstance,
    TwoDInstance,
    ThreeDInstance,
    InternalCommands,
    RenderCommand,
    CommandManager,
    TextureManager,
    Renderer,
};

pub struct CommandProcessor<'frame> {
    command_manager: &'frame mut CommandManager,
    texture_manager: &'frame mut TextureManager,
}

impl <'frame> CommandProcessor<'frame> {
    pub fn create(
        command_manager: &'frame mut CommandManager,
        texture_manager: &'frame mut TextureManager,
    ) -> Self {
        Self {
            command_manager,
            texture_manager,
        }
    }

    pub fn process_cmd(&mut self, new_cmd: RenderCommand) {
        match new_cmd {
            RenderCommand::Clear(colour) => {
                self.command_manager.push_command(InternalCommands::Clear{
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
                let line_index = self.command_manager.push_line_instance(new_line_instance);

                // Check if can be batched with last command
                if let Some(InternalCommands::DrawLinesBatch{line_instance_end, ..}) = self.command_manager.last_mut() {
                    *line_instance_end += 1;

                // If not then create a new batch command
                } else {
                    let new_line_batch = InternalCommands::DrawLinesBatch {
                        line_instance_start: line_index,
                        line_instance_end: line_index + 1,
                    };

                    self.command_manager.push_command(new_line_batch);
                }
            },

            RenderCommand::Draw2D(desc) => {
                // Create an instance
                let instance = TwoDInstance {
                    position: [desc.position.x, desc.position.y],
                    size: [desc.size.x, desc.size.y],
                    colour: [
                        desc.colour.r as f32, 
                        desc.colour.g as f32, 
                        desc.colour.b as f32, 
                        desc.colour.a as f32
                        ],
                    texture: [0.0,0.0,0.0,0.0],
                    texture_opacity: desc.texture_opacity,
                    line_width: desc.line_width,   
                    corner_radius: desc.corner_radius,
                    rotation: desc.rotation,     
                    shape: 1,    
                };

                // Push new instance
                let two_d_index = self.command_manager.push_two_d_instance(instance);

                let mut new_cmd_needed = false;

                // Check if can be batched with last command
                if let Some(InternalCommands::DrawTwoDBatch{instance_end, texture, ..}) = self.command_manager.last_mut() {
                    if *texture == desc.texture {
                        *instance_end += 1;
                    } else {
                        new_cmd_needed = true;
                    }
                } else {
                    new_cmd_needed = true;
                }

                // If not then create a new batch command
                if new_cmd_needed {
                    let new_2d_batch = InternalCommands::DrawTwoDBatch {
                        instance_start: two_d_index,
                        instance_end: two_d_index + 1,
                        texture: desc.texture,
                    };

                    self.command_manager.push_command(new_2d_batch);
                }
            },
        }
    }
}

impl Renderer for CommandProcessor<'_> {
    fn add(&mut self, cmd: RenderCommand) {
        self.process_cmd(cmd);
    }
}