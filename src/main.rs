mod fill;

use glium::{
    implement_vertex, index::PrimitiveType, program, texture::RawImage2d, uniform, Display,
    IndexBuffer, Surface, Texture2d, VertexBuffer,
};
use structopt::StructOpt;
use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::ControlFlow,
    keyboard::{Key, NamedKey},
};

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

fn run_glium(dimensions: (u32, u32)) -> Result<(), impl std::error::Error> {
    // инициализировать окнов  полноэкранном режиме с размерами dimensions
    // 1. The **winit::EventLoop** for handling events.
    let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
    // 2. Create a glutin context and glium Display
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    implement_vertex!(Vertex, position, tex_coords);

    // 3. Буфер координат вершин для "экрана"
    let vert_buffer = VertexBuffer::new(
        &display,
        &[
            Vertex {
                position: [-1.0, -1.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [-1.0, 1.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [1.0, 1.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [1.0, -1.0],
                tex_coords: [1.0, 0.0],
            },
        ],
    )
    .unwrap();

    // 4. Буфер индексов вершин, чтобы построить прямоугольник
    let idx_buf =
        IndexBuffer::new(&display, PrimitiveType::TriangleStrip, &[1_u16, 2, 0, 3]).unwrap();

    // 5. Шейдер
    let program = program!(&display,
        140 => {
            vertex: "
            #version 140
            uniform mat4 matrix;
            in vec2 position;
            in vec2 tex_coords;
            out vec2 v_tex_coords;
            void main() {
                gl_Position = matrix * vec4(position, 0.0, 1.0);
                v_tex_coords = tex_coords;
            }
        ",

            fragment: "
            #version 140
            uniform sampler2D tex;
            in vec2 v_tex_coords;
            out vec4 f_color;
            void main() {
                f_color = texture(tex, v_tex_coords);
            }
        "
        },
    )
    .unwrap();

    // 6. Цвет по умолчанию
    let mut color = 0x101010;

    // 7.  Запуск цикла отрисовки
    let mut close_requested = false;
    event_loop.run(move |event, elwt| {
        //println!("{:?}", event);
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    close_requested = true;
                }
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            logical_key: key,
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => match key.as_ref() {
                    Key::Character("g") => {
                        color = (((color >> 8) as u8).saturating_add(1) as u32) << 8;
                        println!("#{:06X}", color);
                        fill::fill_window_with_color(&window, color);
                    }
                    Key::Character("h") => {
                        color = (((color >> 8) as u8).saturating_sub(1) as u32) << 8;
                        println!("#{:06X}", color);
                        fill::fill_window_with_color(&window, color);
                    }
                    Key::Character("r") => {
                        color = (((color >> 16) as u8).saturating_add(1) as u32) << 16;
                        println!("#{:06X}", color);
                        fill::fill_window_with_color(&window, color);
                    }
                    Key::Character("t") => {
                        color = (((color >> 16) as u8).saturating_sub(1) as u32) << 16;
                        println!("#{:06X}", color);
                        fill::fill_window_with_color(&window, color);
                    }
                    Key::Character("b") => {
                        color = (((color >> 0) as u8).saturating_add(1) as u32) << 0;
                        println!("#{:06X}", color);
                        fill::fill_window_with_color(&window, color);
                    }
                    Key::Character("n") => {
                        color = (((color >> 0) as u8).saturating_sub(1) as u32) << 0;
                        println!("#{:06X}", color);
                        fill::fill_window_with_color(&window, color);
                    }
                    Key::Character("a") => {
                        color = ((((color >> 0) as u8).saturating_add(1) as u32) << 0)
                            | ((((color >> 8) as u8).saturating_add(1) as u32) << 8)
                            | ((((color >> 16) as u8).saturating_add(1) as u32) << 16);
                        println!("#{:06X}", color);
                        fill::fill_window_with_color(&window, color);
                    }
                    Key::Character("s") => {
                        color = ((((color >> 0) as u8).saturating_sub(1) as u32) << 0)
                            | ((((color >> 8) as u8).saturating_sub(1) as u32) << 8)
                            | ((((color >> 16) as u8).saturating_sub(1) as u32) << 16);
                        println!("#{:06X}", color);
                        fill::fill_window_with_color(&window, color);
                    }
                    Key::Character("o") => {
                        color = 0x101010;
                        println!("#{:06X}", color);
                        fill::fill_window_with_color(&window, color);
                    }
                    Key::Named(NamedKey::Escape) => {
                        close_requested = true;
                    }
                    _ => (),
                },
                WindowEvent::RedrawRequested => {
                    fill::fill_window_with_color(&window, color);
                }
                _ => (),
            },
            Event::AboutToWait => {
                elwt.set_control_flow(ControlFlow::Wait);

                if close_requested {
                    elwt.exit();
                }
            }
            _ => (),
        }
    })
}

#[derive(Debug, StructOpt)]
/// Green: "G"+1/"H"-1;
/// Reg: "R"+1/"T"-1;
/// Blue: "B"+1/"N"-1;
/// All: "A"+1/"S"-1;
/// "O" - Reset color to #101010
struct Cli {
    /// width
    #[structopt(long, default_value = "1280")]
    width: u32,

    /// heigth
    #[structopt(long, default_value = "1024")]
    heigth: u32,
}

fn main() -> Result<(), impl std::error::Error> {
    let args = Cli::from_args();

    let width = args.width as u32;
    let height = args.heigth as u32;

    // run glium
    run_glium((width, height))
}
