
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MaterialType {
    Listener,
    Normal,
}

pub struct Material {
    pub material_type: MaterialType,
}