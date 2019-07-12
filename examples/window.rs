use barg::*;

fn redraw(nanos: u64) {
    
}

fn main() {
    let mut window = Window::new("Barg Window", redraw, init_toolbar);
    let toolbar_height = window.toolbar_height;

    window.toolbar(&mut |buffer, width| {
        let height = buffer.len() / (4 * width as usize);
        let size = barg::Size(width, height as u16);
        let mut image = barg::Image::new(size);
        // Render Background.
        let shape = [
            Move(0.0, 0.0),
            Line(width.into(), 0.0),
            Line(width.into(), toolbar_height as f32),
            Line(0.0, toolbar_height as f32),
        ];
        image.fill([48, 24, 64, 0] /*color*/, &shape /*path*/, buffer /**/);
        // 
        let length = buffer.len() / 4;
        let pointer = buffer as *mut _ as *mut _;
        icons::back(unsafe { std::slice::from_raw_parts_mut(pointer, length) }, width, height as u16);
    });

    while window.run() { }
}
