use iced::{
    Element, Row, Alignment, Text, Rule, Length, Column, Radio, scrollable, Checkbox,
    slider, button, alignment,
};
use std::sync::mpsc;
use audioviz;
use crate::theme::Theme;
use crate::audio::AudioDevice;

#[derive(Debug, Clone)]
pub enum SettingMessage {
    BarCount(f32),
    FftResolution(f32),
    Volume(f32),
    Buffering(f32),
    SmoothingSize(f32),
    SmoothingAmount(f32),
    ThemeChanged(Theme),
    Mirroring(bool),
    UpdateAudioDevices,
    AudioDeviceChanged(AudioDevice),
    BarWidthChanged(f32),
    BarRefreshRate(f32),
    MaxFreq(f32),
}

pub struct Settings {
    theme: Theme,
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

    bar_count_s: slider::State,
    bar_count_sv: f32,

    fft_res_s: slider::State,
    fft_res_sv: f32,

    smoothing_size_s: slider::State,
    smoothing_size_sv: f32,

    smoothing_amount_s: slider::State,
    smoothing_amount_sv: f32,

    bar_width_s: slider::State,
    bar_width_sv: f32,

    bar_rr_s: slider::State,
    pub bar_rr_sv: f32, // needs to be public because of bar refresh subscription in main.rs

    max_freq_s: slider::State,
    max_freq_sv: f32,

    buffering_s: slider::State,
    buffering_sv: f32,
}
impl Settings {
    pub fn new(event_sender: mpsc::Sender<audioviz::Event>, theme: Theme, config: audioviz::Config, audio_device_sender: mpsc::Sender<AudioDevice>) -> Self {
        let (input_devices, output_devices) = crate::audio::iter_audio_devices();
        Settings {
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
            volume_sv: config.volume.sqrt(),
            bar_count_s: slider::State::new(),
            bar_count_sv: config.bar_count as f32,
            fft_res_s: slider::State::new(),
            fft_res_sv: config.fft_resolution as f32,
            smoothing_size_s: slider::State::new(),
            smoothing_size_sv: config.smoothing_size as f32,
            smoothing_amount_s: slider::State::new(),
            smoothing_amount_sv: config.smoothing_amount as f32,
            buffering_s: slider::State::new(),
            buffering_sv: config.buffering as f32,
            bar_width_s: slider::State::new(),
            bar_width_sv: 10.0,
            bar_rr_s: slider::State::new(),
            bar_rr_sv: 60.0,
            max_freq_s: slider::State::new(),
            max_freq_sv: config.max_frequency as f32 / 100.0,
        }
    }

    pub fn update(&mut self, msg: SettingMessage) {
        let (tx, rx) = mpsc::channel();
        self.event_sender.send(audioviz::Event::RequestConfig(tx)).unwrap();
        let config = rx.recv().unwrap();

        match msg {
            SettingMessage::Volume(v) => {
                self.volume_sv = v;
                if config.volume != v {
                    let config = audioviz::Config {
                        volume: v.powi(2),
                        ..config
                    };
                    self.event_sender.send(audioviz::Event::SendConfig(config)).unwrap();
                } 
            }
            SettingMessage::BarCount(v) => {
                self.bar_count_sv = v;
                let v = v as usize;
                if config.bar_count != v {
                    let config = audioviz::Config {
                        bar_count: v,
                        ..config
                    };
                    self.event_sender.send(audioviz::Event::ClearBuffer).unwrap();
                    self.event_sender.send(audioviz::Event::SendConfig(config)).unwrap();
                }
            }
            SettingMessage::FftResolution(v) => {
                self.fft_res_sv = v;
                let v = v as usize;
                if config.fft_resolution != v {
                    let config = audioviz::Config {
                        fft_resolution: v,
                        ..config
                    };
                    self.event_sender.send(audioviz::Event::ClearBuffer).unwrap();
                    self.event_sender.send(audioviz::Event::SendConfig(config)).unwrap();
                }
            }
            SettingMessage::Buffering(v) => {
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
            SettingMessage::SmoothingSize(v) => {
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
            SettingMessage::SmoothingAmount(v) => {
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
            SettingMessage::ThemeChanged(t) => {
                self.theme = t;
            }
            SettingMessage::Mirroring(b) => {
                self.mirroring_checkbox = b;
            }
            SettingMessage::UpdateAudioDevices => {
                let (i, o) = crate::audio::iter_audio_devices();
                self.input_devices = i;
                self.output_devices = o;
            }
            SettingMessage::AudioDeviceChanged(d) => {
                self.audio_device = d;
                self.audio_device_sender.send(d).unwrap();
            }
            SettingMessage::BarWidthChanged(w) => {
                self.bar_width_sv = w;
            }
            SettingMessage::BarRefreshRate(r) => {
                self.bar_rr_sv = r;
                if config.refresh_rate != r as usize {
                    let config = audioviz::Config {
                        refresh_rate: r as usize,
                        ..config
                    };
                    self.event_sender.send(audioviz::Event::SendConfig(config)).unwrap();
                }
            }
            SettingMessage::MaxFreq(m) => {
                self.max_freq_sv = m;
                let m = match m as usize {
                    1..=20_000 => m as usize * 100,
                    _ =>  20_000,
                };
                if config.max_frequency != m {
                    let config = audioviz::Config {
                        max_frequency: m,
                        ..config
                    };
                    self.event_sender.send(audioviz::Event::ClearBuffer).unwrap();
                    self.event_sender.send(audioviz::Event::SendConfig(config)).unwrap();
                } 
            }
        }
    }

    pub fn view(&mut self) -> Element<SettingMessage> {
        let mirroring_checkbox = Checkbox::new(
                self.mirroring_checkbox,
                String::from("mirroring"),
                SettingMessage::Mirroring,
            )
            .style(self.theme);

        let device_refresh_button = button::Button::new(
                &mut self.device_refresh_button,
                Text::new("refresh").horizontal_alignment(alignment::Horizontal::Center),
            )
            .on_press(SettingMessage::UpdateAudioDevices)
            .style(self.theme);

        let b_slider = slider::Slider::new(
            &mut self.bar_count_s, 
            1.0..=2000.0, 
            self.bar_count_sv, 
            SettingMessage::BarCount)
            .style(self.theme);

        let fft_r_slider = slider::Slider::new(
            &mut self.fft_res_s, 
            100.0..=16256.0, 
            self.fft_res_sv, 
            SettingMessage::FftResolution)
            .style(self.theme);

        let volume_slider = slider::Slider::new(
            &mut self.volume_s, 
            1.0..=50.0, 
            self.volume_sv, 
            SettingMessage::Volume)
            .style(self.theme);
        
        let buffering_slider = slider::Slider::new(
                &mut self.buffering_s, 
                1.0..=30.0, 
                self.buffering_sv, 
                SettingMessage::Buffering
            )
            .style(self.theme);

        let smoothing_size_slider = slider::Slider::new(
                &mut self.smoothing_size_s,
                1.0..=20.0, 
                self.smoothing_size_sv, 
                SettingMessage::SmoothingSize
            )
            .style(self.theme);


        let smoothing_amount_slider = slider::Slider::new(
                &mut self.smoothing_amount_s,
                1.0..=20.0, 
                self.smoothing_amount_sv, 
                SettingMessage::SmoothingAmount
            )
            .style(self.theme);



        let volume = Row::new()
            .padding(5)
            .spacing(2)
            .align_items(Alignment::End)
            .push(Text::new("Volume").width(Length::FillPortion(1)))
            .push(volume_slider.width(Length::FillPortion(5)));

        let bar_count = Row::new()
            .padding(5)
            .spacing(2)
            .align_items(Alignment::End)
            .push(Text::new("number of bars").width(Length::FillPortion(1)))
            .push(b_slider.width(Length::FillPortion(5)));

        let fft_bar_count = Row::new()
            .padding(5)
            .spacing(2)
            .align_items(Alignment::End)
            .push(Text::new("FFT resolution").width(Length::FillPortion(1)))
            .push(fft_r_slider.width(Length::FillPortion(5)));
        
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


        //
        // Theme selection
        //
        let light_radio = Radio::new(Theme::Light, "Light", Some(self.theme), SettingMessage::ThemeChanged)
            .style(self.theme);

        let dark_radio = Radio::new(Theme::Dark, "Dark", Some(self.theme), SettingMessage::ThemeChanged)
            .style(self.theme);

        let midnight_radio = Radio::new(Theme::Midnight, "Midnight", Some(self.theme), SettingMessage::ThemeChanged)
            .style(self.theme);

        let theme_selection = Row::new()
            .padding(2)
            .spacing(25)
            .push(light_radio)
            .push(dark_radio)
            .push(midnight_radio);
        // END Theme Selection


        //
        // Audio device selection
        //
        let mut output_device_selection = Column::new()
            .padding(5)
            .spacing(5);
        for (i, name) in self.output_devices.iter().enumerate() {
            output_device_selection = output_device_selection.push(
                Radio::new(AudioDevice::Output(i), name, Some(self.audio_device), SettingMessage::AudioDeviceChanged)
                    .style(self.theme)
            )
        };

        let mut input_device_selection = Column::new()
            .padding(5)
            .spacing(5);
        for (i, name) in self.input_devices.iter().enumerate() {
            input_device_selection = input_device_selection.push(
                Radio::new(AudioDevice::Input(i), name, Some(self.audio_device), SettingMessage::AudioDeviceChanged)
                    .style(self.theme)
            )
        };

        let device_selection = Row::new()
            .padding(5)
            .spacing(2)
            .align_items(Alignment::Start)
            .push(output_device_selection)
            .push(input_device_selection);

        let device_selection = Column::new()
            .padding(5)
            .spacing(2)
            .align_items(Alignment::Center)
            .push(device_refresh_button)
            .push(device_selection);
        // END Audio device selection


        //
        //  Bar Settings
        //
        let bar_width_slider = slider::Slider::new(
                &mut self.bar_width_s,
                0.0..=10.0,
                self.bar_width_sv,
                SettingMessage::BarWidthChanged,
            )
            .style(self.theme);

        let bar_refresh_rate_slider = slider::Slider::new(
                &mut self.bar_rr_s,
                1.0..=1000.0,
                self.bar_rr_sv,
                SettingMessage::BarRefreshRate,
            )
            .style(self.theme);

        let max_freq_slider = slider::Slider::new(
                &mut self.max_freq_s,
                10.0..=200.0,
                self.max_freq_sv,
                SettingMessage::MaxFreq,
            )
            .style(self.theme);
        

        let bar_settings = Column::new()
            .push(
            Row::new()
                    .push(Text::new("Bar Width")
                        .width(Length::FillPortion(1))
                    )
                    .push(bar_width_slider
                        .width(Length::FillPortion(5))
                    )
                )

            .push(
            Row::new()
                    .push(Text::new("Bar refresh rate")
                        .width(Length::FillPortion(1))
                    )
                    .push(bar_refresh_rate_slider
                        .width(Length::FillPortion(5))
                    )
                )

            .push(
                Row::new()
                        .push(Text::new("Max frequency")
                            .width(Length::FillPortion(1))
                        )
                        .push(max_freq_slider
                            .width(Length::FillPortion(5))
                        )
                    )
            .spacing(5)
            .padding(5);
        // END Bar Settings


        
        let content = Column::new()
            .padding(2)
            .align_items(Alignment::Center)
            .spacing(2)
            .push(volume)
            .push(fft_bar_count)
            .push(bar_count)
            .push(buffering)
            .push(smoothing_size)
            .push(smoothing_amount)
            .push(mirroring_checkbox)
            .push(Rule::horizontal(10))
            .push(bar_settings)
            .push(Rule::horizontal(10))
            .push(Rule::horizontal(10))
            .push(device_selection)
            .push(Rule::horizontal(10))
            .push(theme_selection);

        scrollable::Scrollable::new(&mut self.scrollable)
            .padding(5)
            .push(content)
            .width(Length::Fill)
            .height(Length::Shrink)
            .max_height(500)
            .style(self.theme)
            .into()
    }
}