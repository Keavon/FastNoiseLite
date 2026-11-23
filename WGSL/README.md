## Getting Started

Here's an example of a fragment shader coloring each pixel with OpenSimplex2 noise

```wgsl
@fragment
fn main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32>
{
    // Create and configure noise state
    var noise = fnlCreateState(1337);
    noise.noise_type = FNL_NOISE_OPENSIMPLEX2;

    // Gather noise data, remapped from -1..1 to 0..1
    let value = fnlGetNoise2D(noise, position.x, position.y) * 0.5 + 0.5;

    return vec4<f32>(value, value, value, 1.0);
}
```

WGSL has no includes, so prepend `FastNoiseLite.wgsl` to your shader's source before compiling it. From Rust, the [`fastnoise-lite`](https://crates.io/crates/fastnoise-lite) crate's `"wgsl"` feature flag provides this file as the string `fastnoise_lite::WGSL_SOURCE`.
