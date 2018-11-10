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

//        println!("{} {} {} {}", points[0].y, points[1].y, points[2].y, points[3].y);

	    rasterizer::rasterize(&lines, &curves, wh.0 as usize, wh.1 as usize,
		    |x, y, v| {
			    let out_index = (y * pitch + (x * 4)) as isize;

                // Calculate Texture Coordinates
                let x = x as f32;
                let y = y as f32;

                let tx = (x as f32 - upleft.0) * resizex;
                let ty = (y as f32 - upleft.1) * resizey;

//                println!("{} {}", tx, ty);

//                let xf = x as f32;
//                let yf = y as f32;

                // Calculate Texture Coordinates
/*                let mut dist = [
                    (((xf - points[0].x) * (xf - points[0].x))
                        + ((yf - points[0].y) * (yf - points[0].y))).sqrt(),
                    (((xf - points[1].x) * (xf - points[1].x))
                        + ((yf - points[1].y) * (yf - points[1].y))).sqrt(),
                    (((xf - points[2].x) * (xf - points[2].x))
                        + ((yf - points[2].y) * (yf - points[2].y))).sqrt(),
                    (((xf - points[3].x) * (xf - points[3].x))
                        + ((yf - points[3].y) * (yf - points[3].y))).sqrt(),
                ];
                let mut total = 0.0;
                for i in dist.iter() {
                    total += i;
                }
                dist[0] /= total;
                dist[1] /= total;
                dist[2] /= total;
                dist[3] /= total;
                let mut point = point(0.0, 0.0);
                point = point + vector(1.0 * dist[1], 0.0);
                point = point + vector(1.0 * dist[2], 1.0 * dist[2]);
                point = point + vector(0.0, 1.0 * dist[3]);

                let x = point.x * ((texture.0 + 1) as f32);
                let y = point.y * ((texture.1 + 1) as f32);*/

/*                let src = [
                    0,
                    (x * 255.0) as u8,
                    (y * 255.0) as u8,
                    (255.0 * v) as u8,
                ];*/

/*                let texx = ((point.x * ((texture.0 + 1) as f32)) as u16).min(texture.0 - 1);
                let texy = ((point.y * ((texture.1 + 1) as f32)) as u16).min(texture.1 - 1);*/

                let ti = (tx as usize).min(texture.0 as usize - 1)
                    + ((ty as usize).min(texture.1 as usize - 1) * texture.0 as usize);

                // Sample Texture
			    let src = [
				    texture.2[ti][0],
				    texture.2[ti][1],
				    texture.2[ti][2],
				    (texture.2[ti][3] as f32 * v) as u8
			    ];

                // Composite Texture
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
