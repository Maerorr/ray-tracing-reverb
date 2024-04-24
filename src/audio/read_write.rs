use crate::SAMPLE_RATE;

pub fn save_as_wav_mono(samples: Vec<f32>, filename: &str) {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    let mut writer = hound::WavWriter::create(filename, spec).unwrap();
    for sample in samples {
        writer.write_sample(sample).unwrap();
    }
    writer.finalize().unwrap();
}

pub fn save_as_wav_stereo(samples_left: Vec<f32>, samples_right: Vec<f32>, filename: &str) {
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    let mut writer = hound::WavWriter::create(filename, spec).unwrap();
    for (left, right) in samples_left.iter().zip(samples_right.iter()) {
        writer.write_sample(*left).unwrap();
        writer.write_sample(*right).unwrap();
    }
    writer.finalize().unwrap();
}