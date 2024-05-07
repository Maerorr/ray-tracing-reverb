use crate::{geometry::Surface, material::{Material, MaterialType}, math::Vector, scene::Scene};

pub fn create_default_scene() -> (Scene, Vec<Material>) {
    let listener_material = Material {
        material_type: MaterialType::Listener,
    };
    let normal_material = Material {
        material_type: MaterialType::Normal,
    };
    let mut materials = Vec::new(); 
    materials.push(listener_material);
    materials.push(normal_material);

    let mut scene = Scene::new();

    let floor = Surface::new_vw(
        Vector::new(0.0, -5.0, 0.0),
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        None,
        None,
        Vector::new(0.0, 1.0, 0.0),
    );
    scene.add_primitive(Box::new(floor), 1);

    let left_wall = Surface::new_vw(
        Vector::new(-15.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        Vector::new(0.0, 1.0, 0.0),
        None,
        None,
        Vector::new(1.0, 0.0, 0.0),
    );

    scene.add_primitive(Box::new(left_wall), 1);

    let right_wall = Surface::new_vw(
        Vector::new(15.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        Vector::new(0.0, 1.0, 0.0),
        None,
        None,
        Vector::new(-1.0, 0.0, 0.0),
    );

    scene.add_primitive(Box::new(right_wall), 1);

    let back_wall = Surface::new_vw(
        Vector::new(0.0, 0.0, -15.0),
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        None,
        None,
        Vector::new(0.0, 0.0, 1.0),
    );

    scene.add_primitive(Box::new(back_wall), 1);

    let front_wall = Surface::new_vw(
        Vector::new(0.0, 0.0, 15.0),
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        None,
        None,
        Vector::new(0.0, 0.0, -1.0),
    );

    scene.add_primitive(Box::new(front_wall), 1);

    let listener = Surface::new_vw(
        Vector::new(0.0, 0.0, 0.0),
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        Some((-1.0, 1.0)),
        Some((-1.0, 1.0)),
        Vector::new(0.0, 1.0, 0.0),
    );

    scene.add_primitive(Box::new(listener), 0);

    (scene, materials)
}