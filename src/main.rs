use std::f32::consts::PI;
use std::fs::File;
use std::io::Write;
use audio::save_as_wav_mono;
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
mod audio;

const SAMPLE_RATE: u32 = 48000;

fn main() {
}