use minifb::{
    Key,
    Window,
    WindowOptions,
};

pub struct MyApp {
    window: Option<Window>,
    buf: Vec<u8>,
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

            win.limit_update_rate(Some(std::time::Duration::from_millis(
                1000 / 24,
            )));
            let _ = self.window.insert(win);
        }

        if self.buf.capacity() == 0 {
            self.buf = vec![0u8; width * height * 4];
        }

        if let Some(window) = &mut self.window {
            if !window.is_open() || window.is_key_down(Key::Escape) {
                return;
            }

            frame.to_rgba(&mut self.buf[..]).unwrap();
            window
                .update_with_buffer(
                    unsafe { std::mem::transmute(&self.buf[..]) },
                    width,
                    height,
                )
                .unwrap();
        }
    }
}
