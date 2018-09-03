// Copyright Jeron A. Lau 2017-2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use Size;
use afi::{VFrame, PathOp, ColorChannels, Lhsva, Srgba};

mod geometry;
mod rasterizer;

use self::geometry::{Curve, Line, point};

fn blend(mut src: [f64; 4], mut dst: [f64; 4]) -> [f64; 4] {
	// Source over destination (algorithm modified for RGBA from
	// https://en.wikipedia.org/wiki/Alpha_compositing#Description

	// Premultiply alpha
	src[0] *= src[3];
	src[1] *= src[3];
	src[2] *= src[3];
	dst[0] *= dst[3];
	dst[1] *= dst[3];
	dst[2] *= dst[3];
	// Calculate the 4 color channels.
	dst[0] = src[0] + dst[0] * (1.0 - src[3]);
	dst[1] = src[1] + dst[1] * (1.0 - src[3]);
	dst[2] = src[2] + dst[2] * (1.0 - src[3]);
	dst[3] = src[3] + dst[3] * (1.0 - src[3]);
	// Bring the color value up (Postdivide alpha) [RGBA dst specific]
	dst[0] /= dst[3];
	dst[1] /= dst[3];
	dst[2] /= dst[3];

	dst
}

// Composite one sRGBA value ontop of another (mixing colors with lHSVA).
fn composite(src: [u8; 4], dst: [u8; 4]) -> [u8; 4] {
	// Convert sRGBA to linear HSVA
	let src = Lhsva.from(Srgba, src);
	let dst = Lhsva.from(Srgba, dst);

	// If alpha value is 0, optimize.
/*	if src[3] == 0 {
		Srgba.from(Lhsva, dst)
	} else if dst[3] == 0 {
		Srgba.from(Lhsva, src)
	} else {
		panic!("wfts");*/

		// Blend Colors
		let mix = blend([
			src[0] as f64 / 255.0,
			src[1] as f64 / 255.0,
			src[2] as f64 / 255.0,
			src[3] as f64 / 255.0
		], [
			dst[0] as f64 / 255.0,
			dst[1] as f64 / 255.0,
			dst[2] as f64 / 255.0,
			dst[3] as f64 / 255.0
		]);

		// Convert linear RGBA back to sRGBA
		Srgba.from(Lhsva, [
			(mix[0] * 255.0) as u8,
			(mix[1] * 255.0) as u8,
			(mix[2] * 255.0) as u8,
			(mix[3] * 255.0) as u8,
		])
//	}
}

/// Draw vector graphics on a VFrame.
pub(crate) fn draw<I>(vframe: &mut VFrame, wh: Size, path: I,
	color: [u8; 4])
		where I: IntoIterator<Item = PathOp>
{
        let mut lines = Vec::new();
        let mut curves = Vec::new();
	let mut last = point(0.0, 0.0);

	for path_op in path.into_iter() {
		match path_op {
			PathOp::Move(x, y, _z) => {
				last = point(x, y);
			}
			PathOp::Line(x, y, _z) => {
				let end = point(x, y);
				lines.push(Line { p: [last, end] });
				last = end;
			}
			PathOp::Quad(cx, cy, _cz, x, y, _z) => {
				let end = point(x, y);
				let control = point(cx, cy);
				curves.push(Curve {
					p: [last, control, end],
				});
				last = end;
			}
			PathOp::Cubic(_c1x, _c1y, _c1z, _c2x, _c2y, _c2z,
				_x, _y, _z) =>
			{
				unimplemented!()
			}
			PathOp::Width(_) => { /*ignore*/ }
		}
	}

	rasterizer::rasterize(&lines, &curves, wh.0, wh.1, |x, y, v| {
		let index = (y as usize * wh.0 as usize) + x as usize;

		let dst = vframe.get_rgba(ColorChannels::Srgba, index);
		let src = [color[0], color[1], color[2],
			(color[3] as f32 * v) as u8];

		vframe.set_rgba(ColorChannels::Srgba, index,
			composite(src, dst));
	});
}
