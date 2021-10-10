use iced::{
    executor, time, Application, Command, Container, Element, Length, alignment,
    Subscription, button, Text, Alignment,
    window::icon,
};
use std::sync::mpsc;


use audioviz;
mod audio;
use audio::*;
mod style;

mod ui;
use ui::bars::*;
use ui::settings::{SettingMessage, Settings};

use gag::Gag;

pub fn main() -> iced::Result {
    // dont print any alsa or jack errors on *nix systems to stderr
    let _print_gag = Gag::stderr().unwrap();

    let image = include_bytes!("../media/icon.png");
    let image = image::load_from_memory(image).unwrap().to_rgba8();
    let rgba: Vec<u8> = image.as_raw().to_vec();

    let icon = icon::Icon::from_rgba(rgba, 1024, 1024).unwrap();
    
    Visual::run(iced::Settings {
        antialiasing: true,
        window: iced::window::Settings {
            icon: Some(icon),
            min_size: Some((500, 0)),
            ..iced::window::Settings::default()
        },
        ..iced::Settings::default()
    })
}

struct Visual {
    theme: style::Theme,
    bars: Bars,
    settings: Settings,
    event_sender: mpsc::Sender<audioviz::Event>,
    toggle_button_state: button::State,
    show_sliders: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    SliderMessage(SettingMessage),
    Update,
    ToggleSliders,
}

impl Application for Visual{
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let audio_device = AudioDevice::Output(0);
        let config = audioviz::Config {
            volume: 100.0,
            max_frequency: 20_000,
            frequency_scale_amount: 1,
            frequency_scale_range: [0, 100],
            ..Default::default()
        };
        let audio_stream = audioviz::AudioStream::init(
            config,
        );
        let event_sender = audio_stream.get_event_sender();
        let (device_tx, device_rx) = mpsc::channel();
        init_audio_sender(event_sender.clone(), audio_device, device_rx);
        
        (
            Visual {
                theme: Default::default(),
                //bars: Bars {data: Vec::new(), ..Default::default()},
                bars: Default::default(),
                settings: Settings::new(event_sender.clone(), style::Theme::default(), config, device_tx),
                event_sender,
                toggle_button_state: button::State::new(),
                show_sliders: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Audiolizer")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Update => {
                self.bars.cache.clear();
                let (tx, rx) = mpsc::channel();
                self.event_sender.send(audioviz::Event::RequestData(tx)).unwrap();
                self.bars.data = rx.recv().unwrap();

                if self.bars.mirroring {
                    for i in 0..self.bars.data.len() {
                        self.bars.data.insert(0, self.bars.data[i * 2]);
                    }
                }
            },
            Message::ToggleSliders => {
                self.show_sliders = !self.show_sliders;
            },
            Message::SliderMessage(msg) => {
                match msg {
                    SettingMessage::ThemeChanged(t) => {
                        self.theme = t;
                        self.settings.update(msg)
                    }
                    SettingMessage::Mirroring(v) => {
                        self.bars.mirroring = v;
                        self.settings.update(msg)
                    }
                    _ => {
                        self.settings.update(msg)
                    }
                }
            },
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(std::time::Duration::from_millis(10))
            .map(|_| Message::Update)
    }


    fn view(&mut self) -> Element<Message> { 
        let canvas = self.bars.view();
        
        let slider_toggle = button::Button::new(
            &mut self.toggle_button_state,
            Text::new("Settings").horizontal_alignment(alignment::Horizontal::Center),
        )
        .on_press(Message::ToggleSliders)
        .style(self.theme);
        
        let bars: Element<Message> = Container::new(canvas)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(2)
            .center_x()
            .center_y()
            .style(self.theme)
            .into();

        let mut content = iced::Column::new()
            .align_items(Alignment::End)
            .height(Length::Fill)
            .spacing(2)
            .padding(2)
            .push(slider_toggle)
            .push(bars);

        if self.show_sliders {
            content = content.push(
                self.settings.view()
                    .map(Message::SliderMessage)
            );
        }
        
        
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(self.theme)
            .into()

    }
}