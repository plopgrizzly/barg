use barg::*;

fn redraw(nanos: u64) {
    
}

fn toolbar(buffer: &mut [u8], width: u16) {
    let height = buffer.len() / (4 * width as usize);
    let size = barg::Size(width, height as u16);
    let mut image = barg::Image::new(size);
    // Render Background.
    let shape = [
        Move(0.0, 0.0),
        Line(width.into(), 0.0),
        Line(width.into(), height as f32),
        Line(0.0, height as f32),
    ];
    image.fill([48, 24, 64, 0] /*color*/, &shape /*path*/, buffer /**/);
    // 
    let length = buffer.len() / 4;
    let pointer = buffer as *mut _ as *mut _;
    let slice = unsafe { std::slice::from_raw_parts_mut(pointer, length) };

    icons::menu(slice, 0, width, height as u16);
    icons::zoom_out(slice, 1, width, height as u16);
    icons::zoom_in(slice, 3, width, height as u16);
    icons::view(slice, 5, width, height as u16);
    icons::search(slice, 7, width, height as u16);
    icons::fullscreen(slice, 9, width, height as u16);
    icons::grid(slice, 11, width, height as u16);
    icons::next(slice, 13, width, height as u16);
}

fn main() {
    let mut window = Window::new("Barg Window", redraw, init_toolbar);

    window.toolbar(toolbar);

    while window.run() { }
}
