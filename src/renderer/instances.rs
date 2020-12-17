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

    pub fn clear(&mut self) {
        self.line_instances.clear();
        self.primative_instances.clear();
        self.geometry_instances.clear();
    }
}

pub struct LineInstance {

}

pub struct PrimativeInstance {

}

pub struct GeometryInstance {

}