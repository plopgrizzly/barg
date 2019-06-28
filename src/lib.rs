//! A GUI toolkit & 3D graphics renderer.
//!
//! ## Naming
//! The name is a combination of "bar" as in "foo bar baz qux" and "g" as in "graphics".  It is also fun to yell because "BARG!!!" sounds like "ARGH!!!!".
//!
//!

extern crate fonterator;

use fonterator::footile;

mod gui;
mod window;

pub use crate::gui::Gui;
pub use crate::window::*;

pub use fonterator::{
    FontGroup, PathOp,
    PathOp::*,
    PathOp::{Line, Move, Quad},
};

use footile::PixFmt;

/// Size of an image (width, height).
#[derive(Copy, Clone)]
pub struct Size(pub u16, pub u16);

/// Texture Coordinates (Mapped to a `PathOp`).
#[derive(Copy, Clone)]
pub struct TexCoord(pub f32, pub f32);

/// An Image
pub struct Image {
    plotter: footile::Plotter,
    raster: footile::RasterB<footile::Rgba8>,
}

impl Image {
    /// Create new Image.
    pub fn new(size: Size) -> Self {
        let (w, h) = (size.0 as u32, size.1 as u32);

        Image {
            plotter: footile::Plotter::new(w, h),
            raster: footile::RasterB::new(w, h),
        }
    }

    /// Get the size of the image.
    pub fn size(&self) -> Size {
        Size(self.raster.width() as u16, self.raster.height() as u16)
    }

    /// Clear the Image.
    pub fn clear_ptr(&mut self, pixels: *mut u8) {
        let len =
            self.raster.width() as usize * self.raster.height() as usize * 4;
        self.clear(unsafe { std::slice::from_raw_parts_mut(pixels, len) })
    }

    /// Clear the Image.
    pub fn clear(&mut self, pixels: &mut [u8]) {
        self.raster.clear(footile::Rgba8::as_slice_mut(pixels));
    }

    /// Draw a path a solid color (sRGBA).
    pub fn fill_ptr<'b, T>(&mut self, color: [u8; 4], path: T, pixels: *mut u8)
    where
        T: IntoIterator<Item = &'b PathOp>,
    {
        let len =
            self.raster.width() as usize * self.raster.height() as usize * 4;
        self.fill(color, path, unsafe {
            std::slice::from_raw_parts_mut(pixels, len)
        })
    }

    /// Draw a path a solid color (sRGBA).
    pub fn stroke_ptr<'b, T>(
        &mut self,
        color: [u8; 4],
        path: T,
        pixels: *mut u8,
    ) where
        T: IntoIterator<Item = &'b PathOp>,
    {
        let len =
            self.raster.width() as usize * self.raster.height() as usize * 4;
        self.stroke(color, path, unsafe {
            std::slice::from_raw_parts_mut(pixels, len)
        })
    }

    /// Draw a path a solid color (sRGBA).
    pub fn fill<'b, T>(&mut self, color: [u8; 4], path: T, pixels: &mut [u8])
    where
        T: IntoIterator<Item = &'b PathOp>,
    {
        let iter = path.into_iter();
        let color = footile::Rgba8::new(color[0], color[1], color[2], color[3]);

        self.raster.over(
            self.plotter.fill(iter, footile::FillRule::NonZero),
            color,
            footile::Rgba8::as_slice_mut(pixels),
        );
    }

    /// Draw a path a solid color (sRGBA).
    pub fn stroke<'b, T>(&mut self, color: [u8; 4], path: T, pixels: &mut [u8])
    where
        T: IntoIterator<Item = &'b PathOp>,
    {
        let iter = path.into_iter();
        let color = footile::Rgba8::new(color[0], color[1], color[2], color[3]);

        self.raster.over(
            self.plotter.stroke(iter),
            color,
            footile::Rgba8::as_slice_mut(pixels),
        );
    }

    /// Draw text.
    pub fn text_ptr(
        &mut self,
        color: [u8; 4],
        xysize: (f32, f32, f32),
        font: &FontGroup,
        text: &str,
        pixels: *mut u8,
    ) -> (f32, f32) {
        let len =
            self.raster.width() as usize * self.raster.height() as usize * 4;
        self.text(color, xysize, font, text, unsafe {
            std::slice::from_raw_parts_mut(pixels, len)
        })
    }

    /// Draw text.
    pub fn text(
        &mut self,
        color: [u8; 4],
        xysize: (f32, f32, f32),
        font: &FontGroup,
        text: &str,
        pixels: &mut [u8],
    ) -> (f32, f32) {
        let color = footile::Rgba8::new(color[0], color[1], color[2], color[3]);

        // Render the text
        let mut path = font.render(
            text,                 /*text*/
            (xysize.0, xysize.1), /*position*/
            (xysize.2, xysize.2), /*size*/
        );

        self.raster.over(
            self.plotter.fill(&mut path, footile::FillRule::NonZero),
            color,
            footile::Rgba8::as_slice_mut(pixels),
        );

        let (cx, cy) = path.xy();

        (cx, cy)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
