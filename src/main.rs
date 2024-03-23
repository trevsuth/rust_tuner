mod audio_input;
mod fft;

use crate::audio_input::capture_audio;
use std::sync::mpsc::channel;

fn main() {
    let (sender, receiver) = channel();
    capture_audio(sender);

    loop {
        match receiver.try_recv() {
            Ok(buffer) => {
                let fft_result = fft::perform_fft(&buffer);
                //Process FFT results here, such as finding the dominant frequency
                println!("Dominant Frequency: {:?}", fft_result);
            },
            Err(_) => continue, // No data available yet
        }
    }
}
