#[macro_use]
extern crate glium;

mod array_2d;

fn handle_keyboard_event(state: glium::glutin::ElementState, key: Option<glium::glutin::VirtualKeyCode>, keys: &mut [bool]) {
    let is_down = if state == glium::glutin::ElementState::Released { false } else { true };
    match key {
        Some(glium::glutin::VirtualKeyCode::Up) => keys[0] = is_down,
        Some(glium::glutin::VirtualKeyCode::Down) => keys[1] = is_down,
        Some(glium::glutin::VirtualKeyCode::Left) => keys[2] = is_down,
        Some(glium::glutin::VirtualKeyCode::Right) => keys[3] = is_down,
        _ => return
    }
}

fn main() {
    use glium::{DisplayBuild, Surface};
    use array_2d::Array2d;

    let display = glium::glutin::WindowBuilder::new()
         .with_dimensions(800, 800)
         .with_title(format!("Experiment"))
         .build_glium()
         .unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2]
    }
    
    implement_vertex!(Vertex, position);


    let vertex1 = Vertex { position: [0.05, -0.05] };
    let vertex2 = Vertex { position: [0.05, 0.05] };
    let vertex3 = Vertex { position: [-0.05, -0.05] };
    let vertex4 = Vertex { position: [-0.05, 0.05] };

    let shape = vec![vertex1, vertex2, vertex3, vertex4];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
    
    let vertex_shader_src = r#"
        #version 130
        in vec2 position;

        uniform float posX;
        uniform float posY;
        
        void main() {
            vec2 pos = position;
            pos.x += posX;
            pos.y += posY;
            gl_Position = vec4(pos, 0.0, 1.0);
        }
    "#;    
    
    let fragment_shader_src = r#"
        #version 130
        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;
    
    let mut pos_x: f32 = 0.0;
    let mut pos_y: f32 = 0.0;
    let mut keys = [false, false, false, false];
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    let mut map: Array2d = Array2d::new(1000, 1000);
    
    map.set_element_at(32, 32, 1);
    map.set_element_at(128, 128, 1);

    loop {
        let mut target = display.draw();
        // Draw the cells
        let mut cellVec = Vec<i32>::new();

        for x in 0..map.width() {
            for y in 0..map.height() {
                if map.element_at(x, y) == 1 {
                    let vertex1 = Vertex { position: [0.05, -0.05] };
                    let vertex2 = Vertex { position: [0.05, 0.05] };
                    let vertex3 = Vertex { position: [-0.05, -0.05] };
                    let vertex4 = Vertex { position: [-0.05, 0.05] };

    let shape = vec![vertex1, vertex2, vertex3, vertex4];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap(); 
                    target.draw(&)
                }
            }
        }

        // Draw the red square
        if keys[0] { pos_y += 0.01 }
        if keys[1] { pos_y -= 0.01 }
        if keys[2] { pos_x -= 0.01 }
        if keys[3] { pos_x += 0.01 }
        target.clear_color(100.0, 100.0, 100.0, 1.0);

        target.draw(&vertex_buffer, &indices, &program, &uniform!{ posX: pos_x, posY: pos_y }, &Default::default()).unwrap();
        target.finish().unwrap();
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::KeyboardInput(state, _, key) => handle_keyboard_event(state, key, &mut keys),
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}