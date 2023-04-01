use macroquad::prelude::*;
use cgmath::*;

#[macroquad::main("PUBG 2D")]
async fn main() {
    let player_speed:f32 = 0.1;

    let mut player_health: i32 = 100;
    let mut player_ammo: i32 = 60;

    // TODO: replace with structure of vector2, health, ammo, etc
    let mut player_target: Vector2<f32> = Vector2::new(100.0, screen_height() - 50.0);
    let mut player_x: f32 = player_target.x;
    let mut player_y: f32 = player_target.y;

    let mut enemy_x: f32 = screen_width() - 50.0;
    let mut enemy_y: f32 = 100.0;

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

        if is_key_pressed(KeyCode::H){
            player_health -= 1;
        }

        if is_key_pressed(KeyCode::Space){
            player_ammo -= 1;
        }

        // Players
        if player_target.x > player_x {
            player_x += player_speed;
        } else if player_target.x < player_x {
            player_x -= player_speed;
        }
        if player_target.y > player_y {
            player_y += player_speed;
        } else if player_target.y < player_y {
            player_y -= player_speed;
        }

        draw_circle(player_x, player_y, 5.0, WHITE);

        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_position = mouse_position();
            draw_circle(mouse_position.0, mouse_position.1, 5.0, RED);
            player_target.x = mouse_position.0;
            player_target.y = mouse_position.1;
        }

        // Enemies
        if enemy_x > player_x {
            enemy_x -= player_speed;
        } else if enemy_x < player_x {
            enemy_x += player_speed;
        }
        if enemy_y > player_y {
            enemy_y -= player_speed;
        } else if enemy_y < player_y {
            enemy_y += player_speed;
        }
        // TODO: Normalize the x/y directions

        draw_circle(enemy_x, enemy_y, 5.0, GRAY);

        // UI
        if is_key_pressed(KeyCode::Q){
            break;
        }

        next_frame().await
    }
}
