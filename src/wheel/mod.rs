use font_kit::font::Font;
use glium::{Display, Surface, glutin::{ContextWrapper, ContextCurrentState, window::Window}, Rect, implement_vertex};
use rand::Rng;
use std::{f32::consts::PI, sync::{Mutex, Arc}, time::Instant};

use super::{DisplayEvent, Controller};

mod pie;

struct Wheel {
    pies: Vec<(String, Color, f32, f32)>,
    display: Display,
    font: Font
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

type Color = (f32, f32, f32, f32);

impl Wheel {
    fn init(people: &[(String, u64)], display: Display) -> Self {
        let aggr: u64 = people.iter().map(|(_, weight)| weight).sum();
        let mut acc = 0.0;
        let mut acc1 = 0;
        let mut rng = rand::thread_rng();
        let wheel = Wheel {
            pies: people.iter().map(|(name, weight)|{
                let angle = *weight as f32 / aggr as f32;
                let phi1 = acc;
                acc += angle * 2.0 * PI;
                acc1 += weight;
                let mut phi2 = acc % (2.0 * PI);
                if acc1 == aggr {
                    phi2 = 0.0;
                }
                (name.to_string(), (rng.gen(), rng.gen(), rng.gen(), 0.0), phi1, phi2)
            }).collect::<Vec<_>>(),
            display,
            font: super::text::load_font()
        };
        wheel.draw();
        wheel
    }

    fn spin(self: &mut Self) {
        let mut speed = 5.0;
        let time = (rand::thread_rng().gen::<f32>() * 1000.0) as u128;
        let now = Instant::now();
        loop {
            self.pies = self.pies.clone().into_iter().map(|(name, color, phi1, phi2)|
                    (name, 
                        color, 
                        (phi1 - PI / (30.0 / speed) + 2.0 * PI) % (2.0 * PI),
                        (phi2 - PI / (30.0 / speed) + 2.0 * PI) % (2.0 * PI)))
                .collect::<Vec<_>>();
            self.draw();
            if now.elapsed().as_millis() > time {
                speed -= 0.017;
            }
            if speed < 0.0 {
                break;
            }
            std::thread::sleep(std::time::Duration::new(1, 0) / 60);
        }
        let winner = &self.pies.iter().find(|(_, _, phi1, phi2)| phi2 >= &0.0 && phi1 > phi2).unwrap().0;
        println!("{}", winner);
    }

    fn draw(self: &Self) {
        let mut frame = self.display.draw();
        let physical_size = self.display.gl_window().window().inner_size();
        let width = physical_size.width;
        let height = physical_size.height;
        frame.clear_color(0.0, 0.0, 0.0, 0.0);
        for (_, color, phi1, phi2) in &self.pies {
            pie::draw_pie_to_frame(&mut frame, height / 2, Some(*color), width / 2, height / 2, (*phi1, *phi2));
        }
        let vec = self.pies.iter().map(|(name, _, phi1, phi2)|{
            if phi2 < phi1 {
                return (name, (phi1 + phi2) / 2.0 + PI);
            }
            (name,(phi1 + phi2) / 2.0)}).collect();
        let size: i64 = height as i64 / 20;
        for y in 1 - size..=size - 1 {
            let abs = y.abs();
            let line_width = size - abs;
            frame.clear(Some(&Rect {
                left: ((width + height) as i64 / 2 + 1 + abs) as u32,
                bottom: (height as i64 / 2 + y) as u32,
                width: line_width as u32,
                height: 1,
            }), Some((1.0, 1.0, 1.0, 0.0)), false, None, None);
        }
        let width2 = width;
        let (text, width) = super::text::test(vec, &self.font, height as i32 / 2);
        let height = text.len() / width;
        for y in 0..height {
            for x in 0..width {
                let data = text[y * width + x];
                if data == 0 {
                    continue;
                }
                let greyscale = 1.0 - (data as f32) / 255.0;
                frame.clear(Some(&Rect {
                    left: x as u32 + width2 / 2 - width as u32 / 2,
                    bottom: (height - y) as u32,
                    width: 1,
                    height: 1,
                }), Some((greyscale, greyscale, greyscale, 0.0)), false, None, None);
            }
        }
        frame.finish().expect("I did bad code");
    }
}

pub fn new_wheel<T: ContextCurrentState>(people: &[(String, u64)], windowed_context: ContextWrapper<T, Window>, controller: Arc<Mutex<Controller>>) {
    let display = Display::from_gl_window(windowed_context).unwrap();
    let mut wheel = Wheel::init(people, display);
    loop {
        let controller_guard = controller.lock().unwrap();
        let event = controller_guard.display_events.last().clone();
        match event {
            Some(DisplayEvent::Spin) => {
                drop(controller_guard);
                wheel.spin();
                let mut controller_guard = controller.lock().unwrap();
                controller_guard.display_events.pop();
            }
            None => ()
        };
    }
}
