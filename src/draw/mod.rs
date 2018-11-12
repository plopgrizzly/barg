// Copyright Jeron A. Lau 2017-2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use Size;
use afi::{PathOp, over};

mod geometry;
mod rasterizer;
mod util;

use self::geometry::{point};

pub(crate) use self::geometry::{Curve, Line};
pub(crate) use self::rasterizer::init;

/// Draw vector graphics on a VFrame.
#[inline(always)]
pub(crate) fn bitmap<'a, I>(image: *mut u8, wh: Size, pitch: usize, path: I,
	texture: (u16, u16, &[[u8; 4]]), lines: &mut Vec<Line>, curves: &mut Vec<Curve>, w: f32, h: f32)
		where I: IntoIterator<Item = &'a PathOp>
{
	let mut last;
    let mut path = path.into_iter();
    let mut points = vec![];

    loop {
        // 1. Move
        if let Some(path_op) = path.next() {
    		match path_op {
			    PathOp::Move(x, y, _z) => {
				    last = point(x * w, y * h);
                    points.push(last);
			    }
                _ => panic!("1st PathOp must be PathOp::Move when drawing bitmaps"),
            }
        } else {
            break;
        }

        // 2, 3, 4.
        while points.len() < 5 {
            if let Some(path_op) = path.next() {
        		match path_op {
			        PathOp::Line(x, y, _z) => {
				        let end = point(x * w, y * h);
				        lines.push(Line { p: [last, end] });
				        last = end;
                        points.push(last);
			        }
			        PathOp::Quad(cx, cy, _cz, x, y, _z) => {
				        let end = point(x * w, y * h);
				        let control = point(cx * w, cy * h);
				        curves.push(Curve {
					        p: [last, control, end],
				        });
                        let t1: crate::draw::geometry::Vector<f32> = (last + control.into()).into();
                        let t1: crate::draw::geometry::Point<f32> = (t1 / 2.0).into();
                        let t2: crate::draw::geometry::Vector<f32> = (control + end.into()).into();
                        let t2: crate::draw::geometry::Vector<f32> = (t1 + (t2 / 2.0)).into();

                        points.push((t2 / 2.0).into());
				        last = end;
                        points.push(last);
			        }
                    _ => panic!("PathOp must be PathOp::Line or PathOp::Quad when drawing bitmaps"),
                }
            } else {
                panic!("Incomplete path for bitmap.");
            }
        }

        let upleft = (points[0].x, points[0].y);
        let dright = (points[2].x, points[2].y);

        let w = dright.0 - upleft.0;
        let h = dright.1 - upleft.1;

        let resizex = texture.0 as f32 / w;
        let resizey = texture.1 as f32 / h;

        let texmaxx = texture.0 as usize - 1;
        let texmaxy = texture.1 as usize - 1;

        let texw = texture.0 as usize;

	    rasterizer::rasterize(&lines, &curves, wh.0 as usize, wh.1 as usize,
		    |x, y, v, xf, yf| {
                // Calculate Texture Coordinates
                let tx = (xf - upleft.0) * resizex;
                let ty = (yf - upleft.1) * resizey;

                let ti = (tx as usize).min(texmaxx) + ((ty as usize).min(texmaxy) * texw);

                // Sample Texture
			    let src = [
				    texture.2[ti][0],
				    texture.2[ti][1],
				    texture.2[ti][2],
                    util::scale_u8(texture.2[ti][3], v),
			    ];

                // Composite Texture
			    let out_index = (y * pitch + (x * 4)) as isize;
			    over(src, unsafe { std::slice::from_raw_parts_mut(image.offset(out_index), 4) });
		    }
	    );

	    lines.clear();
	    curves.clear();
        points.clear();
    }
}

/// Draw vector graphics on a VFrame.
#[inline(always)]
pub(crate) fn draw<'a, I>(image: *mut u8, wh: Size, pitch: usize, path: I,
	color: [u8; 4], lines: &mut Vec<Line>, curves: &mut Vec<Curve>, w: f32, h: f32)
		where I: IntoIterator<Item = &'a PathOp>
{
	let mut last = point(0.0, 0.0);

	for path_op in path.into_iter() {
		match *path_op {
			PathOp::Move(x, y, _z) => {
				last = point(x * w, y * h);
			}
			PathOp::Line(x, y, _z) => {
				let end = point(x * w, y * h);
				lines.push(Line { p: [last, end] });
				last = end;
			}
			PathOp::Quad(cx, cy, _cz, x, y, _z) => {
				let end = point(x * w, y * h);
				let control = point(cx * w, cy * h);
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
		|x, y, v, _xf, _yf| {
			let index = (y * pitch + (x * 4)) as isize;

			let src = [
				color[0],
				color[1],
				color[2],
                util::scale_u8(color[3], v),
			];

			use std::slice::from_raw_parts_mut as raw_slice;

			over(src, unsafe { raw_slice(image.offset(index), 4) });
		}
	);

	lines.clear();
	curves.clear();
}
