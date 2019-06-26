/// A Window.
pub struct Window {
    window: Box<window::Window>,
}

impl Window {
    /// Create a new Window.
    pub fn new(name: &str, _pixels: &[u8], _width: u16, run: fn(u64) -> ()) -> Window {
        let window = window::Window::new(run);

        Window {
            window,
        }
    }

    /// Run the render loop for this window.
    pub fn run(&mut self) -> bool {
        self.window.run()
    }
}
