use crate::Colour;

use super::{Renderer, RenderCommand, LineInstance, TwoDInstance, ThreeDInstance};

// These are the internally stored commands that allow us batch renderpasses together
pub enum InternalCommands {
    Clear {
        colour: Colour,
    },
    DrawLinesBatch{
        line_instance_start: usize,
        line_instance_end: usize,
    },
    DrawTwoDBatch{
        instance_start: usize,
        instance_end: usize,
    },
    None,
}

pub struct CommandManager {
    command_list: Vec<InternalCommands>,
    
    line_instances: Vec<LineInstance>,
    two_d_instance: Vec<TwoDInstance>,
    three_d_instance: Vec<ThreeDInstance>,
}

impl CommandManager {
    pub fn new() -> Self {
        Self { 
            command_list: Vec::new(),
            
            line_instances: Vec::with_capacity(super::MAX_INSTANCES),
            two_d_instance: Vec::with_capacity(super::MAX_INSTANCES),
            three_d_instance: Vec::with_capacity(super::MAX_INSTANCES),
         }
    }

    pub fn commands(&self) -> &Vec<InternalCommands> {
        &self.command_list
    }

    pub fn process_cmd(&mut self, new_cmd: RenderCommand) {
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
                let line_index = self.push_line_instance(new_line_instance);

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
                    shape: desc.shape as u32,    
                };

                // Push new instance
                let two_d_index = self.push_two_d_instance(instance);

                // Check if can be batched with last command
                if let Some(InternalCommands::DrawTwoDBatch{instance_end, ..}) = self.command_list.last_mut() {
                    *instance_end += 1;

                // If not then create a new batch command
                } else {
                    let new_2d_batch = InternalCommands::DrawTwoDBatch {
                        instance_start: two_d_index,
                        instance_end: two_d_index + 1,
                    };

                    self.command_list.push(new_2d_batch);
                }
            },
        }
    }

    // Line Instance functions
    pub fn push_line_instance(&mut self, line: LineInstance) -> usize {
        self.line_instances.push(line);
        self.line_instances.len() - 1
    }

    pub fn n_line_instances(&self) -> usize {
        self.line_instances.len()
    }

    pub fn get_line_instances(&self, start_id: usize, end_id: usize) -> &[LineInstance] {
        &self.line_instances[start_id..end_id]
    }


    // Primative Instance fucntions
    pub fn push_two_d_instance(&mut self, instance: TwoDInstance) -> usize {
        self.two_d_instance.push(instance);
        self.two_d_instance.len() - 1
    }

    pub fn n_two_d_instance(&self) -> usize {
        self.two_d_instance.len()
    }

    pub fn get_two_d_instance(&self, start_id: usize, end_id: usize) -> &[TwoDInstance] {
        &self.two_d_instance[start_id..end_id]
    }


    // Geometry Instance Functions
    pub fn push_geometry_instance(&mut self, instance: ThreeDInstance) -> usize {
        self.three_d_instance.push(instance);
        self.three_d_instance.len() - 1
    }

    pub fn n_three_d_instance(&self) -> usize {
        self.three_d_instance.len()
    }

    pub fn clear(&mut self) {
        self.command_list.clear();

        self.line_instances.clear();
        self.two_d_instance.clear();
        self.three_d_instance.clear();
    }
}

impl Renderer for CommandManager {
    fn add(&mut self, cmd: RenderCommand) {
        self.process_cmd(cmd);
    }
}