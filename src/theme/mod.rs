use iced::{
    button, checkbox, container, progress_bar, radio, rule, scrollable,
    slider, text_input, toggler,
};

mod light;
mod dark;
mod midnight;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
    Midnight,
}

impl Theme {
    pub const ALL: [Theme; 3] = [Theme::Light, Theme::Dark, Theme::Midnight];
}

impl Default for Theme {
    fn default() -> Theme {
        Theme::Dark
    }
}

impl From<Theme> for Box<dyn container::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => light::Container.into(),
            Theme::Dark => dark::Container.into(),
            Theme::Midnight => midnight::Container.into(),

        }
    }
}

impl From<Theme> for Box<dyn radio::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => light::Radio.into(),
            Theme::Dark => dark::Radio.into(),
            Theme::Midnight => midnight::Radio.into(),
        }
    }
}

impl From<Theme> for Box<dyn text_input::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => light::TextInput.into(),
            Theme::Dark => dark::TextInput.into(),
            Theme::Midnight => midnight::TextInput.into(),
        }
    }
}

impl From<Theme> for Box<dyn button::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => light::Button.into(),
            Theme::Dark => dark::Button.into(),
            Theme::Midnight => midnight::Button.into(),
        }
    }
}

impl From<Theme> for Box<dyn scrollable::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => light::Scrollable.into(),
            Theme::Dark => dark::Scrollable.into(),
            Theme::Midnight => midnight::Scrollable.into(),
        }
    }
}

impl From<Theme> for Box<dyn slider::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => light::Slider.into(),
            Theme::Dark => dark::Slider.into(),
            Theme::Midnight => midnight::Slider.into(),
        }
    }
}

impl From<Theme> for Box<dyn progress_bar::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => light::ProgressBar.into(),
            Theme::Dark => dark::ProgressBar.into(),
            Theme::Midnight => midnight::ProgressBar.into(),
        }
    }
}

impl From<Theme> for Box<dyn checkbox::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => light::Checkbox.into(),
            Theme::Dark => dark::Checkbox.into(),
            Theme::Midnight => midnight::Checkbox.into(),
        }
    }
}

impl From<Theme> for Box<dyn toggler::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => light::Toggler.into(),
            Theme::Dark => dark::Toggler.into(),
            Theme::Midnight => midnight::Toggler.into(),
        }
    }
}

impl From<Theme> for Box<dyn rule::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => light::Rule.into(),
            Theme::Dark => dark::Rule.into(),
            Theme::Midnight => midnight::Rule.into(),
        }
    }
}