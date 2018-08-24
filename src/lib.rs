/// Size of an image (width, height).
#[derive(Copy, Clone)] pub struct Size(u16, u16);

/// A color in sRGBA.
#[derive(Copy, Clone)] pub struct Color(u16, u16, u16, u8);

impl Color {
	/// Make a new color from sRGBA pixel
	pub fn srgba(color: [u8; 4]) -> Color {
		let r = color[0] as f32 / ::std::u8::MAX as f32;
		let g = color[1] as f32 / ::std::u8::MAX as f32;
		let b = color[2] as f32 / ::std::u8::MAX as f32;

		// TODO: Conversion
		let r = (r * ::std::u16::MAX as f32) as u16;
		let g = (g * ::std::u16::MAX as f32) as u16;
		let b = (b * ::std::u16::MAX as f32) as u16;

		Color(r, g, b, color[3])
	}
}

/// A raster pixel (64 bits).
#[derive(Clone)]
struct Pixel {
	hue: u16, // linear RGB HSV hue
	sat: u16, // linear RGB HSV saturation
	val: u16, // linear RGB HSV value
	alpha: u8, // Alpha value (linear 0-255).
	fog: u8, // Fog distance (linear 0-255).
}

/// 3 dimensional path operation.
pub enum PathOp3D {
}

/// An image.
pub struct Image {
	#[allow(unused)] // TODO
	size: Size,
	pixels: Vec<Pixel>,
}

impl Image {
	/// Create a new HSV Image.
	pub fn new(size: Size) -> Image {
		Image {
			size, pixels: vec![Pixel {
				hue: 0,
				sat: 0,
				val: 0,
				alpha: 0,
				fog: 0,
			}; size.0 as usize * size.1 as usize],
		}
	}

	/// Clear the Raster
	pub fn clear(&mut self, color: Color) {
		for i in &mut self.pixels {
			*i = Pixel {
				hue: color.0,
				sat: color.1,
				val: color.2,
				alpha: color.3,
				fog: 0,
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
