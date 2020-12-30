use std::mem;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct LineInstance {
    pub position_1: [f32;2],
    pub position_2: [f32;2],
    pub line_colour: [f32;4],
    pub line_width: f32,
}

impl <'a> LineInstance {
    pub fn desc() -> wgpu::VertexBufferDescriptor<'a> {
        wgpu::VertexBufferDescriptor {
            step_mode: wgpu::InputStepMode::Instance,
            stride: mem::size_of::<LineInstance>() as wgpu::BufferAddress,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    format: wgpu::VertexFormat::Float2,
                    offset: 0,
                    shader_location: 4,
                },
                wgpu::VertexAttributeDescriptor {
                    format: wgpu::VertexFormat::Float2,
                    offset: mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 5,
                },
                wgpu::VertexAttributeDescriptor {
                    format: wgpu::VertexFormat::Float4,
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                },
                wgpu::VertexAttributeDescriptor {
                    format: wgpu::VertexFormat::Float,
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                },
            ]
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct TwoDInstance {
    pub position: [f32;2],      // 8
    pub size: [f32;2],          // 8
    pub colour: [f32;4],        // 16
    pub texture: [f32;4],       // 16 TL & BR
    pub texture_opacity: f32,   // 4
    pub line_width: f32,        // 4
    pub corner_radius: f32,     // 4
    pub rotation: f32,          // 4
    pub shape: u32,             // 4
}

impl <'a> TwoDInstance {
    pub fn desc() -> wgpu::VertexBufferDescriptor<'a> {
        wgpu::VertexBufferDescriptor {
            step_mode: wgpu::InputStepMode::Instance,
            stride: mem::size_of::<TwoDInstance>() as wgpu::BufferAddress,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    format: wgpu::VertexFormat::Float2,
                    offset: 0,
                    shader_location: 4,
                },
                wgpu::VertexAttributeDescriptor {
                    format: wgpu::VertexFormat::Float2,
                    offset: mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 5,
                },
                wgpu::VertexAttributeDescriptor {
                    format: wgpu::VertexFormat::Float4,
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                },
                wgpu::VertexAttributeDescriptor {
                    format: wgpu::VertexFormat::Float4,
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                },
                wgpu::VertexAttributeDescriptor {
                    format: wgpu::VertexFormat::Float,
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                },
                wgpu::VertexAttributeDescriptor {
                    format: wgpu::VertexFormat::Float,
                    offset: mem::size_of::<[f32; 13]>() as wgpu::BufferAddress,
                    shader_location: 9,
                },
                wgpu::VertexAttributeDescriptor {
                    format: wgpu::VertexFormat::Float,
                    offset: mem::size_of::<[f32; 14]>() as wgpu::BufferAddress,
                    shader_location: 10,
                },
                wgpu::VertexAttributeDescriptor {
                    format: wgpu::VertexFormat::Float,
                    offset: mem::size_of::<[f32; 15]>() as wgpu::BufferAddress,
                    shader_location: 11,
                },
                wgpu::VertexAttributeDescriptor {
                    format: wgpu::VertexFormat::Uint,
                    offset: mem::size_of::<[f32; 16]>() as wgpu::BufferAddress,
                    shader_location: 12,
                },
            ]
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct ThreeDInstance {

}