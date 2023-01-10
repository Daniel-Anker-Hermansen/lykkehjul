use font_kit::font::Font;
use glium::{Display, Surface, glutin::{ContextWrapper, ContextCurrentState, window::Window}, Rect, implement_vertex, Program};
use rand::Rng;
use std::{f32::consts::PI, sync::{Mutex, Arc}, time::{Duration, Instant}};

use super::{DisplayEvent, Controller};

mod pie;
mod programs;

struct Wheel {
    pies: Arc<Mutex<Vec<(String, Color, f32, f32)>>>,
    display: Display,
    font: Font,
    pie_program: Program,
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
        let pie_program = programs::pie_program(&display);
        let wheel = Wheel {
            pies: Arc::new(Mutex::new(people.iter().map(|(name, weight)|{
                let angle = *weight as f32 / aggr as f32;
                let phi1 = acc;
                acc += angle * 2.0 * PI;
                acc1 += weight;
                let mut phi2 = acc % (2.0 * PI);
                if acc1 == aggr {
                    phi2 = 0.0;
                }
                (name.to_string(), (rng.gen(), rng.gen(), rng.gen(), 0.0), phi1, phi2)
            }).collect::<Vec<_>>())),
            display,
            font: super::text::load_font(),
            pie_program,
        };
        wheel.draw();
        wheel
    }

    fn spin(self: &mut Self) {
        let running_mutex = Arc::new(Mutex::new(true));
        let thread_running_mutex = running_mutex.clone();
        let mutex = self.pies.clone();
        std::thread::spawn(move ||{
            let rng = (rand::random::<f32>() * 1000.0) as u128;
            let begin = Instant::now();
            let mut speed = 5.0;
            loop {
                let time = Instant::now();
                let mut guard = mutex.lock().unwrap();
                *guard = (*guard).clone().into_iter().map(|(name, color, phi1, phi2)|
                    (name, 
                        color, 
                        (phi1 - PI / (60.0 / speed) + 2.0 * PI) % (2.0 * PI),
                        (phi2 - PI / (60.0 / speed) + 2.0 * PI) % (2.0 * PI)))
                .collect::<Vec<_>>();
                drop(guard);
                if begin.elapsed().as_millis() > rng {
                    speed -= 0.008;
                }
                if speed <= 0.0 {
                    break;  
                }
                let wait = Duration::new(1, 0) / 120 - time.elapsed();
                std::thread::sleep(wait);
            }
            let mut guard = thread_running_mutex.lock().unwrap();
            *guard = false;
        });
        loop {
            self.draw();
            let guard = running_mutex.lock().unwrap();
            if !*guard {
                break;
            }
        }
        let guard = self.pies.lock().unwrap();
        let data = (*guard).clone();
        drop(guard);
        let winner = &data.iter().find(|(_, _, phi1, phi2)| phi2 >= &0.0 && phi1 > phi2).unwrap().0;
        println!("{}", winner);
    }

    fn draw(&self) {
        let mut frame = self.display.draw();
        let physical_size = self.display.gl_window().window().inner_size();
        let width = physical_size.width;
        let height = physical_size.height;
        frame.clear_color(0.0, 0.0, 0.0, 0.0);
        let guard = self.pies.lock().unwrap();
        let data = (*guard).clone();
        drop(guard);
        for (_, color, phi1, phi2) in &data {
            pie::draw_pie_to_frame(&self.pie_program, &self.display, &mut frame, height / 2, *color, (*phi1, *phi2));
        }
        let vec = data.iter().map(|(name, _, phi1, phi2)|{
            if phi2 < phi1 {
                return (name, (phi1 + phi2) / 2.0 + PI);
            }
            (name,(phi1 + phi2) / 2.0)}).collect();
        let size = height / 20;
        pie::draw_triangle(&self.pie_program, &self.display, &mut frame, size, height / 2);
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

    fn toggle_fullscreen(&self) {
        self.display.gl_window().window().set_fullscreen(None);
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
            Some(DisplayEvent::ToggleFullscreen) => {
                drop(controller_guard);
                wheel.toggle_fullscreen();
            }
            None => ()
        };
    }
}
