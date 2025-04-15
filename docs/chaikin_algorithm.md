# Chaikin's Algorithm

## What is it?

Chaikin's algorithm is a simple way to create smooth curves from a set of points. It works by repeatedly cutting the corners of a shape.

## How It Works

1. Start with a shape made of connected points
2. For each line segment between points:
   - Create a point 25% of the way along the line
   - Create another point 75% of the way along the line
3. Replace the original points with these new points
4. Repeat to make the curve smoother

![Chaikin Process](https://upload.wikimedia.org/wikipedia/commons/thumb/d/d7/Chaikin_subdivision.svg/400px-Chaikin_subdivision.svg.png)

## Simple Example

Imagine a square with 4 corners. After one iteration of Chaikin's algorithm, it becomes an octagon (8 sides). After more iterations, it looks more and more like a circle.

## In Our Code

We create the new points like this:

```rust
// Create two new points that cut the corner
let q = p0 * (1.0 - ratio) + p1 * ratio;        // 25% point
let r = p0 * ratio + p1 * (1.0 - ratio);        // 75% point
```

## Key Properties

- Each iteration makes the curve smoother
- The curve always stays within the original shape
- The algorithm is fast and simple to implement
- We use 7 iterations for a good balance of smoothness and performance
