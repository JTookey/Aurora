use bytemuck::{Pod, Zeroable};

pub struct InstanceManager {
    line_instances: Vec<LineInstance>,
    primative_instances: Vec<PrimativeInstance>,
    geometry_instances: Vec<GeometryInstance>,
}

impl InstanceManager {
    pub fn new() -> Self {
        Self {
            line_instances: Vec::new(),
            primative_instances: Vec::new(),
            geometry_instances: Vec::new(),
        }
    }

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

    pub fn push_primative_instance(&mut self, prim: PrimativeInstance) -> usize {
        self.primative_instances.push(prim);
        self.primative_instances.len() - 1
    }

    pub fn n_primative_instances(&self) -> usize {
        self.primative_instances.len()
    }

    pub fn push_geometry_instance(&mut self, geom: GeometryInstance) -> usize {
        self.geometry_instances.push(geom);
        self.geometry_instances.len() - 1
    }

    pub fn n_geometry_instances(&self) -> usize {
        self.geometry_instances.len()
    }

    pub fn clear(&mut self) {
        self.line_instances.clear();
        self.primative_instances.clear();
        self.geometry_instances.clear();
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct LineInstance {
    pub position_1: [f32;2],
    pub position_2: [f32;2],
    pub line_colour: [f32;4],
    pub line_width: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct PrimativeInstance {

}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct GeometryInstance {

}