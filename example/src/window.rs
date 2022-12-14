use std::time::Duration;
use minifb::{
    Window,
    WindowOptions,
};

pub struct MyApp {
    window: Option<Window>,
    buf: Vec<u32>,
    name: String,
}

unsafe impl Send for MyApp {}
unsafe impl Sync for MyApp {}

impl MyApp {
    pub fn new(name: &str) -> Self {
        Self {
            window: None,
            buf: Vec::new(),
            name: name.to_string(),
        }
    }

    pub fn push_frame(&mut self, frame: &batrachia::VideoFrame) {
        let width = frame.width() as usize;
        let height = frame.height() as usize;

        if self.window.is_none() {
            let mut win = Window::new(
                &self.name,
                width,
                height,
                WindowOptions::default(),
            )
            .unwrap();

            win.limit_update_rate(Some(Duration::from_millis(
                1000 / 24,
            )));
            
            let _ = self.window.insert(win);
        }

        if self.buf.capacity() == 0 {
            self.buf = vec![0u32; width * height];
        }

        if let Some(window) = &mut self.window {
            if !window.is_open() {
                return;
            }

            unsafe { 
                libyuv::i420_to_argb(
                    frame.data_y().as_ptr(),
                    frame.stride_y() as i32,
                    frame.data_u().as_ptr(),
                    frame.stride_u() as i32,
                    frame.data_v().as_ptr(),
                    frame.stride_v() as i32,
                    self.buf.as_ptr() as *const u8,
                    (frame.width() * 4) as i32,
                    frame.width() as i32,
                    frame.height() as i32
                );
            }
  
            window
                .update_with_buffer(
                    &self.buf[..],
                    width,
                    height,
                )
                .unwrap();
        }
    }
}
