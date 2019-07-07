# Barg
A GUI toolkit & 3D graphics renderer.

## Naming
The name is a combination of "bar" as in "foo bar baz qux" and "g" as in "graphics".  It is also fun to yell because "BARG!!!" sounds like "ARGH!!!!".

## Future Ideas
API's like Vulkan, Metal and OpenGL have trouble with certain things like anti-aliasing (which gets "solved" with Multisampling), and how to make a perfect sphere (It's not really possible).

This renderer will have a lot of CPU-based function calls, so a GPU isn't necessary - but if you have vulkan installed, compute shaders will be used to speed things up.  In order to get *even faster* SIMD will be used!  Hopefully, SIMD + Vulkan Compute for CPU rendering will be just as fast as normal GPU rendering (or at least close).

Having support for 3D curves, will not only make certain things (particulary living things, like plants and people) look better, but also reduce the amount of vertices that need to be stored in memory to draw models with curves.

Another factor that will be improved is color blending and worrying how to do that and have it look accurate.  HSV relative to linear sRGB will be interpolated for blending effects.

## More Future Ideas
Rendering is done face-by-face.  So a cube will be 6 draw calls (1 for each face), but of course at least 3 faces will be culled.  A sphere will need 4 draw calls / 4 faces, these will be 3D faces unlike the cube.

Alpha blending will be done backwards.  No matter what faces that are closest to the camera will always be drawn first.  The alpha value will be stored on the surface we're rendering to.  If it's 255 then pixels will be culled.  If it's less faces will be blended behind the face currently in the render buffer.

## Features
- Render 2D graphics
- Render text

## TODO
- Render 3D graphics
- Render GUI elements / widgets

## Cala
This crate is part of the [cala](https://crates.io/crates/cala) project.

## Links
- [Website](https://code.plopgrizzly.com/barg)
- [Cargo](https://crates.io/crates/barg)
- [Documentation](https://docs.rs/barg)
- [Change Log](https://code.plopgrizzly.com/barg/CHANGELOG)
- [Contributors](https://plopgrizzly.com/barg/CONTRIBUTORS)
- [Code of Conduct](https://code.plopgrizzly.com/barg/CODEOFCONDUCT)
- [Join Cala on Zulip](https://plopgrizzly.zulipchat.com/join/pp13s6clnexk03tvlnrtjvi1/)
