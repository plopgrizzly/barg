//! The accurate, low resource renderer.
//!
//! # Motivation
//! API's like Vulkan, Metal and OpenGL have trouble with certain things like anti-aliasing (which  gets "solved" with Multisampling), and how to make a perfect sphere (It's not really possible).
//!
//! This renderer will have a lot of CPU-based function calls, so a GPU isn't necessary - but if you have vulkan installed, compute shaders will be used to speed things up.  In order to get *even faster* SIMD will be used!  Hopefully, SIMD + Vulkan Compute for CPU rendering will be just as fast as normal GPU rendering (or at least close).
//!
//! Having support for 3D curves, will not only make certain things (particulary living things, like plants and people) look better, but also reduce the amount of vertices that need to be stored in memory to draw models with curves.
//!
//! Another factor that will be improved is color blending and worrying how to do that and have it look accurate.  HSV relative to linear sRGB will be interpolated for blending effects.
//!
//! # Naming
//! The name is a combination of "bar" as in "foo bar baz qux" and "g" as in "graphics".  It is also fun to yell because "BARG!!!" sounds like "ARGH!!!!".
//!
//! # More ideas
//! Rendering is done face-by-face.  So a cube will be 6 draw calls (1 for each face), but of course at least 3 faces will be culled.  A sphere will need 4 draw calls / 4 faces, these will be 3D faces unlike the cube.
//!
//! Alpha blending will be done backwards.  No matter what faces that are closest to the camera will always be drawn first.  The alpha value will be stored on the surface we're rendering to.  If it's 255 then pixels will be culled.  If it's less faces will be blended behind the face currently in the render buffer.

extern crate fonterator;
extern crate footile;

/// The default font.
pub const FONT: &'static [u8] = include_bytes!("../font/LiberationMono-Regular.ttf");
pub use fonterator::{Font, Path2D, PathOp, PathOp::*};

// pub use afi::{PathOp};
pub use PathOp::{Line, Move, Quad};

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
    /*    /// Create a new Image from a pointer.
    pub fn from_ptr(size: Size, pixels: *mut u8) -> Image {
        let len = size.0 as usize * size.1 as usize * 4;
        Image::from_slice(size, unsafe { std::slice::from_raw_parts_mut(pixels, len) })
    }

    /// Create a new Image from a pixel slice.
    pub fn from_slice(size: Size, pixels: &mut [u8]) -> Image {
        let (w, h) = (size.0 as u32, size.1 as u32);

        let pixels = footile::Rgba8.as_slice_mut(pixels);

        Image {
            plotter: footile::Plotter::new(w, h),
            raster: footile::RasterB::new::<footile::Rgba8>(w, h),
        }
    }*/

    /// Create new Image.
    pub fn new(size: Size) -> Self {
        let (w, h) = (size.0 as u32, size.1 as u32);

        Image {
            plotter: footile::Plotter::new(w, h),
            raster: footile::RasterB::new(w, h),
        }
    }

    /// Clear the Image.
    pub fn clear_ptr(&mut self, pixels: *mut u8) {
        let len = self.raster.width() as usize * self.raster.height() as usize * 4;
        self.clear(unsafe { std::slice::from_raw_parts_mut(pixels, len) })
    }

    /// Clear the Image.
    pub fn clear(&mut self, pixels: &mut [u8]) {
        self.raster.clear(footile::Rgba8::as_slice_mut(pixels));
    }

    /// Draw a path a solid color (sRGBA).
    pub fn draw_ptr<'b, T>(&mut self, color: [u8; 4], path: T, pixels: *mut u8)
    where
        T: IntoIterator<Item = &'b PathOp>,
    {
        let len = self.raster.width() as usize * self.raster.height() as usize * 4;
        self.draw(color, path, unsafe {
            std::slice::from_raw_parts_mut(pixels, len)
        })
    }

    /// Draw a path a solid color (sRGBA).
    pub fn draw<'b, T>(&mut self, color: [u8; 4], path: T, pixels: &mut [u8])
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

    /// Draw text.
    pub fn text_ptr(
        &mut self,
        color: [u8; 4],
        xysize: (f32, f32, f32),
        font: &Font,
        text: &str,
        pixels: *mut u8,
    ) {
        let len = self.raster.width() as usize * self.raster.height() as usize * 4;
        self.text(color, xysize, font, text, unsafe {
            std::slice::from_raw_parts_mut(pixels, len)
        })
    }

    /// Draw text.
    pub fn text(
        &mut self,
        color: [u8; 4],
        xysize: (f32, f32, f32),
        font: &Font,
        text: &str,
        pixels: &mut [u8],
    ) {
        let color = footile::Rgba8::new(color[0], color[1], color[2], color[3]);

        let mut x = xysize.0;
        let mut y = xysize.1;
        let size = xysize.2;
        let mut path: Vec<PathOp> = Vec::new();
        let mut first = true;

        //        font.render(text, (size, size), x, y, &mut self.plotter);

        // Loop through the glyphs in the text, adding to the SVG.
        for g in font.glyphs(text, (size, size)) {
            // Check for newline
            if g.2 {
                x = xysize.0;
                y += size;
                continue;
            }

            for i in g.0.draw(x, y).iter() {
                match *i {
                    PathOp::Move(x, y) => {
                        if first {
                            first = false;
                        }
                        path.push(PathOp::Move(x, y));
                    }
                    PathOp::Line(x, y) => path.push(PathOp::Line(x, y)),
                    PathOp::Quad(x, y, z, w) => path.push(PathOp::Quad(x, y, z, w)),
                    _ => panic!("oops"),
                }
            }

            // Position next glyph
            x += g.1;
        }

        self.raster.over(
            self.plotter.fill(path.iter(), footile::FillRule::NonZero),
            color,
            footile::Rgba8::as_slice_mut(pixels),
        );
    }
}

/// A Graphical User Interface.
pub struct Gui {}

impl Gui {
    pub fn new() -> Gui {
        Gui {}
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
