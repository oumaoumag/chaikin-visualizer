#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use macroquad::prelude::{
    Vec2,
    mouse_position,
    is_mouse_button_pressed,
    is_mouse_button_down,
    is_mouse_button_released,
    MouseButton,
    is_key_pressed,
    KeyCode,
    get_frame_time,
    next_frame,
};

use crate::animation::{AnimationManager, AppState};

// Re-export our modules
pub mod animation;
pub mod chaikin;


#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub async fn start() {
    run_app().await;
}

pub async fn run_app() {
    let mut animation_manager = AnimationManager::new();

    loop {
        let mouse_position = Vec2::new(mouse_position().0, mouse_position().1);

        if matches!(animation_manager.state, AppState::Drawing) {
            if is_mouse_button_pressed(MouseButton::Left) {
                animation_manager.start_dragging(mouse_position);

                // Add a new point if we're not starting to drag an existing one
                if animation_manager.dragging_point_index.is_none() {
                    animation_manager.add_point(mouse_position);
                }
            }

            if is_mouse_button_down(MouseButton::Left) {
                animation_manager.update_dragging(mouse_position);
            }

            if is_mouse_button_released(MouseButton::Left) {
                animation_manager.stop_dragging();
            }
        }

        if is_key_pressed(KeyCode::Space) {
            animation_manager.toggle_animation_pause();
        }

        if is_key_pressed(KeyCode::Enter) && matches!(animation_manager.state, AppState::Drawing) {
            animation_manager.start_animation();
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::R) {
            animation_manager.reset();
        }

        animation_manager.update(get_frame_time());
        animation_manager.draw();
        next_frame().await;
    }
}