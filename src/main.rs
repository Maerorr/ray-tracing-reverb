use std::f32::consts::PI;
use std::fs::File;
use std::i32::MAX;
use std::io::Write;
use audio::{save_as_wav_mono, SoundRaw};
use float_cmp::{approx_eq};
use geometry::{create_box_surfaces, Sphere, Surface};
use hound::WavReader;
use image::{DynamicImage, ImageBuffer};
use math::{Quaternion, RayCastHit, Vector};
use ray_trace::RayTrace;
use scene::Scene;
use scenes::create_default_scene;

use crate::math::{as_degrees, as_radians, IntersectionPrimitive};

use crate::geometry::{Line, Triangle};

mod scene;
mod material;

mod geometry;
mod math;
mod audio;
mod scenes;
mod ray_trace;

const SAMPLE_RATE: u32 = 48000;
const SPEED_OF_SOUND: f32 = 343.0;

fn main() {
    let mut sound = hound::WavReader::open("res/noise.wav").unwrap();
    let data = sound.samples::<i32>().map(|s| s.unwrap()).collect::<Vec<i32>>();
    let data_f32 = data.iter().map(|&x| x as f32 / 16777216.0).collect::<Vec<f32>>();

    let mut sound_raw = SoundRaw::new(data_f32);
    sound_raw.normalize();

    let (scene, materials) = create_default_scene();
    let sound_source_position = Vector::new(0.0, 0.0, -12.0);
    let listener = Surface::new_vw(
        Vector::new(0.0, 0.0, -12.0),
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        Some((-3.0, 3.0)),
        Some((-3.0, 3.0)),
        Vector::new(0.0, 0.0, -1.0),
    );

    let mut raytrace = RayTrace::new(scene, materials, sound_source_position, listener);
    raytrace.run();

    println!("{}", raytrace.returned_times.len());

    let delays_count = raytrace.returned_times.len();
    for time in raytrace.returned_times.iter() {
        sound_raw.add_with_delay(*time * 1000.0, 1.0 / delays_count as f32);
    }
    sound_raw.normalize();
    save_as_wav_mono(sound_raw.data, "res/noise_delayed.wav");
}