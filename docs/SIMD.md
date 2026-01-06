### SIMD Implementation

The `wide` crate uses `f64x4` SIMD types to process 4 double-precision floats simultaneously:

- **Vector operations**: 4x parallelism for x, y, z components
- **Dot products**: Parallel multiplication followed by reduction
- **Normalization**: Parallel computation of magnitude and scaling

### SIMD Operations

```rust
let v_simd = f64x4::new([v.x, v.y, v.z, 0.0]);  // Pack into SIMD register
let norm_sq = arr[0] * arr[0] + arr[1] * arr[1] + arr[2] * arr[2];  // Compute magnitude
let inv_norm = f64x4::splat(1.0 / norm_sq.sqrt());  // Broadcast scalar
let normalized = v_simd * inv_norm;  // SIMD multiplication
```

### Compatibility

- **CPU Requirements**: Modern x86_64 CPUs with SSE2/AVX (standard on all modern processors) (Mine: I7 14650HX 😊)
- **Portability**: `wide` crate provides fallbacks for platforms without SIMD support
- **Precision**: Maintains full f64 precision throughout

