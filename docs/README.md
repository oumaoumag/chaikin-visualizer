# Chaikin's Algorithm Visualizer Documentation

Welcome to the documentation for the Chaikin's Algorithm Visualizer!

## Contents

- [Usage Guide](usage_guide.md) - How to use the application
- [Chaikin's Algorithm](chaikin_algorithm.md) - How the algorithm works
- [Animation Module](animation.md) - Details about the animation system
- [Chaikin Module](chaikin.md) - Details about the algorithm implementation

## What is Chaikin's Algorithm?

Chaikin's Algorithm is a simple way to create smooth curves from a set of points. It works by repeatedly "cutting the corners" of a shape until it becomes smooth.

## Features

- Draw points by clicking on the screen
- Move points by dragging them
- Watch the smoothing process step by step
- Pause and resume the animation
- Reset and try different shapes

## Code Structure

The application has three main parts:

1. **Main Module** (`main.rs`) - Handles user input and window setup
2. **Animation Module** (`animation.rs`) - Controls the visual display and state
3. **Chaikin Module** (`chaikin.rs`) - Implements the smoothing algorithm

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.
