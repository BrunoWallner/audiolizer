use iced::{
    canvas::{self, Cache, Canvas, Cursor, Geometry},
    Color, Element, Length,
    Point, Rectangle, Vector, Size,
};
use crate::Message;

pub struct Bars {
    pub data: Vec<f32>,
    pub cache: Cache,
    pub mirroring: bool,
    pub width: f32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Bars {
    pub fn view<'a>(
        &'a mut self,
    ) -> Element<'a, Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl Default for Bars {
    fn default() -> Self {
        Bars {
            data: Vec::new(),
            cache: Cache::new(),
            mirroring: true,
            width: 10.0,
            r: 255,
            g: 0,
            b: 0,
        }
    }
}

impl canvas::Program<Message> for Bars {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let clock = self.cache.draw(bounds.size(), |frame| {
            let center = frame.center();

            for i in 0..self.data.len() {
                let x: f32 = (frame.width() / self.data.len() as f32) * i as f32;
                let mut y: f32 = frame.height() - self.data[i] * 100.0_f32;
                if y < 0.0 {
                    y = 1.0;
                }
                let size_x: f32 = frame.size().width / self.data.len() as f32 * self.width * 0.1;
                let size_y: f32 = self.data[i] * 100.0_f32;
                frame.fill_rectangle(Point::new(x, y), Size::new(size_x, size_y), Color::from_rgb8(self.r, self.g, self.b));
            }

            frame.translate(Vector::new(center.x, center.y));
        });

        vec![clock]
    }
}
