mod fill;

use structopt::StructOpt;
use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::ControlFlow,
    keyboard::{Key, NamedKey},
    window::Fullscreen,
};

fn run_glium(monitor: usize) -> Result<(), impl std::error::Error> {
    // инициализировать окно в полноэкранном режиме с размерами dimensions

    // 1. The **winit::EventLoop** for handling events.
    let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
    // 2. Create a glutin context and glium Display
    let (window, _display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    // 3. Установить полноэкранный режим
    let wh = window.available_monitors().skip(monitor).next().unwrap();
    let fs = Fullscreen::Borderless(Some(wh));
    window.set_fullscreen(Some(fs));

    // 3. Цвет по умолчанию
    let mut color = 0x101010;

    // 5.  Запуск цикла отрисовки
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
                        color = ((((color >> 8) as u8).saturating_add(1) as u32) << 8)
                            | (color & 0xFF00FF);
                        println!("#{:06X}", color);
                        fill::fill_window_with_color(&window, color);
                    }
                    Key::Character("h") => {
                        color = ((((color >> 8) as u8).saturating_sub(1) as u32) << 8)
                            | (color & 0xFF00FF);
                        println!("#{:06X}", color);
                        fill::fill_window_with_color(&window, color);
                    }
                    Key::Character("r") => {
                        color = ((((color >> 16) as u8).saturating_add(1) as u32) << 16)
                            | (color & 0x00FFFF);
                        println!("#{:06X}", color);
                        fill::fill_window_with_color(&window, color);
                    }
                    Key::Character("t") => {
                        color = ((((color >> 16) as u8).saturating_sub(1) as u32) << 16)
                            | (color & 0x00FFFF);
                        println!("#{:06X}", color);
                        fill::fill_window_with_color(&window, color);
                    }
                    Key::Character("b") => {
                        color = ((((color >> 0) as u8).saturating_add(1) as u32) << 0)
                            | (color & 0xFFFF00);
                        println!("#{:06X}", color);
                        fill::fill_window_with_color(&window, color);
                    }
                    Key::Character("n") => {
                        color = ((((color >> 0) as u8).saturating_sub(1) as u32) << 0)
                            | (color & 0xFFFF00);
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
    /// monitor id
    #[structopt(long, default_value = "0")]
    monitor: usize,
}

fn main() -> Result<(), impl std::error::Error> {
    let args = Cli::from_args();

    // run glium
    run_glium(args.monitor)
}
