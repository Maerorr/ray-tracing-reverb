use crate::{math::intersection::IntersectionPrimitive};

pub struct Scene {
    pub primitives: Vec<Box<dyn IntersectionPrimitive + Send + Sync>>,
    pub material_index: Vec<usize>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            primitives: Vec::new(),
            material_index: Vec::new(),
        }
    }

    pub fn add_primitive(&mut self, primitive: Box<dyn IntersectionPrimitive + Send + Sync>, material_idx: usize) {
        self.primitives.push(primitive);
        self.material_index.push(material_idx);
    }
}