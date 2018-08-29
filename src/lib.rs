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

/// Size of an image (width, height).
#[derive(Copy, Clone)] pub struct Size(u16, u16);

/// A raster pixel (64 bits).
#[derive(Clone)]
struct Pixel {
	hsv: (u16, u8, u8), // 32 bit linear HSV.
	pct: u8, // 8 bit linear alpha value.
	fog: u8, // 8 bit linear fog value.
	int: u8, // 8 bit light intensity value.
	hue: u8, // 8 bit light hue value.
}

/// 3 dimensional path operation.
#[derive(Copy, Clone)]
pub enum PathOp3D {
	/// Set origin for a new face: `X`, `Y`, `Z`.
	Move(f32, f32, f32),
	/// Add a line to this face: `X`, `Y`, `Z`.
	Line(f32, f32, f32),
	/// Add a qaudratic curve to this face: `cX`, `cY`, `cZ`, `X`, `Y`, `Z`.
	/// This doesn't work yet.
	Quad(f32, f32, f32, f32, f32, f32),
}

pub use PathOp3D::{Move, Line, Quad};

/// Texture Coordinates (Mapped to a `PathOp3D`).
#[derive(Copy, Clone)]
pub struct TexCoord(pub f32, pub f32);

/// 32-bit sRGBA Color.
#[derive(Copy, Clone)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

impl Color {
	/// Convert sRGB to HSV
	fn hsv(self) -> (u16, u8, u8) {
		let r = self.0 as f32 / ::std::u8::MAX as f32;
		let g = self.1 as f32 / ::std::u8::MAX as f32;
		let b = self.2 as f32 / ::std::u8::MAX as f32;

		// TODO: Conversion
		let h = (r * ::std::u16::MAX as f32) as u16;
		let s = (g * ::std::u8::MAX as f32) as u8;
		let v = (b * ::std::u8::MAX as f32) as u8;

		(h, s, v)
	}
}

/// An HSV Surface.
pub struct Surface {
	#[allow(unused)] // TODO
	size: Size,
	pixels: Vec<Pixel>,
}

impl Surface {
	/// Create a new HSV Image.
	pub fn new(size: Size) -> Surface {
		Surface {
			size, pixels: vec![Pixel {
				hsv: (0u16, 0u8, 0u8),
				pct: 0,
				fog: 0,
				int: 0,
				hue: 0,
			}; size.0 as usize * size.1 as usize],
		}
	}

	/// Clear the Raster
	pub fn clear(&mut self, color: Color) {
		for i in &mut self.pixels {
			*i = Pixel {
				hsv: color.hsv(),
				pct: 0,
				fog: 0,
				int: 0,
				hue: 0,
			};
		}
	}

	/// 
	#[allow(unused)] // TODO
	pub fn draw<T>(&mut self, path: T) where T: Iterator<Item=PathOp3D> {
		
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
