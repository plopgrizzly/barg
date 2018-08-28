/// Size of an image (width, height).
#[derive(Copy, Clone)] pub struct Size(u16, u16);

/// A color in sRGBA.
#[derive(Copy, Clone)] pub struct Color(u16, u8, u8);

impl Color {
	/// Make a new color from sRGBA pixel
	pub fn srgb(color: [u8; 4]) -> Color {
		let r = color[0] as f32 / ::std::u8::MAX as f32;
		let g = color[1] as f32 / ::std::u8::MAX as f32;
		let b = color[2] as f32 / ::std::u8::MAX as f32;

		// TODO: Conversion
		let h = (r * ::std::u16::MAX as f32) as u16;
		let s = (g * ::std::u8::MAX as f32) as u8;
		let v = (b * ::std::u8::MAX as f32) as u8;

		Color(h, s, v)
	}
}

/// A raster pixel (64 bits).
#[derive(Clone)]
struct Pixel {
	hsv: Color, // 32 bit linear HSV.
	pct: u8, // 8 bit linear alpha value.
	fog: u8, // 8 bit linear fog value.
	int: u8, // 8 bit light intensity value.
	hue: u8, // 8 bit light hue value.
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
				hsv: Color(0u16, 0u8, 0u8),
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
				hsv: color,
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
