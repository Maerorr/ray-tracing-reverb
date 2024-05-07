use crate::SAMPLE_RATE;

pub struct SoundRaw {
    pub data: Vec<f32>,
}

impl SoundRaw {
    pub fn new(data: Vec<f32>) -> Self {
        let mut data = data;
        for _ in 0..(20 * SAMPLE_RATE) {
            data.push(0.0);
        }
        Self { data }
    }

    pub fn add_with_delay(&mut self, ms: f32, vol: f32) {
        let delay = (ms * 0.001 * SAMPLE_RATE as f32) as usize;
        let mut new_data = Vec::new();
        for i in 0..self.data.len() {
            if i < delay {
                new_data.push(self.data[i] as f32);
            } else {
                new_data.push(self.data[i] + self.data[i - delay] * vol);
            }
        }
        self.data = new_data;
    }

    pub fn normalize(&mut self) {
        let max = self.data.iter().fold(0.0f32, |acc, &x| acc.max(x.abs()));
        for i in 0..self.data.len() {
            self.data[i] /= max;
        }
    }
}