use rust_realtime_nursery::waste_cpu_cycles;

const SAMPLE_RATE: f64 = 48_000.0;
const FRAMES: u32 = 256;
const CHANNELS: i32 = 1;

fn main() -> anyhow::Result<()> {
    let pa = portaudio::PortAudio::new()?;

    let mut iterations = 5;

    let settings = pa.default_output_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES)?;

    let callback = move |portaudio::OutputStreamCallbackArgs { buffer, flags, .. }| {
        waste_cpu_cycles();
        buffer.fill(0.0);
        if !flags.is_empty() {
            println!("callback flags: {}", flags);
        }

        let thread0 = std::thread::spawn(|| {
            waste_cpu_cycles();
        });

        thread0.join().unwrap();

        iterations -= 1;
        if iterations == 0 {
            portaudio::Complete
        } else {
            portaudio::Continue
        }
    };

    let mut stream = pa.open_non_blocking_stream(settings, callback)?;

    stream.start()?;

    std::thread::sleep(std::time::Duration::from_millis(50));

    stream.stop()?;

    Ok(())
}
