//! Interactive prototype: renders the simulation with macroquad.
//!
//! This is the only place that knows about graphics. Run it with:
//!
//! ```text
//! cargo run --bin prototype --features prototype
//! ```

use ace_simulator as sim;
use macroquad::prelude::*;

const WORLD_MIN_X: f32 = -200.0;
const WORLD_MAX_X: f32 = 200.0;
const WORLD_MIN_Y: f32 = -150.0;
const WORLD_MAX_Y: f32 = 150.0;

const TARGETING_BUTTON: (f32, f32, f32, f32) = (12.0, 12.0, 260.0, 34.0);
const REVERSE_BUTTON: (f32, f32, f32, f32) = (12.0, 54.0, 260.0, 34.0);

/// How long a detonation flash stays on screen. Renderer-only: the simulation's
/// explosion projectile lives a single tick.
const BLAST_DURATION: f32 = 0.25;

/// A renderer-only explosion flash, aged by frame time.
struct Blast {
    position: sim::Vec2,
    remaining: f32,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "ace prototype".to_owned(),
        window_width: 960,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut world = sim::World::new(sim::SimConfig::default());

    // The ace is placed at (0, -37) with an initial movement offset of (30, 0).
    let mut ace = sim::Ace::new(sim::Vec2::new(0.0, 37.0), 0.0);
    ace.position = sim::Vec2::new(30.0, 0.0);
    ace.on_path = false;
    world.ace = Some(ace);

    // A ZOMG enters on the first frame.
    world.schedule_spawn(0, sim::EnemyType::ZOMG, sim::PathId::Indexed(0));

    let mut blasts: Vec<Blast> = Vec::new();

    loop {
        handle_input(&mut world);
        world.simulate();
        update_blasts(&world, &mut blasts, get_frame_time());

        clear_background(WHITE);
        draw_track(&world);
        draw_ace_center(&world);
        draw_bloons(&world);
        draw_projectiles(&world);
        draw_blasts(&blasts);
        draw_target(&world);
        draw_ace(&world);
        draw_buttons(&world);
        draw_debug(&world);

        next_frame().await;
    }
}

/// Read the on-screen buttons and mutate the ace accordingly.
fn handle_input(world: &mut sim::World) {
    let targeting_clicked = clicked(TARGETING_BUTTON);
    let reverse_clicked = clicked(REVERSE_BUTTON);

    let Some(ace) = world.ace.as_mut() else {
        return;
    };

    if targeting_clicked {
        ace.targeting = next_targeting(ace.targeting);
        ace.on_path = false;
    }

    if reverse_clicked {
        if ace.reverse {
            ace.reverse = false;
            ace.phase += std::f32::consts::PI / 5.0;
        } else {
            ace.reverse = true;
            ace.phase -= std::f32::consts::PI / 5.0;
        }
        ace.on_path = false;
    }
}

fn next_targeting(targeting: sim::Targeting) -> sim::Targeting {
    match targeting {
        sim::Targeting::Circle => sim::Targeting::FigureInfinite,
        sim::Targeting::FigureInfinite => sim::Targeting::FigureEight,
        sim::Targeting::FigureEight => sim::Targeting::Circle,
    }
}

fn clicked(rect: (f32, f32, f32, f32)) -> bool {
    if !is_mouse_button_pressed(MouseButton::Left) {
        return false;
    }
    let (mouse_x, mouse_y) = mouse_position();
    mouse_x >= rect.0
        && mouse_x <= rect.0 + rect.2
        && mouse_y >= rect.1
        && mouse_y <= rect.1 + rect.3
}

/// The world-to-screen scale and origin, fitted to the window.
fn view() -> (f32, f32, f32) {
    let scale = (screen_width() / (WORLD_MAX_X - WORLD_MIN_X))
        .min(screen_height() / (WORLD_MAX_Y - WORLD_MIN_Y));
    let width = (WORLD_MAX_X - WORLD_MIN_X) * scale;
    let height = (WORLD_MAX_Y - WORLD_MIN_Y) * scale;
    (
        scale,
        (screen_width() - width) * 0.5,
        (screen_height() - height) * 0.5,
    )
}

/// Map a world position to the screen. Increasing world `y` points down.
fn to_screen(position: sim::Vec2) -> Vec2 {
    let (scale, origin_x, origin_y) = view();
    Vec2::new(
        origin_x + (position.x - WORLD_MIN_X) * scale,
        origin_y + (position.y - WORLD_MIN_Y) * scale,
    )
}

fn world_scale() -> f32 {
    view().0
}

/// The track, sampled into a connected polyline.
fn draw_track(world: &sim::World) {
    let path = world.paths.get(sim::PathId::Indexed(0));
    let length = path.length();
    let step = 4.0;
    let mut distance = 0.0;
    while distance < length {
        let from = to_screen(path.sample(distance).0);
        let to = to_screen(path.sample((distance + step).min(length)).0);
        draw_line(
            from.x,
            from.y,
            to.x,
            to.y,
            2.0,
            Color::new(0.6, 0.6, 0.6, 1.0),
        );
        distance += step;
    }
}

fn bloon_color(kind: sim::EnemyType) -> Color {
    match kind {
        sim::EnemyType::Red => RED,
        sim::EnemyType::Ceramic => BROWN,
        sim::EnemyType::MOAB => BLUE,
        sim::EnemyType::BFB => RED,
        sim::EnemyType::ZOMG => GREEN,
        sim::EnemyType::DDT => BLACK,
        sim::EnemyType::BAD => PURPLE,
        sim::EnemyType::CeramicFortified => DARKBROWN,
        sim::EnemyType::MOABFortified => SKYBLUE,
        sim::EnemyType::BFBFortified => MAROON,
        sim::EnemyType::ZOMGFortified => DARKGREEN,
        sim::EnemyType::DDTFortified => DARKGRAY,
        sim::EnemyType::BADFortified => DARKPURPLE,
    }
}

/// Bloons are drawn as their hitbox circles.
fn draw_bloons(world: &sim::World) {
    let scale = world_scale();
    for enemy in &world.enemies {
        let color = bloon_color(enemy.kind);
        for circle in sim::enemy_stats(enemy.kind).hitbox.circles() {
            let center = to_screen(circle.world_center(enemy.position, enemy.heading));
            draw_circle(center.x, center.y, circle.radius * scale, color);
        }
    }
}

fn draw_projectiles(world: &sim::World) {
    for projectile in &world.projectiles {
        // Projectiles that have spent all their pierce are pending removal.
        if projectile.is_spent() {
            continue;
        }
        let direction = Vec2::new(projectile.heading.cos(), projectile.heading.sin());
        match projectile.kind {
            sim::ProjectileType::Dart => {
                let from = to_screen(projectile.position);
                let tip = to_screen(
                    projectile.position + sim::Vec2::new(direction.x * 8.0, direction.y * 8.0),
                );
                draw_line(from.x, from.y, tip.x, tip.y, 2.0, BLACK);
            }
            sim::ProjectileType::Missile => {
                let center = to_screen(projectile.position);
                draw_rectangle_ex(
                    center.x,
                    center.y,
                    36.0,
                    15.0,
                    DrawRectangleParams {
                        offset: vec2(0.5, 0.5),
                        rotation: projectile.heading,
                        color: GREEN,
                    },
                );
            }
            // Explosions are drawn from the renderer-only flash list so they can
            // outlive the one-tick projectile.
            sim::ProjectileType::Explosion => {}
        }
    }
}

/// Seed a flash for each explosion that appeared this tick, then age them.
fn update_blasts(world: &sim::World, blasts: &mut Vec<Blast>, dt: f32) {
    for projectile in &world.projectiles {
        if projectile.kind == sim::ProjectileType::Explosion {
            blasts.push(Blast {
                position: projectile.position,
                remaining: BLAST_DURATION,
            });
        }
    }
    blasts.retain_mut(|blast| {
        blast.remaining -= dt;
        blast.remaining > 0.0
    });
}

/// Draw the blast flashes, fading out as they age.
fn draw_blasts(blasts: &[Blast]) {
    let scale = world_scale();
    let radius = sim::projectile_stats(sim::ProjectileType::Explosion).radius * scale;
    for blast in blasts {
        let fade = (blast.remaining / BLAST_DURATION).clamp(0.0, 1.0);
        let center = to_screen(blast.position);
        draw_circle(
            center.x,
            center.y,
            radius,
            Color::new(1.0, 0.6, 0.0, 0.35 * fade),
        );
        draw_circle_lines(
            center.x,
            center.y,
            radius,
            2.0,
            Color::new(1.0, 0.4, 0.0, fade),
        );
    }
}

/// The ace's orbit center, as a square.
fn draw_ace_center(world: &sim::World) {
    let Some(ace) = &world.ace else {
        return;
    };
    let center = to_screen(ace.center);
    draw_rectangle(center.x - 5.0, center.y - 5.0, 10.0, 10.0, BLUE);
    draw_rectangle_lines(center.x - 5.0, center.y - 5.0, 10.0, 10.0, 1.5, BLACK);
}

/// The ace's current target, as a small circle.
fn draw_target(world: &sim::World) {
    let Some(ace) = &world.ace else {
        return;
    };
    // `target_pos` is relative to `center`, like `position`.
    let center = to_screen(ace.center + ace.target_pos);
    draw_circle(center.x, center.y, 5.0, ORANGE);
    draw_circle_lines(center.x, center.y, 5.0, 1.5, BLACK);
}

/// The ace as an arrow pointing along its heading.
fn draw_ace(world: &sim::World) {
    let Some(ace) = &world.ace else {
        return;
    };
    let center = to_screen(ace.world_position());
    let forward = Vec2::new(ace.heading.cos(), ace.heading.sin());
    let side = Vec2::new(-forward.y, forward.x);
    let tip = center + forward * 16.0;
    let left = center - forward * 12.0 + side * 9.0;
    let right = center - forward * 12.0 - side * 9.0;
    draw_triangle(tip, left, right, BLACK);
}

fn draw_buttons(world: &sim::World) {
    let targeting = world
        .ace
        .map(|ace| format!("{:?}", ace.targeting))
        .unwrap_or_else(|| "n/a".to_owned());
    let reverse = world.ace.map(|ace| ace.reverse).unwrap_or(false);
    draw_button(TARGETING_BUTTON, &format!("Targeting: {targeting}"));
    draw_button(REVERSE_BUTTON, &format!("Reverse: {reverse}"));
}

/// A live panel of the ace's current properties (and a few world counts).
fn draw_debug(world: &sim::World) {
    let Some(ace) = &world.ace else {
        return;
    };

    let darts = world
        .projectiles
        .iter()
        .filter(|p| p.kind == sim::ProjectileType::Dart)
        .count();
    let missiles = world
        .projectiles
        .iter()
        .filter(|p| p.kind == sim::ProjectileType::Missile)
        .count();
    let world_position = ace.world_position();

    let lines = [
        "ACE".to_owned(),
        format!("center      ({:.1}, {:.1})", ace.center.x, ace.center.y),
        format!("position    ({:.2}, {:.2})", ace.position.x, ace.position.y),
        format!(
            "target      ({:.2}, {:.2})",
            ace.target_pos.x, ace.target_pos.y
        ),
        format!(
            "world       ({:.2}, {:.2})",
            world_position.x, world_position.y
        ),
        format!(
            "heading     {:+.3} rad ({:+.1}°)",
            ace.heading,
            ace.heading.to_degrees()
        ),
        format!(
            "phase       {:.3} rad ({:.1}°)",
            ace.phase,
            ace.phase.to_degrees()
        ),
        format!("targeting   {:?}", ace.targeting),
        format!("reverse     {}", ace.reverse),
        format!("on_path     {}", ace.on_path),
        format!("dart timer  {:.2}", ace.attacks.dart_timer),
        format!("missile tim {:.2}", ace.attacks.missile_timer),
        format!("tick        {}", world.tick),
        format!("enemies     {}", world.enemies.len()),
        format!("darts       {darts}"),
        format!("missiles    {missiles}"),
    ];

    let padding = 10.0;
    let line_height = 20.0;
    let width = 300.0;
    let height = padding * 2.0 + line_height * lines.len() as f32;
    let x = screen_width() - width - 12.0;
    let y = 12.0;

    draw_rectangle(x, y, width, height, Color::new(1.0, 1.0, 1.0, 0.85));
    draw_rectangle_lines(x, y, width, height, 1.5, BLACK);
    for (index, line) in lines.iter().enumerate() {
        draw_text(
            line,
            x + padding,
            y + padding + line_height * (index as f32 + 0.8),
            18.0,
            BLACK,
        );
    }
}

fn draw_button(rect: (f32, f32, f32, f32), label: &str) {
    draw_rectangle(
        rect.0,
        rect.1,
        rect.2,
        rect.3,
        Color::new(0.9, 0.9, 0.9, 1.0),
    );
    draw_rectangle_lines(rect.0, rect.1, rect.2, rect.3, 2.0, BLACK);
    draw_text(label, rect.0 + 10.0, rect.1 + rect.3 - 10.0, 22.0, BLACK);
}
