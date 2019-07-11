use barg::*;

fn redraw(nanos: u64) {
    
}

// Initialize graphic shader.
fn init_gui_shape(window: &mut Window) -> (Shader, Shape) {
    let mut gui = window.shader_new(barg::shader!("gui"));

    // Define vertices.
    #[rustfmt::skip]
    let vertices = [
        -1.0, -1.0,  0.0, 1.0,
         1.0, -1.0,  1.0, 1.0,
         1.0,  1.0,  1.0, 0.0,

        -1.0, -1.0,  0.0, 1.0,
        -1.0,  1.0,  0.0, 0.0,
         1.0,  1.0,  1.0, 0.0,
    ];

    // Build cube Shape
    let mut rect = window.shape_new(ShapeBuilder::new(&mut gui).vert(&vertices).face([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]));
    window.instances(&mut rect, &[Transform::new()]);
    window.build(&mut gui);

    (gui, rect)
}

fn main() {
    let mut window = Window::new("Barg Window", redraw, init_gui_shape);

    while window.run() { }
}
