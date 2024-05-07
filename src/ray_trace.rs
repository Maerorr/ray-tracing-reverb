use crate::{geometry::{Line, Surface}, material::{Material, MaterialType}, math::{as_radians, RayCastHit, Vector}, scene::Scene, SPEED_OF_SOUND};

pub struct RayTrace {
    scene: Scene,
    materials: Vec<Material>,
    sound_source: Vector,
    listener: Surface,
    pub returned_times: Vec<f32>,
}

impl RayTrace {
    pub fn new(scene: Scene, materials: Vec<Material>, sound_source: Vector, listener: Surface) -> Self {
        Self {
            scene,
            materials,
            sound_source,
            listener,
            returned_times: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        for i in 0..16 {
            let direction = Vector::random_on_sphere();
            let ray = Line::new(self.sound_source, direction);
            self.shoot_ray(&ray, 0.0, 5)
        }
    }

    pub fn shoot_ray(&mut self, ray: &Line, time: f32, max_bounces: i32) {
        if max_bounces == -1 {
            return;
        }
        let mut closest_intersection = RayCastHit::new(None);
        let mut closest_distance = 0.0;
        let mut closest_material_idx = 0;
        for (i, primitive) in self.scene.primitives.iter().enumerate() {
            let hit = primitive.intersect(&ray);
            if hit.is_some() {
                let from_cam_to_point = hit.unwrap().0 - ray.point;
    
                if from_cam_to_point.dot(&ray.direction) >= 0.0 {
                    let intersection = hit.unwrap();
                    let distance = ray.point.distance(&intersection.0);
    
                    if closest_intersection.is_none() {
                        closest_intersection = hit.clone();
                        closest_material_idx = self.scene.material_index[i];
                        closest_distance = distance;
                    } else if distance < closest_distance {
                        closest_intersection = hit.clone();
                        closest_material_idx = self.scene.material_index[i];
                        closest_distance = distance;
                    }
                }
            }
        }

        if closest_intersection.is_some() {
            let intersection = closest_intersection.unwrap().0;
            let normal = closest_intersection.normal.unwrap();
            let material = &self.materials[closest_material_idx];
            let distance = ray.point.distance(&intersection);
            let total_sound_time = time + distance / SPEED_OF_SOUND;

            match material.material_type {
                MaterialType::Listener => {
                    self.returned_times.push(total_sound_time);
                    return;
                },
                MaterialType::Normal => {
                    let reflected_dir = ray.direction.reflect(&normal);
                    for _ in 0..8 {
                        let dir = Vector::random_around_direction(&reflected_dir, as_radians(15.0));
                        let ray = Line::new(intersection, dir);
                        self.shoot_ray(&ray, total_sound_time, max_bounces - 1);
                    }

                }
            }
        } else { return; }
    }
}

