use std::f32::consts::PI;

use glium::{implement_vertex, Frame, Surface, Display};

#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

// TEMPORARY
const WIDTH: usize = 1920 / 2;
const HEIGHT: usize = 1080 / 2;

pub fn draw_pie_to_frame(display: &Display, frame: &mut Frame, radius: u32, color: (f32, f32, f32, f32), mut cut: (f32, f32)) {
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = &format!(r#"
        #version 140

        out vec4 color;

        void main() {{
            color = vec4({}, {}, {}, {});
        }}
    "#, color.0, color.1, color.2, color.3);

    let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();
    if cut.1 < cut.0 {
        cut.1 += 2.0 * PI;
    }
    let diff = cut.1 - cut.0;
    let no = (20.0 * diff) as usize;
    let inc = diff / no as f32;
    let vertices: Vec<_> = (0..no).flat_map(|i| {
        let phi1 = cut.0 + inc * i as f32;
        let phi2 = cut.0 + inc * (i + 1) as f32;
        let x2 = phi1.cos() * radius as f32 / WIDTH as f32;
        let y2 = phi1.sin() * radius as f32 / HEIGHT as f32;
        let x3 = phi2.cos() * radius as f32 / WIDTH as f32;
        let y3 = phi2.sin() * radius as f32 / HEIGHT as f32;
        let v1 = Vertex{ position: [0.0, 0.0] };
        let v2 = Vertex{ position: [x2, y2] };
        let v3 = Vertex{ position: [x3, y3] };
        [v1, v2, v3]
    }).collect();
    let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    frame.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
        &Default::default()).unwrap();
}

pub fn draw_triangle(display: &Display, frame: &mut Frame, size: u32, radius: u32) {
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = &format!(r#"
        #version 140

        out vec4 color;

        void main() {{
            color = vec4({}, {}, {}, {});
        }}
    "#, 1.0, 1.0, 1.0, 1.0);

    let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();
    let left = radius + 1;

    let v1 = Vertex{ position: [left as f32 / WIDTH as f32, 0.0] };
    let v2 = Vertex{ position: [(left as f32 + size as f32) / WIDTH as f32, size as f32 / HEIGHT as f32] };
    let v3 = Vertex{ position: [(left as f32 + size as f32) / WIDTH as f32, - (size as f32 / HEIGHT as f32)] };
    let vertices = vec![v1, v2, v3];
    let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    frame.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
        &Default::default()).unwrap();

}