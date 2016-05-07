#![allow(dead_code)]
#![allow(unused_variables)]

extern crate glfw;
#[macro_use]
extern crate log;

use glfw::{Action, Context, Key};
use std::cell::Cell;

use log::{LogRecord, LogLevel, LogLevelFilter, LogMetadata};

struct RELogger;

impl log::Log for RELogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }
}


fn main() {
    // Init logger
    let relogger = log::set_logger(|max_log_level| {
        max_log_level.set(LogLevelFilter::Info);
        Box::new(RELogger)
    });

    let mut glfw_context = glfw::init(Some(glfw::Callback {
                               f: error_callback as fn(glfw::Error, String, &Cell<usize>),
                               data: Cell::new(0),
                           }))
                               .unwrap();

    info!("starting up");
    glfw_context.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    let (mut window, events) = match glfw_context.create_window(300, 300, "Rul Engine", glfw::WindowMode::Windowed) {
        None => panic!("Initialization failed."),
        Some(result) => result
    };

    window.set_all_polling(true);
    window.set_key_polling(true);
    window.make_current();

    println!("GLFW version: {}", glfw::get_version_string());

    let render_context = window.render_context();

    while !window.should_close() {
        glfw_context.poll_events();
        for event in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }
}

fn error_callback(_: glfw::Error, desc: String, error_count: &Cell<usize>) {
    error!("GLFW error {:?}: {:?}", error_count.get(), desc);
    error_count.set(error_count.get() + 1);
}

fn handle_window_event(window: &mut glfw::Window, (time, event): (f64, glfw::WindowEvent)) {
    match event {
        glfw::WindowEvent::Pos(x, y) => {
            window.set_title(&format!("Time: {:?}, Window pos: ({:?}, {:?})", time, x, y))
        }
        glfw::WindowEvent::Size(w, h) => {
            window.set_title(&format!("Time: {:?}, Window size: ({:?}, {:?})", time, w, h))
        }
        glfw::WindowEvent::Close => println!("Time: {:?}, Window close requested.", time),
        glfw::WindowEvent::Refresh => println!("Time: {:?}, Window refresh triggered.", time),
        glfw::WindowEvent::Focus(true) => println!("Time: {:?}, Window focus gained.", time),
        glfw::WindowEvent::Focus(false) => println!("Time: {:?}, Window focus lost.", time),
        glfw::WindowEvent::Iconify(true) => println!("Time: {:?}, Window was minimized.", time),
        glfw::WindowEvent::Iconify(false) => println!("Time: {:?}, Window was maximized.", time),
        glfw::WindowEvent::FramebufferSize(w, h) => {
            println!("Time: {:?}, Framebuffer size: ({:?}, {:?}).", time, w, h)
        }
        glfw::WindowEvent::Char(character) => {
            println!("Time: {:?}, Character: {:?}", time, character)
        }
        glfw::WindowEvent::MouseButton(btn, action, mods) => {
            println!("Time: {:?}, Button: {:?}l Action: {:?}, Modifiers: [{:?}].",
                     time,
                     glfw::DebugAliases(btn),
                     action,
                     mods)
        }
        glfw::WindowEvent::CursorPos(xpos, ypos) => {
            println!("Time: {:?}, Cursor position: ({:?}, {:?})",
                     time,
                     xpos,
                     ypos)
        }
        glfw::WindowEvent::CursorEnter(true) => {
            println!("Time: {:?}, Cursor entered window.", time)
        }
        glfw::WindowEvent::CursorEnter(false) => println!("Time: {:?}, Cursor left window.", time),
        glfw::WindowEvent::Scroll(x, y) => {
            println!("Time: {:?}, Scroll offset: ({:?}, {:?})", time, x, y)
        }

        glfw::WindowEvent::Key(key, scancode, action, mods) => {
            match (key, action) {
                (Key::Escape, Action::Release) => window.set_should_close(true),
                (Key::R, Action::Release) => {
                    let (window_w, window_h) = window.get_size();
                    window.set_size(window_w + 1, window_h);
                    window.set_size(window_w, window_h);
                }
                _ => {}
            }
        }
    }
}
