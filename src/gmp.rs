#[macro_use]
extern crate glium;
extern crate gmp;

use glium::{DisplayBuild, Surface, Program};
use glium::{glutin, index, vertex};
use gmp::mpf::Mpf;

#[derive(Copy, Clone)]
struct Vertex {
  pos: [f64; 2],
  color: [f64; 4]
}

implement_vertex!(Vertex, pos, color);

fn mandelbrot(a: f64, b: f64, max: f64) -> [f64; 4] {
  let mut y: f64 = 0.0;
  let mut x: f64 = 0.0;
  let mut m = 0.0;

  while m < max {
    let yy = 2.0 * x * y + b;
    let xx = x * x - y * y + a;
    y = yy;
    x = xx;

    if (x * x + y * y).sqrt() > 4.0 {
      return [0.0, m / max, 0.0, 1.0];
    }

    m += 1.0;
  }

  return [0.0, 0.0, 0.0, 1.0];
}

fn mandelbrot_set(
  density: f64, scale: f64, center_y: f64, center_x: f64, max: f64, 
) -> Vec<Vertex> {
  let mut vs = vec!();
  let mut y: f64 = -1.0;

  while y < 1.0 {
    let yy = center_y + y * scale;
    let mut x: f64 = -1.0;

    while x < 1.0 {
      let xx = center_x + x * scale;

      vs.push(
        Vertex {
          pos: [x, y],
          color: mandelbrot(xx, yy, max)
        }
      );

      x += density;
    }
    y += density;
  }

  return vs;
}

fn main() {
  let display = glutin::WindowBuilder::new().build_glium().unwrap();

  let program = Program::from_source(
    &display, 
    r#"
      #version 400

      in vec2 pos;
      in vec4 color;
      out vec4 c;

      void main() {
        c = color;
        gl_Position = vec4(pos, 0.0, 1.0);
      }
    "#, 

    r#"
      #version 400

      in vec4 c;
      out vec4 color;

      void main() {
        color = c;
      }
    "#, 

    None
  );

  if let Err(msg) = program {
    println!("{}", msg); 
    return;
  }
  let program = program.unwrap();

  let index = index::NoIndices(index::PrimitiveType::Points);
  let uniforms = uniform!();
  let param = Default::default();

  let mut density: f64 = 0.01;
  let mut scale: f64 = 3.0;
  let mut center_y: f64 = 0.0;
  let mut center_x: f64 = 0.0;
  let mut max: f64 = 100.0;

  loop {
    let mut frame = display.draw();
    frame.clear_color(0.0, 0.0, 0.0 ,0.0);

    for e in display.wait_events() {
      match e {
        glutin::Event::KeyboardInput(
          glutin::ElementState::Pressed, _, Some(keycode)
        ) => {
          println!("{:?}", e);
          match keycode {
            glutin::VirtualKeyCode::Up => {
              center_y += 0.05 * scale; 
            },

            glutin::VirtualKeyCode::Down => {
              center_y -= 0.05 * scale;
            },

            glutin::VirtualKeyCode::Left => {
              center_x -= 0.05 * scale;
            },

            glutin::VirtualKeyCode::Right => {
              center_x += 0.05 * scale;
            },

            glutin::VirtualKeyCode::Return => {
              scale *= 0.9;
            },

            glutin::VirtualKeyCode::Back => {
              scale /= 0.9;
            },

            glutin::VirtualKeyCode::Z => {
              max += 1.0;
            },

            glutin::VirtualKeyCode::X => {
              max -= 1.0;
            },

            glutin::VirtualKeyCode::C => {
              if density > 0.002 {
                density *= 0.9
              }
            },

            glutin::VirtualKeyCode::V => {
              density /= 0.9;
            },

            glutin::VirtualKeyCode::Escape => {
              frame.finish().unwrap();
              return
            },

            _ => {}
          }

        },

        _ => break
      }
    }

    println!("{} {} {} {} {}", center_y, center_x, scale, max, density);

    let vertex_buffer = vertex::VertexBuffer::new(
      &display, &mandelbrot_set(density, scale, center_y, center_x, max)
    ).unwrap();

    frame.draw(&vertex_buffer, &index, &program, &uniforms, &param).unwrap();
    frame.finish().unwrap();
  }
}
