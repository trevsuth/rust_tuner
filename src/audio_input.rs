use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}};
use std::sync::mpsc::Sender;

pub fn capture_audio(sender: Sender<Vec<f32>>) {
    let host = cpal::default_host();
    let device = host.default_input_device().expect("Failed to find input device");
    let config = device.default_input_config().expect("Failed to get input config");

    let stream = device.build_input_stream(
        &config.into(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            sender.send(data.to_vec()).ok();
        },
        |err| eprintln!("Error: {:?}", err),
    ).unwrap();

    stream.play().unwrap();
    println!("Audio input stream has started.");
}