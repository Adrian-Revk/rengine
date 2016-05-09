extern crate glfw;
extern crate gl;

use std::ptr;
use std;
use std::ffi::CString;
use self::gl::types::{GLboolean, GLint, GLuint, GLchar, GLfloat, GLsizeiptr};
use self::glfw::{Action, Context, CursorMode, Key, Window};
use std::cell::Cell;
use std::sync::mpsc::Receiver;

pub mod shader;

pub struct ReDevice {
    context: glfw::Glfw,
    window: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,

    test_program: shader::ShaderProgram,
}

impl ReDevice {
    pub fn new() -> ReDevice {
        let glfw_context = glfw::init(Some(glfw::Callback {
                               f: error_callback as fn(glfw::Error, String, &Cell<usize>),
                               data: Cell::new(0),
                           }))
                               .unwrap();

        // glfw_context.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        let (mut window, events) = match glfw_context.create_window(300,
                                                                    300,
                                                                    "Rul Engine",
                                                                    glfw::WindowMode::Windowed) {
            None => panic!("Initialization failed."),
            Some(result) => result,
        };

        window.set_all_polling(true);
        window.make_current();
        // window.set_cursor_mode(CursorMode::Disabled);

        println!("GLFW version: {}", glfw::get_version_string());
        println!("Context version: {}", window.get_context_version());

        unsafe {
            gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
            gl::ClearColor(0.4, 0.3, 0.6, 1.0);
        }

        // Init random stuff.
        // TODO : move this in test place (with init + runtime callbacks)
        let vs = match shader::Shader::from_source(VS_SRC, gl::VERTEX_SHADER) {
            Ok(vs) => vs,
            Err(err) => panic!("vs not working : {}", err),
        };
        let fs = match shader::Shader::from_source(FS_SRC, gl::FRAGMENT_SHADER) {
            Ok(fs) => fs,
            Err(err) => panic!("fs not working : {}", err),
        };



        let program = match shader::ShaderProgram::link(String::from("p1"), &[&vs, &fs]) {
            Ok(p) => p,
            Err(err) => panic!("fs not working : {}", err),
        };

        let mut vao = 0;
        let mut vbo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER,
                           (VERTEX_DATA.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                           std::mem::transmute(&VERTEX_DATA[0]),
                           gl::STATIC_DRAW);

            gl::UseProgram(program.id);
            gl::BindFragDataLocation(program.id, 0, CString::new("out_color").unwrap().as_ptr());

            let pos_attr = gl::GetAttribLocation(program.id,
                                                 CString::new("position").unwrap().as_ptr());
            gl::EnableVertexAttribArray(pos_attr as GLuint);
            gl::VertexAttribPointer(pos_attr as GLuint,
                                    2,
                                    gl::FLOAT,
                                    gl::FALSE as GLboolean,
                                    0,
                                    ptr::null());
        }

        ReDevice {
            context: glfw_context,
            window: window,
            events: events,
            test_program: program,
        }
    }

    pub fn main_loop(&mut self) {
        while !self.window.should_close() {
            self.context.poll_events();
            for event in glfw::flush_messages(&self.events) {
                handle_window_event(&mut self.window, event);
            }

            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
                
                gl::DrawArrays(gl::TRIANGLES, 0, 3);
            }
            self.window.swap_buffers();
        }
    }
}
static VERTEX_DATA: [GLfloat; 6] = [0.0, 0.5, 0.5, -0.5, -0.5, -0.5];
static VS_SRC: &'static str = "#version 150\nin vec2 position;\nvoid main() {\ngl_Position = \
                               vec4(position, 0.0, 1.0);\n}";

static FS_SRC: &'static str = "#version 150\nout vec4 out_color;\nvoid main() {\nout_color = \
                               vec4(1.0, 1.0, 1.0, 1.0);\n}";

fn error_callback(_: glfw::Error, desc: String, error_count: &Cell<usize>) {
    panic!("GLFW error : {:?}.", desc);
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