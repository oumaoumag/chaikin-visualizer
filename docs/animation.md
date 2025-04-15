# Animation Module

## Overview

The animation module handles the state management and rendering for the Chaikin's Algorithm Visualizer. It controls how points are drawn, manipulated, and animated.

## Key Components

### AppState

The application can be in one of three states:

- **Drawing**: User is adding and adjusting control points
- **Animating**: The smoothing animation is playing
- **Paused**: The animation is paused

### AnimationManager

This is the main class that manages the application state:

- Stores and manages control points
- Handles user interactions (adding/dragging points)
- Controls the animation playback
- Renders the current state to the screen

## Main Functions

- **add_point**: Adds a new control point
- **start_animation**: Calculates all animation steps and begins playback
- **toggle_animation_pause**: Pauses or resumes the animation
- **update**: Advances the animation based on elapsed time
- **draw**: Renders the current state to the screen
- **reset**: Clears all points and returns to drawing mode

## Usage Example

```rust
// Create a new animation manager
let mut animation_manager = AnimationManager::new();

// Add some points
animation_manager.add_point(Vec2::new(100.0, 100.0));
animation_manager.add_point(Vec2::new(200.0, 150.0));
animation_manager.add_point(Vec2::new(150.0, 250.0));

// Start the animation
animation_manager.start_animation();

// In the main loop
animation_manager.update(delta_time);
animation_manager.draw();
```
