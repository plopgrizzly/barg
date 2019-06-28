pub use window::ShaderBuilder;
pub use window::Shader;
pub use window::Shape;
pub use window::VertexList;

/// A Window.
pub struct Window {
    window: Box<window::Window>,
}

impl Window {
    /// Create a new Window.
    pub fn new(name: &str, _pixels: &[u8], _width: u16, run: fn(u64) -> ()) -> Window {
        let window = window::Window::new(name, run);

        Window {
            window,
        }
    }

    /// Run the render loop for this window.
    pub fn run(&mut self) -> bool {
        self.window.run()
    }

    /// Set the background color of the window.
    pub fn background(&mut self, r: f32, g: f32, b: f32) {
        self.window.background(r, g, b);
    }

    /// Build a shader.
    pub fn shader_new(&mut self, shader_builder: ShaderBuilder) -> Shader {
        self.window.shader_new(shader_builder)
    }

    /// Create a new vertex list.
    pub fn vertex_list_new(&mut self, vertices: &[f32], gradient: Option<&[f32]>, graphic_coords: Vec<&[f32]>) -> VertexList {
        self.window.vertex_list_new(vertices, gradient, graphic_coords)
    }

    /// Build a shape.
    pub fn shape_new(&mut self, indices: &[u16]) -> Shape {
        self.window.shape_new(indices)
    }
}
