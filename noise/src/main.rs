use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Sample;
use rand::Rng;

fn main() {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("no default output device found");

    let config = device
        .default_output_config()
        .expect("no default config available");

    let channels = config.channels() as usize;

    let stream = device
        .build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let mut rng = rand::thread_rng();

                for frame in data.chunks_mut(channels) {
                    for sample in frame.iter_mut() {
                        let s: f32 = rng.gen();
                        *sample = Sample::from(&s);
                    }
                }
            },
            move |err| eprintln!("an error occurred on the output audio stream: {}", err),
        )
        .unwrap();
    stream.play().unwrap();

    std::thread::sleep(std::time::Duration::from_millis(1000));
}
