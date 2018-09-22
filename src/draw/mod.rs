// Copyright Jeron A. Lau 2017-2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use Size;
use afi::{PathOp, over};

mod geometry;
mod rasterizer;

use self::geometry::{point};

pub(crate) use self::geometry::{Curve, Line};

/// Draw vector graphics on a VFrame.
#[inline(always)]
pub(crate) fn draw<I>(image: *mut u8, wh: Size, pitch: usize, path: I,
	color: [u8; 4], lines: &mut Vec<Line>, curves: &mut Vec<Curve>)
		where I: IntoIterator<Item = PathOp>
{
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

	rasterizer::rasterize(&lines, &curves, wh.0 as usize, wh.1 as usize,
		|x, y, v| {
			let index = (y * pitch + (x * 4)) as isize;

			let src = [
				color[0],
				color[1],
				color[2],
				(color[3] as f32 * v) as u8
			];

			use std::slice::from_raw_parts_mut as raw_slice;

			over(src, unsafe { raw_slice(image.offset(index), 4) });
		}
	);

	lines.clear();
	curves.clear();
}
