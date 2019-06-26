use barg::{FontGroup, Gui, Image, Size};

use png::HasParameters;
use std::fs::File;
use std::io;

/// Write the raster to a PNG (portable network graphics) file.
pub fn write_png(
    width: u32,
    height: u32,
    pixels: &[u8],
    filename: &str,
) -> io::Result<()> {
    let fl = File::create(filename)?;
    let ref mut bw = io::BufWriter::new(fl);
    let mut enc = png::Encoder::new(bw, width, height);
    enc.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = enc.write_header()?;
    writer.write_image_data(&pixels[..])?;
    Ok(())
}

fn main() {
    // Load GUI resources.
    let font = FontGroup::default();
    let mut gui = Gui::new(font);

    // Initialize variables need to write to PNG
    let w = 640;
    let h = 360;
    let mut buffer = vec![0; w * h * 4];
    let mut surface = Image::new(Size(w as u16, h as u16));

    gui.head(&mut surface, &mut buffer, &|row, color| {
        *color = [48, 48, 64, 255, 1];
        &[(&[], "Hello, worldy!"), (&[], "Test")]
    });

    gui.page(&mut surface, &mut buffer, &|row, color| {
        *color = [0x80, 0xFF, 0x80, 255, 1];
        match row {
            0 => &[(&[], "Hello, worldy!")],
            1 => &[(&[], "Yo!"), (&[], "Sup‽")],
            _ => &[],
        }
    });

    // Save the image to a PNG file.
    write_png(w as u32, h as u32, buffer.as_slice(), "image_example.png")
        .unwrap();
}
