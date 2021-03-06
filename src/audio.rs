use std::thread;
use std::sync::mpsc;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use colored::*;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum AudioDevice {
    Input(usize),
    Output(usize),
}
impl PartialEq for AudioDevice {
    fn eq(&self, other: &Self) -> bool {
        match *self {
            AudioDevice::Input(a) => {
                match *other {
                    AudioDevice::Input(b) => {
                        return a == b;
                    }
                    _ =>  {
                        return false;
                    }
                }
            },
            AudioDevice::Output(a) => {
                match *other {
                    AudioDevice::Output(b) => {
                        return a == b;
                    }
                    _ =>  {
                        return false;
                    }
                }      
            }
        }
    }
}
impl Eq for AudioDevice {}

pub fn init_audio_sender(event_sender: mpsc::Sender<audioviz::Event>, mut audio_device: AudioDevice, audio_device_receiver: mpsc::Receiver<AudioDevice>) {
    thread::spawn(move || loop {
        let _stream = match stream_audio(event_sender.clone(), audio_device) {
            Ok(s) => Some(s),
            Err(_) => None
        };
        audio_device = match audio_device_receiver.recv() {
            Ok(v) => v,
            Err(_) => break
        };
    });
}

fn stream_audio(event_sender: mpsc::Sender<audioviz::Event>, audio_device: AudioDevice) -> Result<cpal::Stream, ()> {
    let event_sender = event_sender.clone();

    let host = cpal::default_host();
    let input_devices = host.input_devices().unwrap().collect::<Vec<cpal::Device>>();
    let output_devices = host.output_devices().unwrap().collect::<Vec<cpal::Device>>();

    let device = match audio_device {
        AudioDevice::Input(i) => {
            &input_devices[i]
        }
        AudioDevice::Output(i) => {
            &output_devices[i]
        }
    };

    match audio_device {
        AudioDevice::Input(_) => {
            println!("using input device: {}", device.name().unwrap().green().bold());
        }
        AudioDevice::Output(_) => {
            println!("using output device: {}", device.name().unwrap().green().bold());
        }
    }

    //let device_config =  device.default_input_config().unwrap();
    let device_config = match audio_device {
        AudioDevice::Input(_) => {
            device.default_input_config().unwrap()
        }
        AudioDevice::Output(_) => {
            device.default_output_config().unwrap()
        }
    };

    let stream = match device_config.sample_format() {
        cpal::SampleFormat::F32 => match device.build_input_stream(
            &device_config.into(),
            move |data, _: &_| handle_input_data_f32(data, event_sender.clone()),
            err_fn,
        ) {
            Ok(v) => v,
            Err(_) => return Err(())
        },
        _ => {
            // Unsupported sample forma
            return Err(());
        }
    };

    stream.play().unwrap();

    Ok(stream)
}

#[allow(dead_code)]
pub fn iter_audio_devices() -> (Vec<String>, Vec<String>) {
    let host = cpal::default_host();
    let input_devices = 
        host.input_devices()
        .unwrap()
        .collect::<Vec<cpal::Device>>();

    let output_devices = 
        host.output_devices()
        .unwrap()
        .collect::<Vec<cpal::Device>>();
    
    let input_devices: Vec<String> = input_devices.iter().map(|x| x.name().unwrap()).collect();
    let output_devices: Vec<String> = output_devices.iter().map(|x| x.name().unwrap()).collect();

    (input_devices, output_devices)
}

fn handle_input_data_f32(data: &[f32], sender: mpsc::Sender<audioviz::Event>) {
    // sends the raw data to audio_stream via the event_sender
    sender.send(audioviz::Event::SendData(data.to_vec())).unwrap();
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", err);
}