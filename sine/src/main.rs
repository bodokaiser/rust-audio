use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("no default output device found");

    let config = device
        .default_output_config()
        .expect("no default config available");

    let channels = config.channels() as usize;
    let mut sample_clock = 0f32;
    let sample_rate = config.sample_rate().0 as f32;

    let mut next_value = move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        (sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
    };

    let stream = device
        .build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for frame in data.chunks_mut(channels) {
                    let value: f32 = cpal::Sample::from::<f32>(&next_value());
                    for sample in frame.iter_mut() {
                        *sample = value;
                    }
                }
            },
            move |err| eprintln!("an error occurred on the output audio stream: {}", err),
        )
        .unwrap();
    stream.play().unwrap();

    std::thread::sleep(std::time::Duration::from_millis(1000));
}
