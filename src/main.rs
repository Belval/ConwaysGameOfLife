#[macro_use]
extern crate glium;

use std::time::Duration;
use std::thread;

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
    let mut map: Array2d = Array2d::new(50, 50);
    
    map.set_random_elements(60, 1);

    loop {
        let mut target = display.draw();
        let mut new_map: Array2d = Array2d::new(200, 200);
        
        // Draw the red square
        if keys[0] { pos_y += 0.01 }
        if keys[1] { pos_y -= 0.01 }
        if keys[2] { pos_x -= 0.01 }
        if keys[3] { pos_x += 0.01 }
        target.clear_color(100.0, 100.0, 100.0, 1.0);

        for x in 0..map.width() {
            for y in 0..map.height() {
                let count_surrounding = map.get_surrounding_count(x, y);
                let mut is_alive = map.element_at(x, y) == 1;
                if (count_surrounding < 2 || count_surrounding > 3) && is_alive {
                    new_map.set_element_at(x, y, 0);
                    is_alive = false;
                } else if count_surrounding == 3 {
                    new_map.set_element_at(x, y, 1);
                    is_alive = true;
                }
                if is_alive {
                    new_map.set_element_at(x, y, 1);
                    let cell_pos_x = x as f32 / 25.0 - 1.;
                    let cell_pos_y = y as f32 / 25.0 - 1.;
                    let cell_vertex1 = Vertex { position: [cell_pos_x - pos_x, cell_pos_y - pos_y - 0.025] };
                    let cell_vertex2 = Vertex { position: [cell_pos_x - pos_x, cell_pos_y - pos_y] };
                    let cell_vertex3 = Vertex { position: [cell_pos_x - pos_x - 0.025, cell_pos_y - pos_y - 0.025] };
                    let cell_vertex4 = Vertex { position: [cell_pos_x - pos_x - 0.025, cell_pos_y - pos_y] };
                    let cell_shape = vec![cell_vertex1, cell_vertex2, cell_vertex3, cell_vertex4];
                    let cell_vertex_buffer = glium::VertexBuffer::new(&display, &cell_shape).unwrap(); 
                    target.draw(&cell_vertex_buffer, &indices, &program, &uniform!{ }, &Default::default()).unwrap();
                }
            }
        }

        map = new_map;

        target.draw(&vertex_buffer, &indices, &program, &uniform!{ posX: pos_x, posY: pos_y }, &Default::default()).unwrap();
        target.finish().unwrap();
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::KeyboardInput(state, _, key) => handle_keyboard_event(state, key, &mut keys),
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    thread::sleep(Duration::from_millis(500))
    }
}
