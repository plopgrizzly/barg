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

extern crate afi;
extern crate fonterator;
extern crate ami;
extern crate arrayvec;
extern crate ordered_float;
#[macro_use] extern crate approx;

mod draw;

/// The default font.
pub const FONT: &'static [u8] = include_bytes!("../font/LiberationMono-Regular.ttf");
pub use fonterator::Font;

pub use afi::{PathOp};
pub use PathOp::{Move, Line, Quad};

/// Size of an image (width, height).
#[derive(Copy, Clone)] pub struct Size(pub u16, pub u16);

/// Texture Coordinates (Mapped to a `PathOp`).
#[derive(Copy, Clone)]
pub struct TexCoord(pub f32, pub f32);

pub struct LinkSurface<'a>(&'a mut SurfaceInfo, *mut u8);

impl<'a> LinkSurface<'a> {
	/// New LinkSurface.
	#[inline(always)]
	pub fn new(info: &'a mut SurfaceInfo, pixbuf: *mut u8) -> LinkSurface {
		LinkSurface(info, pixbuf)
	}

	/// Clear
	#[inline(always)]
	pub fn clear(&self) {
		self.0.clear(self.1)
	}

	/// Draw a path a solid color (sRGBA).
	#[inline(always)]
	pub fn draw<T>(&mut self, color: [u8; 4], path: T)
		where T: IntoIterator<Item=PathOp>
	{
		self.0.draw(self.1, color, path);
	}

	/// Draw text.
	#[inline(always)]
	pub fn text(&mut self, color: [u8; 4], xysize: (f32, f32, f32),
		font: &Font, text: &str)
	{
		self.0.text(self.1, color, xysize, font, text);
	}
}

/// Surface Information.
pub struct SurfaceInfo {
	pub size: Size,
	lines: Vec<draw::Line>,
	curves: Vec<draw::Curve>,
	pitch: usize,
	len: usize,
}

impl SurfaceInfo {
	/// Create a new Surface Information.
	pub fn new(size: Size, pitch: Option<usize>) -> SurfaceInfo {
		let pitch = pitch.unwrap_or(size.0 as usize * 4);
		let len = pitch * size.1 as usize;

		SurfaceInfo {
			size, lines: vec![], curves: vec![], pitch, len,
		}
	}

	/// Clear the surface
	fn clear(&self, pixbuf: *mut u8) {
		unsafe { pixbuf.write_bytes(0, self.len) }
	}

	/// Draw a path a solid color (sRGBA).
	fn draw<T>(&mut self, pixbuf: *mut u8, color: [u8; 4], path: T)
		where T: IntoIterator<Item=PathOp>
	{
		let iter = path.into_iter();

		draw::draw(pixbuf, self.size, self.pitch, iter, color,
			&mut self.lines, &mut self.curves);
	}

	/// Draw text.
	fn text(&mut self, pixbuf: *mut u8, color: [u8; 4],
		xysize: (f32, f32, f32), font: &Font, text: &str)
	{
		let mut x = xysize.0;
		let mut y = xysize.1;
		let size = xysize.2;

		// Loop through the glyphs in the text, adding to the SVG.
		for g in font.glyphs(text, (size, size)) {
			// Check for newline
			if g.2 {
				x = xysize.0;
				y += size;
				continue;
			}

			// Draw the glyph
			self.draw(pixbuf, color, g.0.draw(x, y));

			// Position next glyph
			x += g.1;
		}
	}
}

/// An sRGBA Surface.
pub struct Surface {
	pub info: SurfaceInfo,
	pixels: Vec<u8>,
}

impl Surface {
	/// Create a new HSV Surface.
	pub fn new(size: Size, pitch: Option<usize>) -> Surface {
		let info = SurfaceInfo::new(size, pitch);

		println!("size {}", info.len);

		Surface { pixels: vec![0; info.len], info }
	}

	/// Clear the Surface to nothing.
	#[inline(always)]
	pub fn clear(&mut self) {
		for i in &mut self.pixels {
			*i = 0;
		}
	}

	#[inline(always)]
	pub fn len(&mut self) -> usize {
		self.info.size.0 as usize * self.info.size.1 as usize
	}

	/// Draw a path a solid color (sRGBA).
	#[inline(always)]
	pub fn draw<T>(&mut self, color: [u8; 4], path: T)
		where T: IntoIterator<Item=PathOp>
	{
		self.info.draw(self.pixels.as_mut_ptr(), color, path);
	}

	/// Draw text.
	#[inline(always)]
	pub fn text(&mut self, color: [u8; 4], xysize: (f32, f32, f32),
		font: &Font, text: &str)
	{
		self.info.text(self.pixels.as_mut_ptr(), color, xysize, font,
			text);
	}

	/// Blit to pixel buffer.
	#[inline(always)]
	pub unsafe fn blit(&self, pixel_buffer: *mut u8) {
		use std::ptr::copy_nonoverlapping as copy;

		copy(self.pixels.as_ptr(), pixel_buffer, self.info.len);
	}
}

/// A Graphical User Interface.
pub struct Gui {
	
}

impl Gui {
	pub fn new() -> Gui {
		Gui {
		}
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
