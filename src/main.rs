use cgmath::*;
use macroquad::prelude::*;
use  macroquad::rand::*;

#[macroquad::main("PUBG 2D")]
async fn main() {
    let player_speed: f32 = 0.1;

    // TODO: replace with structure of vector2, health, ammo, etc
    let mut player_health: i32 = 100;
    let mut player_ammo: i32 = 60;

    let mut player_target: Vector2<f32> = Vector2::new(100.0, screen_height() - 50.0);
    let mut player: Vector2<f32> = Vector2::new(player_target.x, player_target.y);

    // https://doc.rust-lang.org/std/vec/struct.Vec.html
    let mut enemies: Vec<Enemy> = Vec::new();
    let mut bullets: Vec<Bullet> = Vec::new();

    for i in 0..20 {
        enemies.push(Enemy {
            position: Vector2::new(rand::gen_range(200.0, 500.0), 100.0 + i as f32 * 30.0),
            dead: false,
        });
    }

    loop {
        let frame_t = get_time();

        clear_background(BLACK);

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

        if is_key_pressed(KeyCode::Space) && player_ammo > 0 && enemies.len() > 0 {
            player_ammo -= 1;

            let mut closest_enemy = &enemies[0];
            let mut closest_distance = 100000.0;
            for enemy in &enemies {
                let distance = (enemy.position - player).magnitude();
                if distance < closest_distance {
                    closest_distance = distance;
                    closest_enemy = enemy;
                }
            }

            let x: f32 = gen_range(-10.0, 10.0);
            let y: f32 = gen_range(-10.0, 10.0);

            bullets.push(Bullet {
                position: player,
                direction: closest_enemy.position - player + Vector2::new(x, y),
                collided: false,
                shot_at: frame_t,
            });
        }

        for bullet in &mut bullets {
            draw_circle(bullet.position.x, bullet.position.y, 2.0, RED);
            bullet.position += bullet.direction.normalize() * 2.0;

            for enemy in &mut enemies {
                let distance_x: f32 = (enemy.position.x - bullet.position.x).abs();
                let distance_y: f32 = (enemy.position.y - bullet.position.y).abs();
                if distance_x < 5.0 && distance_y < 5.0 {
                    bullet.collided = true;
                    enemy.dead = true;
                }
            }

            if bullet.position.x < 0.0
                || bullet.position.x > screen_width()
                || bullet.position.y < 0.0
                || bullet.position.y > screen_height()
            {
                bullet.collided = true;
            }
        }
        bullets.retain(|bullet| !bullet.collided);
        bullets.retain(|bullet| bullet.shot_at + 1.5 > frame_t);
        enemies.retain(|enemy| !enemy.dead);

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
        for enemy in &mut enemies {
            let enemy_direction: Vector2<f32> = player - enemy.position;
            if enemy_direction.x != 0.0 || enemy_direction.y != 0.0 {
                enemy.position += enemy_direction.normalize() * player_speed;
            }

            draw_circle(enemy.position.x, enemy.position.y, 5.0, GRAY);
        }



        // UI
        if is_key_pressed(KeyCode::Q) {
            break;
        }

        next_frame().await
    }
}

// https://doc.rust-lang.org/book/ch05-01-defining-structs.html
struct Bullet {
    position: Vector2<f32>,
    direction: Vector2<f32>,
    collided: bool,
    shot_at: f64,
}

struct Enemy {
    position: Vector2<f32>,
    dead: bool,
}
