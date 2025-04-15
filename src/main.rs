mod animation;
mod chaikin;

use macroquad::prelude::*;
use macroquad::math::Vec2;

use animation::AnimationManager;
use animation::AppState;

#[macroquad::main("Chaikin's Algorithm")]
async fn main() {
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
