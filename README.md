# Chaikin's Algorithm Visualization

This project is an interactive visualization of Chaikin's algorithm, a curve subdivision algorithm that smooths a polyline or polygon by "cutting corners" - replacing each corner with two new points.

## About Chaikin's Algorithm

Chaikin's algorithm is a simple yet powerful method for curve subdivision. It works by:

1. Taking a set of points that form a polyline or polygon
2. For each pair of consecutive points, creating two new points that are positioned at 1/4 and 3/4 along the line segment
3. Replacing the original points with these new points
4. Repeating the process to achieve smoother curves

With each iteration, the curve becomes smoother and approaches a quadratic B-spline.

## Features

- Interactive point placement with mouse clicks
- Visualization of the original polyline/polygon
- Animation of Chaikin's algorithm through 7 iterations
- Special handling for cases with fewer than 3 points
- Simple UI with instructions
- Reset functionality

## Implementation

This project is implemented in Rust using the macroquad graphics library. The code is organized into three main modules:

- `src/main.rs`: Main application entry point and event loop
- `src/chaikin.rs`: Implementation of Chaikin's algorithm
- `src/animation.rs`: Animation system and state management

### Key Components

- **Point Management**: Points are stored as Vec2 objects and can be added via mouse clicks
- **Chaikin's Algorithm**: Implemented with a configurable ratio parameter (set to 0.25)
- **Animation System**: Uses a state machine to manage drawing and animation states
- **User Interface**: Simple text instructions and visual feedback

## How to Run

1. Make sure you have Rust and Cargo installed on your system
2. Clone this repository
3. Run the application with:

```bash
cargo run
```

## How to Use

1. **Drawing Mode**:
   - Click anywhere on the canvas to add control points (at least 3 for a proper polygon)
   - Press Enter to start the animation
   - Press Escape to quit the application
   - Press R to reset and clear all points

2. **Animation Mode**:
   - The animation will automatically cycle through 7 steps of Chaikin's algorithm
   - Each step shows the result of applying the algorithm one more time
   - Press Escape to quit the application

## Technical Details

- The algorithm uses a ratio of 0.25 for corner cutting
- The animation cycles through 7 iterations of the algorithm
- The application supports both open and closed curves
- The animation speed is configurable (currently set to 0.5 seconds per step)

## Project Structure

- `Cargo.toml`: Project configuration and dependencies
- `src/main.rs`: Main application entry point and loop
- `src/chaikin.rs`: Implementation of Chaikin's algorithm
- `src/animation.rs`: Animation system and state management
