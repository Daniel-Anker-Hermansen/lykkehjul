use glium::{glutin::{self, event_loop::{EventLoop, EventLoopWindowTarget, ControlFlow}, window::{WindowBuilder, Fullscreen}, dpi::LogicalSize, event::{Event, DeviceEvent, ElementState, WindowEvent, KeyboardInput, VirtualKeyCode}, platform::windows::EventLoopExtWindows}};

use std::sync::{Arc, Mutex};

mod wheel;

mod text;

pub struct Controller {
    display_events: Vec<DisplayEvent>
}

impl Controller {
    fn new() -> Self {
        Controller { display_events:  vec![] }
    }
}

#[derive(Debug, PartialEq)]
enum DisplayEvent {
    Spin,
    ToggleFullscreen,
}

pub fn run(data: Vec<(String, u64)>) {
    let events_loop = EventLoop::<()>::new_any_thread();
    let wb = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(1920,1080))
        .with_fullscreen(Some(Fullscreen::Borderless(None)))
        .with_title("Lykkehjul");
    let windowed_context = glutin::ContextBuilder::new()
        .build_windowed(wb, &events_loop)
        .unwrap();
    let controller = Arc::new(Mutex::new(Controller::new()));
    let sub_thread_controller = controller.clone();
    std::thread::spawn(move || wheel::new_wheel(&data[..], windowed_context, sub_thread_controller));
    events_loop.run(event_handler(controller));
}

fn event_handler<T: std::fmt::Debug>(controller: Arc<Mutex<Controller>>) -> Box<dyn FnMut(Event<'_, T>, &EventLoopWindowTarget<T>, &mut ControlFlow) -> ()> {
    Box::new(move |event: Event<'_, _>, _event_loop: &EventLoopWindowTarget<_>, control: &mut ControlFlow|{
        *control = ControlFlow::Wait;
        match event {
        Event::DeviceEvent { device_id: _, event: e } => match e {
            DeviceEvent::Button { button: 1, state: ElementState::Released } => {
                let mut controller_guard = controller.lock().unwrap();
                if !controller_guard.display_events.contains(&DisplayEvent::Spin) {
                    controller_guard.display_events.push(DisplayEvent::Spin);
                }
            }
            DeviceEvent::Key(KeyboardInput{virtual_keycode, ..}) => match virtual_keycode {
                Some(VirtualKeyCode::Escape) => *control = ControlFlow::Exit,
                Some(VirtualKeyCode::F11) => {
                    let mut controller_guard = controller.lock().unwrap();
                    controller_guard.display_events.push(DisplayEvent::ToggleFullscreen);
                }
                _ => ()
            },
            _ => ()
        }
        Event::WindowEvent { window_id: _, event: WindowEvent::CloseRequested } => std::process::exit(0),
        _ => ()
        };
    })
}