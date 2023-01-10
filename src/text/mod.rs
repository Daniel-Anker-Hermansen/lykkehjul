use font_kit::canvas::{Canvas, Format, RasterizationOptions};
use font_kit::family_name::FamilyName;
use font_kit::font::Font;
use font_kit::hinting::HintingOptions;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use pathfinder_geometry::transform2d::Transform2F;
use pathfinder_geometry::vector::{Vector2F, Vector2I};

pub fn test(data: Vec<(&String, f32)>, font: &Font, radius: i32) -> (Vec<u8>, usize) {
    let mut canvas = Canvas::new(Vector2I::splat(radius * 2), Format::A8);
    for (str, theta) in data {
        print_string(font, &mut canvas, str, radius as f32, theta);
    }
    (canvas.pixels, canvas.stride)
}

fn print_string(font: &Font, canvas: &mut Canvas, string: &str, radius: f32, theta: f32) {
    let upem = font.metrics().units_per_em;
    let mut vec = vec![];
    let mut offset = 0.0;
    for ch in string.chars() {
        let id = font.glyph_for_char(ch).unwrap();
        let advance = font.advance(id).unwrap();
        vec.push((id, offset));
        offset += (advance.x() / upem as f32) * 32.0;
    }
    let start = radius - offset - 5.0;
    for (id, offset) in vec {
        print_char(font, canvas, id, start + offset, radius, theta);
    }
}

fn print_char(font: &Font, canvas: &mut Canvas, id: u32, offset: f32, radius: f32, theta: f32) {
    let y = theta.cos() * 32.0 / 3.0;
    let x = theta.sin() * 32.0 / 3.0;
    let z = theta.cos() * offset;
    let w = theta.sin() * offset;
    let diameter = radius as usize * 2;
    let mut local_canvas = Canvas::new(Vector2I::splat(diameter as i32), Format::A8);
    let offset_y = radius + y - w;
    let offset_x = radius + x + z;
    font.rasterize_glyph(&mut local_canvas, 
        id, 
        32.0,
        Transform2F::from_rotation(theta).translate(Vector2F::new(offset_x, offset_y)),
        HintingOptions::None, 
        RasterizationOptions::Bilevel)
        .unwrap();
    let offset_y = offset_y as usize;
    let offset_x = offset_x as usize;
    let start_y = offset_y.saturating_sub(32);
    let start_x = offset_x.saturating_sub(32);
    let end_y = diameter.min(offset_y + 32);
    let end_x = diameter.min(offset_x + 32);
    for y in start_y..end_y {
        for x in start_x..end_x {
            let idx = y * diameter + x;
            if local_canvas.pixels[idx] > 0 {
                canvas.pixels[idx] = 255;
            }
        }
    }
}

pub fn load_font() -> Font {
    SystemSource::new().select_best_match(&[FamilyName::SansSerif], 
        &Properties::new())
        .unwrap()
        .load()
        .unwrap()
}