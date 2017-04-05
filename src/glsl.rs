#[macro_use]
extern crate glium;
use glium::{DisplayBuild, Surface, Program};
use glium::{glutin, index, vertex};

#[derive(Copy, Clone)]
struct Vertex {
  pos: [f64; 2]
}

implement_vertex!(Vertex, pos);

fn vertexs(range_y: f64, range_x: f64, density: f64) -> Vec<Vertex> {
  let mut vs = vec!();
  let mut y: f64 = -range_y;

  while y < range_y {
    let mut x: f64 = -range_x;

    while x < range_x {
      vs.push(Vertex { pos: [x,y] });
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
      out vec2 p;

      void main() {
        p = pos;
        gl_Position = vec4(pos, 0.0, 1.0);
      }
    "#, 

    r#"
      #version 400

      in vec2 p;
      out vec4 color;

      uniform double max;
      uniform double scale;
      uniform double center_y;
      uniform double center_x;

      vec2 mandelbrot(double a, double b) {
        double y = 0;
        double x = 0;
        for (double n = 0; n < max; n++) {
          double yy = 2 * x * y + b;
          double xx = x * x - y * y + a;
          y = yy;
          x = xx;

          if (sqrt(y * y + x * x) > 4.0) {
            return vec2(true, n / max);
          }
        }

        return vec2(false, max / max);
      }

      void main() {
        double x = center_x + p.x * scale;
        double y = center_y + p.y * scale;
        vec2 m = mandelbrot(x, y);
        if (bool(m.x)) { 
          color = vec4(m.y, m.y, m.y, m.y);
        }
        else {
          color = vec4(0.0, 0.0, 0.0, 0.0);
        }
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

  let mut density: f64 = 0.01;
  let mut max: f64 = 100.0;
  let mut scale: f64 = 3.0;
  let mut center_y: f64 = 0.0;
  let mut center_x: f64 = 0.0;

  loop {
    let vertex_buffer = vertex::VertexBuffer::new(
      &display, &vertexs(1.0, 1.0, density)
    ).unwrap();

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

    let uniforms = uniform! {
      max: max,
      scale: scale,
      center_y: center_y,
      center_x: center_x,
    };
    
    println!("{} {} {} {} {}", center_y, center_x, scale, max, density);

    frame.draw(&vertex_buffer, &index, &program, &uniforms, &Default::default()).unwrap();
    frame.finish().unwrap();
  }
}
