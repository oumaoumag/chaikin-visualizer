use macroquad::math::Vec2;
use macroquad::prelude::*;

use crate::chaikin;

pub enum AppState {
    Drawing,
    Animating,
}

pub struct AnimationManager {
    pub points: Vec<Vec2>,
    pub state: AppState,
    pub animation_steps: Vec<Vec<Vec2>>,
    pub current_step: usize,
    pub animation_timer: f32,
    pub animation_speed: f32,
}

impl AnimationManager {
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

    pub fn add_point(&mut self, position: Vec2) {
        self.points.push(position);
    }

    pub fn start_animation(&mut self) {
        if self.points.len() >= 3 {
            self.animation_steps = chaikin::apply_chaikin(&self.points, 7, 0.25);
            self.current_step = 0;
            self.animation_timer = 0.0;
            self.state = AppState::Animating;
        }
    }

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