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

use std::cell::RefCell;

/// The default font.
pub const FONT: &'static [u8] = include_bytes!("../font/LiberationMono-Regular.ttf");
pub use fonterator::{Font, Path2D, PathOp, PathOp::*};

// pub use afi::{PathOp};
pub use PathOp::{Line, Move, Quad};

/// Size of an image (width, height).
#[derive(Copy, Clone)]
pub struct Size(pub u16, pub u16);

/// Texture Coordinates (Mapped to a `PathOp`).
#[derive(Copy, Clone)]
pub struct TexCoord(pub f32, pub f32);

/// An Image
pub struct Image<'a> {
    plotter: footile::Plotter,
    raster: footile::Raster<'a, footile::Rgba8>,
}

impl<'a> Image<'a> {
    /// Create a new Image from a pointer.
    pub fn from_ptr(size: Size, pixels: *mut u8) -> Image<'a> {
        let len = size.0 as usize * size.1 as usize * 4;
        Image::from_slice(size, unsafe { std::slice::from_raw_parts_mut(pixels, len) })
    }

    /// Create a new Image from a pixel slice.
    pub fn from_slice(size: Size, pixels: &'a mut [u8]) -> Image<'a> {
        let (w, h) = (size.0 as u32, size.1 as u32);

        let (_, pixels, _) = unsafe {
            pixels.align_to_mut::<footile::Rgba8>()
        };

        Image {
            plotter: footile::Plotter::new(w, h),
            raster: footile::Raster::with_pixels::<footile::Rgba8>(w, h, RefCell::new(pixels)),
        }
    }

    /// Create new Image.
    pub fn new(size: Size) -> Self {
        let (w, h) = (size.0 as u32, size.1 as u32);

        Image {
            plotter: footile::Plotter::new(w, h),
            raster: footile::Raster::new(w, h),
        }
    }

    /// Clear the Image.
    pub fn clear(&mut self) {
        self.raster.clear();
    }

    /// Draw a path a solid color (sRGBA).
    pub fn draw<'b, T>(&mut self, color: [u8; 4], path: T)
    where
        T: IntoIterator<Item = &'b PathOp>,
    {
        let iter = path.into_iter();
        let color = footile::Rgba8::new(color[0], color[1], color[2], color[3]);

        self.raster.over(self.plotter.fill(iter, footile::FillRule::NonZero), color);
    }

    /// Draw text.
    pub fn text(
        &mut self,
        color: [u8; 4],
        xysize: (f32, f32, f32),
        font: &'a Font,
        text: &str,
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
                    },
                    PathOp::Line(x, y) => path.push(PathOp::Line(x, y)),
                    PathOp::Quad(x, y, z, w) => path.push(PathOp::Quad(x, y, z, w)),
                    _ => panic!("oops"),
                }
            }

            // Position next glyph
            x += g.1;
        }

        self.raster.over(self.plotter.fill(path.iter(), footile::FillRule::NonZero), color);
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
