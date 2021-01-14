use crate::Colour;
use super::{CommandManager, SectionManager, PipelineManager, TextureManager, InternalCommands, MAX_INSTANCES};

pub struct CommandExecutor<'ce, 'frame> {
    device: &'ce wgpu::Device,
    queue:  &'ce wgpu::Queue,
    frame: &'ce wgpu::SwapChainFrame,

    command_manager: &'ce CommandManager,
    section_manager: &'ce mut SectionManager<'frame>,
    pipeline_manager: &'ce mut PipelineManager,
    texture_manager: &'ce mut TextureManager,

    clear_colour: Option<&'ce Colour>,

    line_instances_on_gpu: Option<(usize, usize)>,
    two_d_instances_on_gpu: Option<(usize, usize)>,
    _three_d_instances_on_gpu: Option<(usize, usize)>,
}

impl <'ce, 'frame: 'ce> CommandExecutor<'ce, 'frame> {
    pub fn new(
        device: &'ce wgpu::Device,
        queue:  &'ce wgpu::Queue,
        frame: &'ce wgpu::SwapChainFrame,

        command_manager: &'ce CommandManager,
        section_manager: &'ce mut SectionManager<'frame>,
        pipeline_manager: &'ce mut PipelineManager,
        texture_manager: &'ce mut TextureManager,
    ) -> Self {
        Self {
            device,
            queue,
            frame,

            command_manager,
            section_manager,
            pipeline_manager,
            texture_manager,

            clear_colour: None,

            line_instances_on_gpu: None,
            two_d_instances_on_gpu: None,
            _three_d_instances_on_gpu: None,
        }   
    }

    pub fn build_frame(&mut self) {
        // First check if buffers need preparing
        self.pipeline_manager.prepare_buffers(
            self.device, 
            self.texture_manager.buffer_dimensions_required(),
        );

        // Check if Textures need loading
        if self.texture_manager.needs_preparing() {
            self.texture_manager.prepare(self.device, self.queue);
        }

        // Loop through commands
        for cmd in self.command_manager.commands() {

            // Change the load opp depending on if a clear colour has been set
            let load_op = if let Some(c) = self.clear_colour.take() {
                wgpu::LoadOp::Clear(c.clone())
            } else {
                wgpu::LoadOp::Load
            };

            // Match the commands
            match cmd {
                InternalCommands::Clear{colour} => {
                    self.clear_colour = Some(colour);
                },

                InternalCommands::DrawLinesBatch{line_instance_start, line_instance_end} => {
                    // Create local variables
                    let mut load_start_id = *line_instance_start;
                    let load_end_id = *line_instance_end;
                    
                    // Count the number of instances that require rendereing
                    let mut n_instances_remaining = line_instance_end - line_instance_start;

                    // Not all might fit in a single render pass due to limits on buffer size and therefore
                    // number of instances that can be rendered... so we will loop.
                    while n_instances_remaining > 0 {
                        if let Some((start_id, end_id)) = instances_to_load(
                            (load_start_id, load_end_id), 
                            self.line_instances_on_gpu, 
                            self.command_manager.n_line_instances()
                        ) {

                            // Write instances to the GPU
                            self.pipeline_manager.update_line_instances(
                                self.queue, 
                                self.command_manager.get_line_instances(load_start_id, load_end_id));

                            // Count how many - will end the loop if n_instances_remaining reaches zero
                            n_instances_remaining -= end_id - start_id;
                            
                            // Prep for next loop if required
                            if n_instances_remaining != 0 {
                                load_start_id = end_id + 1;
                            }

                            // Update the records
                            self.line_instances_on_gpu = Some((start_id, end_id));

                        } else {
                            // Everything required already on GPU - cancel the loop
                            n_instances_remaining = 0;
                        }

                        // Render instances
                        self.pipeline_manager.render_lines(
                            self.device, 
                            self.queue, 
                            self.frame,
                            *line_instance_start as u32, 
                            *line_instance_end as u32, 
                            load_op,
                        );
                    }
                },

                InternalCommands::DrawTwoDBatch{instance_start, instance_end, texture} => {
                    // Create local variables
                    let mut load_start_id = *instance_start;
                    let load_end_id = *instance_end;
                    
                    // Count the number of instances that require rendereing
                    let mut n_instances_remaining = instance_end - instance_start;

                    // Get the texture
                    let texture_for_instances = if let Some(texture_handle) = texture {
                        self.texture_manager.get_texture(texture_handle)
                    } else {
                        None
                    };

                    // Not all might fit in a single render pass due to limits on buffer size and therefore
                    // number of instances that can be rendered... so we will loop.
                    while n_instances_remaining > 0 {
                        if let Some((start_id, end_id)) = instances_to_load(
                            (load_start_id, load_end_id), 
                            self.two_d_instances_on_gpu, 
                            self.command_manager.n_two_d_instance()
                        ) {

                            // Write instances to the GPU
                            self.pipeline_manager.update_two_d_instances(
                                self.queue, 
                                self.command_manager.get_two_d_instances(load_start_id, load_end_id));

                            // Count how many - will end the loop if n_instances_remaining reaches zero
                            n_instances_remaining -= end_id - start_id;
                            
                            // Prep for next loop if required
                            if n_instances_remaining != 0 {
                                load_start_id = end_id + 1;
                            }

                            // Update the records
                            self.two_d_instances_on_gpu = Some((start_id, end_id));

                        } else {
                            // Everything required already on GPU - cancel the loop
                            n_instances_remaining = 0;
                        }

                        // Render instances
                        self.pipeline_manager.render_2d(
                            self.device, 
                            self.queue, 
                            self.frame,
                            *instance_start as u32, 
                            *instance_end as u32,
                            texture_for_instances,
                            load_op,
                        );
                    }
                },

                InternalCommands::DrawTextBatch{section_start, section_end} => {
                    self.pipeline_manager.render_sections(
                        self.device, 
                        self.queue, 
                        self.frame, 
                        self.section_manager.get_sections(*section_start, *section_end));
                }

                _ => {},
            }
        }
    }
}

fn instances_to_load(range_requested: (usize, usize), on_gpu: Option<(usize, usize)>, total_instances: usize) -> Option<(usize, usize)> {
    // Check if anything is on gpu
    if let Some((on_gpu_start_id, on_gpu_end_id)) = on_gpu {
        // If required instances are in the range of what is on the GPU -> do nothing
        if range_requested.0 >= on_gpu_start_id && range_requested.1 <= on_gpu_end_id { return None; }
    }

    // Else get all the instances that are possible starting from the first requested
    let new_end = (range_requested.0 + MAX_INSTANCES).min(total_instances);

    Some((range_requested.0, new_end))
} 