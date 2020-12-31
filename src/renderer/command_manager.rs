use crate::{Colour};

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
        texture: Option<usize>,
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

    pub fn push_command(&mut self, new_cmd: InternalCommands) {
        self.command_list.push(new_cmd);
    }

    pub fn last(&self) -> Option<&InternalCommands> {
        self.command_list.last()
    }

    pub fn last_mut(&mut self) -> Option<&mut InternalCommands> {
        self.command_list.last_mut()
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

    pub fn get_two_d_instances(&self, start_id: usize, end_id: usize) -> &[TwoDInstance] {
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

