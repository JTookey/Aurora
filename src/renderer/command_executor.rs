use crate::Colour;
use super::{CommandManager, PipelineManager, InternalCommands, MAX_INSTANCES};

pub struct CommandExecutor<'frame> {
    device: &'frame wgpu::Device,
    queue:  &'frame wgpu::Queue,
    frame: &'frame wgpu::SwapChainFrame,

    command_manager: &'frame CommandManager,
    pipeline_manager: &'frame mut PipelineManager,

    clear_colour: Option<&'frame Colour>,

    instances_on_gpu: Option<(usize, usize)>,
}

impl <'frame> CommandExecutor<'frame> {
    pub fn new(
        device: &'frame wgpu::Device,
        queue:  &'frame wgpu::Queue,
        frame: &'frame wgpu::SwapChainFrame,

        command_manager: &'frame CommandManager,
        pipeline_manager: &'frame mut PipelineManager,
    ) -> Self {
        Self {
            device,
            queue,
            frame,

            command_manager,
            pipeline_manager,

            clear_colour: None,

            instances_on_gpu: None,
        }   
    }

    pub fn build_frame(&mut self) {
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
                            self.instances_on_gpu, 
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
                            self.instances_on_gpu = Some((start_id, end_id));

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