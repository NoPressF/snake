use crossterm::style::Color;

#[derive(Clone, Copy, PartialEq)]
pub struct Vector2D<T> {
    pub x: T,
    pub y: T,
}

pub struct Utils;
impl Utils {
    pub(crate) fn lerp_rgb_color(t: f32, first_color: Color, second_color: Color) -> Color {
        let t = t.max(0.0).min(1.0);

        let (r1, g1, b1) = match first_color {
            Color::Rgb { r, g, b } => (r as f32, g as f32, b as f32),
            _ => panic!("First color must be an RGB color"),
        };

        let (r2, g2, b2) = match second_color {
            Color::Rgb { r, g, b } => (r as f32, g as f32, b as f32),
            _ => panic!("Second color must be an RGB color"),
        };

        let r = (r1 + t * (r2 - r1)) as u8;
        let g = (g1 + t * (g2 - g1)) as u8;
        let b = (b1 + t * (b2 - b1)) as u8;

        let r = r.max(0).min(255);
        let g = g.max(0).min(255);
        let b = b.max(0).min(255);

        Color::Rgb { r, g, b }
    }
}
