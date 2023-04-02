use cgmath::*;
use macroquad::prelude::*;

#[macroquad::main("PUBG 2D")]
async fn main() {
    let player_speed: f32 = 0.1;

    // TODO: replace with structure of vector2, health, ammo, etc
    let mut player_health: i32 = 100;
    let mut player_ammo: i32 = 60;

    let mut player_target: Vector2<f32> = Vector2::new(100.0, screen_height() - 50.0);
    let mut player: Vector2<f32> = Vector2::new(player_target.x, player_target.y);
    let mut enemy: Vector2<f32> = Vector2::new(screen_width() - 50.0, 100.0);

    loop {
        clear_background(BLACK);

        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        draw_text("PLAYER 1", 20.0, 20.0, 20.0, LIGHTGRAY);
        draw_text("- HEALTH", 20.0, 40.0, 20.0, LIGHTGRAY);
        draw_text("- AMMO  ", 20.0, 60.0, 20.0, LIGHTGRAY);

        let health_text = player_health.to_string();
        let ammo_text = player_ammo.to_string();

        draw_text(&health_text, 110.0, 40.0, 20.0, LIGHTGRAY);
        draw_text(&ammo_text, 110.0, 60.0, 20.0, LIGHTGRAY);

        if is_key_pressed(KeyCode::H) {
            player_health -= 1;
        }

        if is_key_pressed(KeyCode::Space) {
            player_ammo -= 1;
        }

        // Players
        let player_direction: Vector2<f32> = player_target - player;
        if player_direction.x != 0.0 || player_direction.y != 0.0 {
            player += player_direction.normalize() * player_speed;
        }

        draw_circle(player.x, player.y, 5.0, WHITE);
        draw_line(
            player_target.x - 5.0,
            player_target.y - 5.0,
            player_target.x + 5.0,
            player_target.y + 5.0,
            1.0,
            DARKGRAY,
        );
        draw_line(
            player_target.x - 5.0,
            player_target.y + 5.0,
            player_target.x + 5.0,
            player_target.y - 5.0,
            1.0,
            DARKGRAY,
        );

        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_position = mouse_position();
            player_target.x = mouse_position.0;
            player_target.y = mouse_position.1;
        }

        // Enemies
        let enemy_direction: Vector2<f32> = player - enemy;
        if enemy_direction.x != 0.0 || player_direction.y != 0.0 {
            enemy += enemy_direction.normalize() * player_speed;
        }

        draw_circle(enemy.x, enemy.y, 5.0, GRAY);

        // UI
        if is_key_pressed(KeyCode::Q) {
            break;
        }

        next_frame().await
    }
}
