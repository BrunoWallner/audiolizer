use iced::{
    Element, Row, Alignment, Text, Rule, Length, Column, Radio, scrollable, Checkbox,
    slider, button, alignment,
};
use std::sync::mpsc;
use audioviz;
use crate::style;
use crate::audio::AudioDevice;

#[derive(Debug, Clone)]
pub enum SliderMessage {
    DensityReduction(f32),
    Volume(f32),
    Buffering(f32),
    SmoothingSize(f32),
    SmoothingAmount(f32),
    ThemeChanged(style::Theme),
    Mirroring(bool),
    UpdateAudioDevices,
    AudioDeviceChanged(AudioDevice),
}

pub struct Sliders {
    theme: style::Theme,
    event_sender: mpsc::Sender<audioviz::Event>,
    scrollable: scrollable::State,


    audio_device_sender: mpsc::Sender<AudioDevice>,
    input_devices: Vec<String>,
    output_devices: Vec<String>,
    device_refresh_button: button::State,
    audio_device: AudioDevice,

    mirroring_checkbox: bool,

    volume_s: slider::State,
    volume_sv: f32,

    density_reduction_s: slider::State,
    density_reduction_sv: f32,

    smoothing_size_s: slider::State,
    smoothing_size_sv: f32,

    smoothing_amount_s: slider::State,
    smoothing_amount_sv: f32,

    buffering_s: slider::State,
    buffering_sv: f32,
}
impl Sliders {
    pub fn new(event_sender: mpsc::Sender<audioviz::Event>, theme: style::Theme, config: audioviz::Config, audio_device_sender: mpsc::Sender<AudioDevice>) -> Self {
        let (input_devices, output_devices) = crate::audio::iter_audio_devices();
        Sliders {
            theme,
            event_sender,
            scrollable: scrollable::State::new(),

            audio_device_sender,
            input_devices,
            output_devices,
            device_refresh_button: button::State::new(),
            audio_device: AudioDevice::Output(0),

            mirroring_checkbox: true,

            volume_s: slider::State::new(),
            volume_sv: config.volume as f32,
            density_reduction_s: slider::State::new(),
            density_reduction_sv: config.density_reduction as f32,
            smoothing_size_s: slider::State::new(),
            smoothing_size_sv: config.smoothing_size as f32,
            smoothing_amount_s: slider::State::new(),
            smoothing_amount_sv: config.smoothing_amount as f32,
            buffering_s: slider::State::new(),
            buffering_sv: config.buffering as f32,
        }
    }

    pub fn update(&mut self, msg: SliderMessage) {
        let (tx, rx) = mpsc::channel();
        self.event_sender.send(audioviz::Event::RequestConfig(tx)).unwrap();
        let config = rx.recv().unwrap();

        match msg {
            SliderMessage::Volume(v) => {
                self.volume_sv = v;
                if config.volume != v {
                    let config = audioviz::Config {
                        volume: v,
                        ..config
                    };
                    self.event_sender.send(audioviz::Event::SendConfig(config)).unwrap();
                } 
            }
            SliderMessage::DensityReduction(v) => {
                self.density_reduction_sv = v;
                let v = v as usize;
                if config.density_reduction != v {
                    let config = audioviz::Config {
                        density_reduction: v,
                        ..config
                    };
                    self.event_sender.send(audioviz::Event::ClearBuffer).unwrap();
                    self.event_sender.send(audioviz::Event::SendConfig(config)).unwrap();
                }
            }
            SliderMessage::Buffering(v) => {
                self.buffering_sv = v;
                let v = v as usize;
                if config.buffering != v {
                    let config = audioviz::Config {
                        buffering: v,
                        ..config
                    };
                    self.event_sender.send(audioviz::Event::SendConfig(config)).unwrap();
                } 
            }
            SliderMessage::SmoothingSize(v) => {
                self.smoothing_size_sv = v;
                let v = v as usize;
                if config.buffering != v {
                    let config = audioviz::Config {
                        smoothing_size: v,
                        ..config
                    };
                    self.event_sender.send(audioviz::Event::SendConfig(config)).unwrap();
                } 
            }
            SliderMessage::SmoothingAmount(v) => {
                self.smoothing_amount_sv = v;
                let v = v as usize;
                if config.buffering != v {
                    let config = audioviz::Config {
                        smoothing_amount: v,
                        ..config
                    };
                    self.event_sender.send(audioviz::Event::SendConfig(config)).unwrap();
                } 
            }
            SliderMessage::ThemeChanged(t) => {
                self.theme = t;
            }
            SliderMessage::Mirroring(b) => {
                self.mirroring_checkbox = b;
            }
            SliderMessage::UpdateAudioDevices => {
                let (i, o) = crate::audio::iter_audio_devices();
                self.input_devices = i;
                self.output_devices = o;
            }
            SliderMessage::AudioDeviceChanged(d) => {
                self.audio_device = d;
                self.audio_device_sender.send(d).unwrap();
            }
        }
    }

    pub fn view(&mut self) -> Element<crate::SliderMessage> {
        let mirroring_checkbox = Checkbox::new(
                self.mirroring_checkbox,
                String::from("mirroring"),
                SliderMessage::Mirroring,
            )
            .style(self.theme);

        let device_refresh_button = button::Button::new(
                &mut self.device_refresh_button,
                Text::new("refresh").horizontal_alignment(alignment::Horizontal::Center),
            )
            .on_press(SliderMessage::UpdateAudioDevices)
            .style(self.theme);

        let d_r_slider = slider::Slider::new(
            &mut self.density_reduction_s, 
            0.0..=20.0, 
            self.density_reduction_sv, 
            SliderMessage::DensityReduction)
            .style(self.theme);

        let volume_slider = slider::Slider::new(
            &mut self.volume_s, 
            0.0..=100.0, 
            self.volume_sv, 
            SliderMessage::Volume)
            .style(self.theme);
        
        let buffering_slider = slider::Slider::new(
                &mut self.buffering_s, 
                0.0..=20.0, 
                self.buffering_sv, 
                SliderMessage::Buffering
            )
            .style(self.theme);

        let smoothing_size_slider = slider::Slider::new(
                &mut self.smoothing_size_s,
                0.0..=20.0, 
                self.smoothing_size_sv, 
                SliderMessage::SmoothingSize
            )
            .style(self.theme);


        let smoothing_amount_slider = slider::Slider::new(
                &mut self.smoothing_amount_s,
                0.0..=20.0, 
                self.smoothing_amount_sv, 
                SliderMessage::SmoothingAmount
            )
            .style(self.theme);



        let volume = Row::new()
            .padding(5)
            .spacing(2)
            .align_items(Alignment::End)
            .push(Text::new("Volume").width(Length::FillPortion(1)))
            .push(volume_slider.width(Length::FillPortion(5)));

        let density_reduction = Row::new()
            .padding(5)
            .spacing(2)
            .align_items(Alignment::End)
            .push(Text::new("Density Reduction").width(Length::FillPortion(1)))
            .push(d_r_slider.width(Length::FillPortion(5)));
        
        let buffering = Row::new()
            .padding(5)
            .spacing(2)
            .push(Text::new("Buffering").width(Length::FillPortion(1)))
            .push(buffering_slider.width(Length::FillPortion(5)));

        let smoothing_size = Row::new()
            .padding(5)
            .spacing(2)
            .push(Text::new("Smoothing Size").width(Length::FillPortion(1)))
            .push(smoothing_size_slider.width(Length::FillPortion(5)));

        let smoothing_amount = Row::new()
            .padding(5)
            .spacing(2)
            .push(Text::new("Smoothing Amount").width(Length::FillPortion(1)))
            .push(smoothing_amount_slider.width(Length::FillPortion(5)));

        let light_radio = Radio::new(style::Theme::Light, "Light", Some(self.theme), SliderMessage::ThemeChanged)
            .style(self.theme);

        let dark_radio = Radio::new(style::Theme::Dark, "Dark", Some(self.theme), SliderMessage::ThemeChanged)
            .style(self.theme);

        let theme_selection = Row::new()
            .padding(2)
            .spacing(25)
            .push(light_radio)
            .push(dark_radio);

        let mut output_device_selection = Column::new()
            .padding(5)
            .spacing(5);
        for (i, name) in self.output_devices.iter().enumerate() {
            output_device_selection = output_device_selection.push(
                Radio::new(AudioDevice::Output(i), name, Some(self.audio_device), SliderMessage::AudioDeviceChanged)
            )
        };

        let mut input_device_selection = Column::new()
            .padding(5)
            .spacing(5);
        for (i, name) in self.input_devices.iter().enumerate() {
            input_device_selection = input_device_selection.push(
                Radio::new(AudioDevice::Input(i), name, Some(self.audio_device), SliderMessage::AudioDeviceChanged)
            )
        };

        let device_selection = Row::new()
            .padding(5)
            .spacing(2)
            .align_items(Alignment::Center)
            .push(output_device_selection)
            .push(input_device_selection);

        let device_selection = Column::new()
            .padding(5)
            .spacing(2)
            .push(device_refresh_button)
            .push(device_selection);


        
        let content = Column::new()
            .padding(2)
            .align_items(Alignment::Center)
            .spacing(2)
            .push(volume)
            .push(density_reduction)
            .push(buffering)
            .push(smoothing_size)
            .push(smoothing_amount)
            .push(mirroring_checkbox)
            .push(Rule::horizontal(10))
            .push(device_selection)
            .push(Rule::horizontal(10))
            .push(theme_selection);

        scrollable::Scrollable::new(&mut self.scrollable)
            .push(content)
            .width(Length::Fill)
            .height(Length::Shrink)
            .max_height(500)
            .style(self.theme)
            .into()
    }
}