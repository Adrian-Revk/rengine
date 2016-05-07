#![allow(dead_code)]
#![allow(unused_variables)]

extern crate glfw;

use glfw::{Action, Context, Key};

fn main() {
    let mut glfw_context = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw_context.create_window(300,
                                                          300,
                                                          "Rul Engine",
                                                          glfw::WindowMode::Windowed)
                                           .expect("Failed to create GLFW Window.");

    window.set_key_polling(true);
    window.make_current();

    while !window.should_close() {
        glfw_context.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}
