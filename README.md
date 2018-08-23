# barg
An accurate, low resource renderer.

## Motivation
API's like Vulkan, Metal and OpenGL have trouble with certain things like anti-aliasing (which gets "solved" with Multisampling), and how to make a perfect sphere (It's not really possible).

This renderer will have a lot of CPU-based function calls, so a GPU isn't necessary - but if you have vulkan installed, compute shaders will be used to speed things up.
