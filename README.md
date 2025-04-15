# Chaikin's Algorithm Visualization

An interactive visualization of Chaikin's curve subdivision algorithm implemented in Rust using the Macroquad graphics library. This tool demonstrates how a simple corner-cutting technique can create smooth curves from a set of control points.

![Chaikin's Algorithm Demo](demo.gif)

## About Chaikin's Algorithm

Chaikin's algorithm is an elegant method for curve subdivision that creates smooth curves by iteratively "cutting corners". The process:

1. Takes a set of points forming a polyline/polygon
2. For each line segment between consecutive points:
   - Creates a point at 25% of the segment length
   - Creates another point at 75% of the segment length
3. Replaces original points with these new points
4. Repeats the process for smoother results

Each iteration produces a smoother curve that approaches a quadratic B-spline.

## Features

- **Interactive Drawing**: Click to place control points anywhere on the canvas
- **Point Manipulation**: Drag existing points to adjust the curve
- **Real-time Preview**: See the original polygon while drawing
- **Animated Visualization**: Watch the smoothing process through multiple iterations
- **Playback Control**: Pause/resume animation, reset the canvas
- **Automatic Closure**: Creates closed curves with 3 or more points

## Controls

- **Left Mouse Button**: Add or drag control points
- **Enter**: Start the animation
- **Space**: Pause/resume animation
- **R**: Reset the canvas
- **Escape**: Quit the application

## Technical Details

- **Implementation Language**: Rust
- **Graphics Library**: Macroquad 0.4
- **Algorithm Parameters**:
  - Corner cutting ratio: 0.25 (25%)
  - Animation steps: 7 iterations
  - Animation speed: 0.5 seconds per step
- **Window Size**: 1024x768 pixels

## Building and Running

### Prerequisites
- Rust toolchain (1.70 or later)
- Cargo package manager

### Installation

1. Clone the repository:
```bash
git clone https://github.com/oumaoumag/chaikin-visualizer.git
cd chaikin-visualizer
```

2. Build and run:
```bash
cargo run --release
```

## Project Structure

```
chaikin-visualizer/
├── src/
│   ├── main.rs         # Application entry point and event loop
│   ├── chaikin.rs      # Core algorithm implementation
│   └── animation.rs    # Animation and state management
├── Cargo.toml          # Project dependencies and configuration
├── README.md           # Project documentation
└── LICENSE            # MIT License
```

## Contributing

Contributions are welcome! Feel free to:
- Report bugs
- Suggest features
- Submit pull requests

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Authors

- oumouma
- shayo victor
- Elijah Gathanga

## Acknowledgments

- Thanks to George Chaikin for the original algorithm (1974)
- Built with [Macroquad](https://github.com/not-fl3/macroquad)
