use std::fs::File;
use std::io::Write;
use float_cmp::{approx_eq};
use geometry::{create_box_surfaces, Sphere, Surface};
use image::{DynamicImage, ImageBuffer};
use math::{Quaternion, RayCastHit, Vector};
use scene::Scene;

use crate::math::{as_degrees, as_radians, IntersectionPrimitive};

use crate::geometry::{Line, Triangle};

mod scene;

mod geometry;
mod math;

fn main() {

}