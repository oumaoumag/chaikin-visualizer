# Chaikin Module

## Overview

The Chaikin module implements the core algorithm for curve smoothing. It provides functions to perform the corner-cutting operations that transform a polygon into a smooth curve.

## Main Functions

### chaikin_iteration

Performs a single iteration of the corner-cutting algorithm:

```rust
pub fn chaikin_iteration(points: &[Vec2], ratio: f32) -> Vec<Vec2>
```

- **points**: A slice of 2D points representing a polyline or polygon
- **ratio**: The corner-cutting ratio (typically 0.25)
- **returns**: A new vector of points representing the smoothed curve

### apply_chaikin

Applies multiple iterations of the algorithm and stores all intermediate steps:

```rust
pub fn apply_chaikin(points: &[Vec2], iterations: usize, ratio: f32) -> Vec<Vec<Vec2>>
```

- **points**: Initial set of points
- **iterations**: Number of times to apply the algorithm
- **ratio**: Corner cutting ratio
- **returns**: A vector containing all steps of the algorithm

## How It Works

For each pair of consecutive points (P₁, P₂), the algorithm:

1. Creates a point Q at 25% of the way from P₁ to P₂
2. Creates a point R at 75% of the way from P₁ to P₂
3. Replaces the original points with these new points

This process is repeated for the desired number of iterations.

## Example

```rust
// Define some points for a square
let points = vec![
    Vec2::new(100.0, 100.0),
    Vec2::new(300.0, 100.0),
    Vec2::new(300.0, 300.0),
    Vec2::new(100.0, 300.0),
];

// Apply 3 iterations of Chaikin's algorithm with a 0.25 ratio
let all_steps = apply_chaikin(&points, 3, 0.25);

// The final smoothed curve is the last element
let smooth_curve = &all_steps[all_steps.len() - 1];
```
