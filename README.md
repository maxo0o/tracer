# tracer
Hobby Pathtracer written in Rust.

This project started as a way of learning the Rust programming language. Initially the project focused on learning Rust by porting the popular ray tracing tutorial series https://raytracing.github.io/ - you might notice a few simularities. This was a great jumping off point for adding the ability to render hundreds of large, textured Obj files.

The pathtracer uses a KDTree acceleration structure using the Surface Area Heuristic described in https://www.pbrt.org/.

![car](https://i.imgur.com/rlSgYAX.jpeg)
*Image: Rendered at 6000x4000 at 1200 samples per pixel*

## Limitations
  - Currently only supports Wavefront .obj file-format for models.
  - These .obj files need to be triangulated. If you're familiar with Blender, there is a Triangulate modifier that does just this (make sure to apply it before export).
  - No GPU implementation. Only runs on the CPU.

## Usuage
Spits output to `src/`.

`cargo run --release -- --scene examples/materials/scene.json`

Specify the output file yourself:
`cargo run --release -- --scene examples/car/scene.json --out ~/Desktop/my-cool-render.jpg`

## Platforms

Tested on both Windows 10 and MacOS. Should build without much pain.


