use glium::{Program, Display};

pub fn pie_program(display: &Display) -> Program {
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        uniform vec4 color2;

        out vec4 color3;

        void main() {
            color3 = color2;
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec4 color3;

        out vec4 color;

        void main() {{
            color = color3;
        }}
    "#;

    glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap()
}