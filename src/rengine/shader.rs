extern crate gl;

use std;
use self::gl::types::{GLint, GLuint};

pub struct Shader {
    id: GLuint
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id) };
    }
}

impl Shader {
    pub fn from_source(source: &str, shader_type: GLuint) -> Result<Shader, String> {
        let shader = Shader {
            id: unsafe { gl::CreateShader(shader_type) }
        };
        
        unsafe {
            let ptr: *const u8 = source.as_bytes().as_ptr();
            let ptr_i8: *const i8 = std::mem::transmute(ptr);
            let len = source.len() as GLint;
            gl::ShaderSource(shader.id, 1, &ptr_i8, &len);
        }
        
        let successful = unsafe {
            gl::CompileShader(shader.id);
            
            let mut result: GLint = 0;
            gl::GetShaderiv(shader.id, gl::COMPILE_STATUS, &mut result);
            result != 0
        };
        
        if successful {
            Ok(shader)
        } else {
            Err(shader.get_compilation_log())
        }
    }
    
    fn get_compilation_log(&self) -> String {
        let mut len = 0;
        unsafe { gl::GetShaderiv(self.id, gl::INFO_LOG_LENGTH, &mut len) };
        assert!(len > 0);
        
        let mut buf = Vec::with_capacity(len as usize);
        let buf_ptr = buf.as_mut_ptr() as *mut gl::types::GLchar;
        unsafe {
            gl::GetShaderInfoLog(self.id, len, std::ptr::null_mut(), buf_ptr);
            buf.set_len(len as usize);
        }
        
        match String::from_utf8(buf) {
            Ok(log) => log,
            Err(vec) => panic!("Could not convert compilation log from buffer: {}", vec)
        }
    }
}

pub struct ShaderProgram {
    pub name: String,
    pub id: GLuint
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id) };
    }
}

impl ShaderProgram {
    pub fn link(name: String, shaders: &[&Shader]) -> Result<ShaderProgram, String> {
        let program = ShaderProgram {
            name: name,
            id: unsafe { gl::CreateProgram() }
        };
        
        let successful: bool;
        
        unsafe {
            for shader in shaders.iter() {
                gl::AttachShader(program.id, shader.id);
            }
            gl::LinkProgram(program.id);
            
            successful = {
                let mut result: GLint = 0;
                gl::GetProgramiv(program.id, gl::LINK_STATUS, &mut result);
                result != 0
            };
        }
        
        if successful {
            Ok(program)
        } else {
            Err(program.get_link_log())
        }
    }
    
    fn get_link_log(&self) -> String {
        let mut len = 0;
        unsafe { gl::GetProgramiv(self.id, gl::INFO_LOG_LENGTH, &mut len) };
        assert!(len > 0);
        
        let mut buf = Vec::with_capacity(len as usize);
        let buf_ptr = buf.as_mut_ptr() as *mut gl::types::GLchar;
        unsafe {
            gl::GetProgramInfoLog(self.id, len, std::ptr::null_mut(), buf_ptr);
            buf.set_len(len as usize);
        };
        
        match String::from_utf8(buf) {
            Ok(log) => log,
            Err(vec) => panic!("Could not convert link log from buffer: {}", vec)
        }
    }
}