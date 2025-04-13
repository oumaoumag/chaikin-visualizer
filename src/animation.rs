use macroquad::math::Vec2;
use macroquad::prelude::*;

use crate::chaikin;

/// Represents the current state of the application
pub enum AppState {
    /// User is drawing control points
    Drawing,
    /// Animation is playing
    Animating,
}

/// Manages the animation and state of the application
pub struct AnimationManager {
    /// Control points drawn by the user
    pub points: Vec<Vec2>,
    /// Current application state
    pub state: AppState,
    /// All animation steps generated by Chaikin's algorithm
    pub animation_steps: Vec<Vec<Vec2>>,
    /// Current step being displayed
    pub current_step: usize,
    /// Timer for animation transitions
    pub animation_timer: f32,
    /// Speed of animation in seconds per step
    pub animation_speed: f32,
}

impl AnimationManager {
    /// Creates a new animation manager with default values
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            state: AppState::Drawing,
            animation_steps: Vec::new(),
            current_step: 0,
            animation_timer: 0.0,
            animation_speed: 0.5,
        }
    }

    /// Adds a point to the list of control points
    pub fn add_point(&mut self, position: Vec2) {
        self.points.push(position);
    }

    /// Starts the animation by calculating all steps of Chaikin's algorithm
    pub fn start_animation(&mut self) {
        if self.points.len() >= 3 {
            self.animation_steps = chaikin::apply_chaikin(&self.points, 7, 0.25);
            self.current_step = 0;
            self.animation_timer = 0.0;
            self.state = AppState::Animating;
        }
    }

    /// Updates the animation state
    pub fn update(&mut self, dt: f32) {
        match self.state {
            AppState::Drawing => {}
            AppState::Animating => {
                self.animation_timer += dt;

                if self.animation_timer >= self.animation_speed {
                    self.animation_timer = 0.0;
                    self.current_step = (self.current_step + 1) % self.animation_steps.len();
                }
            }
        }
    }

    /// Draws the current state of the application
    pub fn draw(&self) {
        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));

        match self.state {
            AppState::Drawing => {
                for point in &self.points {
                    draw_circle(point.x, point.y, 5.0, RED);
                }

                if self.points.len() >= 2 {
                    for i in 0..self.points.len() - 1 {
                        draw_line(
                            self.points[i].x,
                            self.points[i].y,
                            self.points[i + 1].x,
                            self.points[i + 1].y,
                            2.0,
                            WHITE,
                        );
                    }

                    if self.points.len() >= 3 {
                        let last = self.points.len() - 1;
                        draw_line(
                            self.points[last].x,
                            self.points[last].y,
                            self.points[0].x,
                            self.points[0].y,
                            2.0,
                            WHITE,
                        );
                    }
                }

                draw_text(
                    "Click to add points. Press Enter to start animation. Press Escape to quit.",
                    20.0,
                    20.0,
                    20.0,
                    WHITE,
                );
            }
            AppState::Animating => {
                let current_points = &self.animation_steps[self.current_step];

                for point in current_points {
                    draw_circle(point.x, point.y, 3.0, BLUE);
                }

                if current_points.len() >= 2 {
                    for i in 0..current_points.len() - 1 {
                        draw_line(
                            current_points[i].x,
                            current_points[i].y,
                            current_points[i + 1].x,
                            current_points[i + 1].y,
                            2.0,
                            GREEN,
                        );
                    }

                    let last = current_points.len() - 1;
                    draw_line(
                        current_points[last].x,
                        current_points[last].y,
                        current_points[0].x,
                        current_points[0].y,
                        2.0,
                        GREEN,
                    );
                }

                draw_text(
                    &format!("Step: {}/{}", self.current_step, self.animation_steps.len() - 1),
                    20.0,
                    20.0,
                    20.0,
                    WHITE,
                );

                draw_text(
                    "Press Escape to quit.",
                    20.0,
                    50.0,
                    20.0,
                    WHITE,
                );
            }
        }
    }

    pub fn reset(&mut self) {
        self.points.clear();
        self.animation_steps.clear();
        self.current_step = 0;
        self.animation_timer = 0.0;
        self.state = AppState::Drawing;
    }
}