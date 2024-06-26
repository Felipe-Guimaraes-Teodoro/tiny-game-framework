use glfw::{self, Action, Key};
use crate::{cstr, graphics::shader::Shader};
use crate::glam::{vec3, Vec3, Mat4};
use std::ffi::CString;

const UP: Vec3 = Vec3::Y;
const SENSITIVITY: f32 = 0.1; // todo: make this editable

pub enum ProjectionType {
    Perspective,
    Orthographic,
}

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub proj: Mat4,
    pub view: Mat4,

    pub pos: Vec3,
    target: Vec3,
    direction: Vec3,
    pub right: Vec3,
    pub front: Vec3,
    pub up: Vec3,

    pub pitch: f32,
    pub yaw: f32,

    pub speed: f32,

    pub dt: f32,
    last_frame: f32,

    first_mouse: bool,
    last_x: f32,
    last_y: f32,
}

impl Camera {
    pub fn new() -> Self {
        let (pitch, yaw): (f32, f32) = (0.0, -90.0);
        let pos = vec3(0.0, 0.0, 3.0);
        let target = vec3(0.0, 0.0, -1.0);
        let mut direction = (pos - target).normalize();
        direction.x = yaw.to_radians().cos() * pitch.to_radians().cos();
        direction.y = pitch.to_radians().sin();
        direction.z = yaw.to_radians().sin() * pitch.to_radians().cos();
        
        let right = UP.cross(direction).normalize();
        let up = direction.cross(right);
        let front = direction.normalize();

        let view = Mat4::look_at_rh(pos, pos + front, up);

        Self {
            proj: Mat4::perspective_rh_gl(70.0f32.to_radians(), 1.0, 0.1, 100000.0),
            view,

            pos,
            target,
            direction,
            right,
            front,
            up,

            speed: 1.0,

            pitch,
            yaw,

            dt: 0.0,
            last_frame: 0.0,

            first_mouse: true,
            last_x: 400.0,
            last_y: 400.0,
        }
    }

    pub fn update(&mut self, y: Vec3) {
        self.pos = y;
        
        self.view = Mat4::look_at_rh(
            self.pos,
            self.pos + self.front,
            self.up,
        );
    }

    pub fn input(
        &mut self,
        window: &glfw::Window, 
        glfw: &glfw::Glfw,
    ) {
        let mut speed = self.speed;
        let curr_frame = glfw.get_time() as f32;
        self.dt = curr_frame - self.last_frame;
        self.last_frame = curr_frame;

        if window.get_key(Key::LeftShift) == Action::Press {
            speed *= 20.0;
        }
        
        if window.get_key(Key::RightShift) == Action::Press {
            speed *= 20.0;
        }

        if window.get_key(Key::W) == Action::Press {
            self.pos += speed * self.dt * self.front; 
        }
        if window.get_key(Key::S) == Action::Press {
            self.pos -= speed * self.dt * self.front; 
        }
        if window.get_key(Key::Space) == Action::Press {
            self.pos += speed * self.dt * self.up;
        }
        if window.get_key(Key::LeftControl) == Action::Press {
            self.pos -= speed * self.dt * self.up;
        }
        if window.get_key(Key::A) == Action::Press {
            self.pos -= speed * self.dt * self.front.cross(self.up).normalize(); 
        }
        if window.get_key(Key::D) == Action::Press {
            self.pos += speed * self.dt * self.front.cross(self.up).normalize(); 
        }

        let (w, h) = window.get_framebuffer_size();
        self.proj = Mat4::perspective_rh_gl(70.0f32.to_radians(), w as f32 / h as f32, 0.0001, 1000.0);
    }

    pub fn mouse_callback(
        &mut self, 
        xpos: f32, 
        ypos: f32,
        window: &glfw::Window,
    ) {
        if window.get_cursor_mode() != glfw::CursorMode::Disabled {
            self.first_mouse = true;
            // return 
        };
        if self.first_mouse { 
            self.last_x = xpos;
            self.last_y = ypos;
            self.first_mouse = false;
        }

        let mut xoffs = xpos - self.last_x;
        let mut yoffs = self.last_y - ypos;

        self.last_x = xpos;
        self.last_y = ypos;

        xoffs *= SENSITIVITY;
        yoffs *= -SENSITIVITY;

        self.yaw += xoffs;
        self.pitch += yoffs;

        self.direction.x = self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
        self.direction.y = self.pitch.to_radians().sin();
        self.direction.z = self.yaw.to_radians().sin() * self.pitch.to_radians().cos();

        self.front = self.direction.normalize();
    }

    // RENDERING //
    pub unsafe fn send_uniforms(&self, shader: &Shader) {
        shader.uniform_mat4fv(
            cstr!("view"),
            &self.view.to_cols_array(),
        );

        shader.uniform_mat4fv(
            cstr!("proj"),
            &self.proj.to_cols_array(),
        );
    }

    pub fn set_projection(
        &mut self, 
        projection_type: ProjectionType,
    ) {
        match projection_type {
            ProjectionType::Perspective => {
                self.proj = Mat4::perspective_rh_gl(70.0f32.to_radians(), 1.0, 0.1, 10000.0);
            },
            ProjectionType::Orthographic => {
                self.proj = Mat4::orthographic_rh(-1.0, 1.0, -1.0, 1.0, -100.0, 100.0);
            }
        }
    }
 
 }
