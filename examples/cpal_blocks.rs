use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");

    let mut supported_configs_range = device
        .supported_output_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range
        .next()
        .expect("no supported config?!")
        .with_max_sample_rate();
    let config = supported_config.into();
    //let config = device.default_output_config().unwrap().into();
    let stream = device
        .build_output_stream(
            &config,
            move |_data: &mut [f32], _: &cpal::OutputCallbackInfo| {},
            move |_err| {},
        )
        .unwrap();
    stream.play().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));
}
