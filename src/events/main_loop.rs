extern crate glfw;   

use std::time::{self, Instant};

use gl::{BlendFunc, Enable, Viewport, BLEND, DEPTH_TEST, ONE_MINUS_SRC_ALPHA, SRC_ALPHA};
use glam::{vec2, Vec2};
use glfw::{fail_on_errors, Glfw, GlfwReceiver, PWindow, Window, WindowEvent};
use glfw::{Action, Context, Key};
use imgui::Ui;

use crate::{Camera, Imgui};

use super::EventHandler;

pub struct EventLoop {
    pub event_handler: EventHandler,
    pub window: PWindow,
    pub ui: Imgui,
    pub glfw: Glfw,
    events: GlfwReceiver<(f64, WindowEvent)>,
    pub now: Instant,
    pub dt: f32,
    pub time: f32,

    pub timescale: f32,
}

impl EventLoop {
    pub fn new(w: u32, h: u32) -> Self {
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();
        
        glfw.window_hint(glfw::WindowHint::TransparentFramebuffer(true));

        let (mut window, events) = glfw.create_window(w, h, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    
        let ui = Imgui::new(&mut window);

        window.make_current();
        window.set_key_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_mouse_button_polling(true);
        window.set_scroll_polling(true);
        // window.set_size_callback(|window: &mut Window, width: i32, height: i32| resize_callback(&*window, width, height));


        gl::load_with(|s| window.get_proc_address(s) );

        unsafe {
            BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
            Enable(BLEND | DEPTH_TEST);
        }
    
        let mut event_handler = EventHandler::new();
        event_handler.on_window_resize(w as i32, h as i32);

        Self {
            event_handler,
            window,
            ui,
            glfw,
            events,
            now: std::time::Instant::now(),
            dt: 0.0,
            time: 0.0,
            timescale: 1.0,
        }
    }

    pub fn size(&self) -> Vec2 {
        vec2(self.window.get_size().0 as f32, self.window.get_size().1 as f32)
    }

    pub fn update(&mut self) {
        self.dt = self.now.elapsed().as_secs_f32() * self.timescale;
        self.time += self.dt;
        self.now = std::time::Instant::now();


        self.window.swap_buffers();
    
        self.glfw.poll_events();

        self.event_handler.update();

        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true)
                },
                glfw::WindowEvent::Key(key, _, Action::Press, _ ) => {
                    self.event_handler.on_key_press(key);
                }
                glfw::WindowEvent::Key(key, _, Action::Release, _ ) => {
                    self.event_handler.on_key_release(key);
                }

                glfw::WindowEvent::CursorPos(x, y) => {
                    self.event_handler.on_mouse_move(x, y);
                    self.ui.on_mouse_move(x as f32, y as f32);
                }

                glfw::WindowEvent::MouseButton(button, Action::Press, _) => {
                    self.ui.on_mouse_click(button, Action::Press);

                    // todo: figure out if the user
                    // is clicking on imgui or the 
                    // actual game
                    match button {
                        glfw::MouseButton::Button1 => {
                            // if self.ui.ctx.io().mouse_down[0] {
                            //     break;
                            // }
                            self.event_handler.on_lmb_press();
                        },
                        glfw::MouseButton::Button2 => {
                            // if self.ui.ctx.io().mouse_down[1] {
                            //     break;
                            // }
                            self.event_handler.on_rmb_press();
                        },
                        _ => ()
                    }
                }

                glfw::WindowEvent::MouseButton(button, Action::Release, _) => {
                    self.ui.on_mouse_click(button, Action::Release);
                    match button {
                        glfw::MouseButton::Button1 => {
                            self.event_handler.on_lmb_release();
                        },
                        glfw::MouseButton::Button2 => {
                            self.event_handler.on_rmb_release();
                        },
                        
                        _ => ()
                    }
                }

                glfw::WindowEvent::Scroll(xoff, yoff) => {
                    self.event_handler.on_scroll_change(vec2(xoff as f32, yoff as f32));
                    self.ui.on_mouse_scroll(xoff as f32, yoff as f32);
                }

                glfw::WindowEvent::FramebufferSize(w, h) => {
                    self.event_handler.on_window_resize(w, h);
                    unsafe {
                        Viewport(0, 0, w, h);
                    }
                }
                _ => {},
            }
        }
    }

    pub fn is_key_down(&mut self, key: Key) -> bool {
        if self.window.get_key(key) == Action::Press {
            true
        } else { 
            false 
        }
    }

    pub fn is_key_up(&mut self, key: Key) -> bool {
        if self.window.get_key(key) == Action::Release {
            true
        } else {
            false
        }
    }

    // TODO: Fix this for the love of gohf
    pub fn set_fullscreen(&mut self, fullscreen: &bool) {
        if self.event_handler.key_just_pressed(Key::F11) {
            if !fullscreen {
                self.glfw.with_primary_monitor(|_, monitor| {
                    let monitor = monitor.unwrap();
                    let mode = monitor.get_video_mode().unwrap();
                    self.window.set_monitor(
                        glfw::WindowMode::FullScreen(&monitor), 
                        0, 
                        0, 
                        mode.width, 
                        mode.height, 
                        Some(mode.refresh_rate),
                    );
                });
            } else {
                self.glfw.with_primary_monitor(|_, monitor| {
                    let monitor = monitor.unwrap();
                    let mode = monitor.get_video_mode().unwrap();
                    self.window.set_monitor(
                        glfw::WindowMode::Windowed, 
                        200, 
                        200, 
                        800, 
                        800, 
                        Some(mode.refresh_rate),
                    );
                });
            }
        }
    }
}
