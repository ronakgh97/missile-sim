use macroquad::prelude::*;
use missile_sim::core::{State3D, calculate_closing_speed, calculate_los_rate};
use missile_sim::entity::{Missile, Target};
use missile_sim::guidance::*;
use nalgebra::Vector3;

const HIT_THRESHOLD: f64 = 15.0;
const MISSILE_SPEED: f64 = 250.0;
const MISSILE_ACCEL: f64 = 500.0;
const NAV: f64 = 4.0;
const MAX_CLOSING: f64 = 2250.0;
const TRAIL_LEN: usize = 200;
const HIT_FREEZE: f32 = 2.0;
const LERP: f32 = 0.07;
const VEL_SCALE: f64 = 0.4;
const ACCEL_SCALE: f64 = 0.2;

const PALETTE: [Color; 6] = [
    Color::new(0.95, 0.25, 0.25, 1.0),
    Color::new(0.95, 0.65, 0.15, 1.0),
    Color::new(0.95, 0.95, 0.20, 1.0),
    Color::new(0.20, 0.90, 0.90, 1.0),
    Color::new(0.30, 0.45, 0.95, 1.0),
    Color::new(0.80, 0.25, 0.90, 1.0),
];

struct TrackedMissile {
    missile: Missile,
    trail: Vec<[f32; 2]>,
    age: f32,
    color: Color,
    accel_cmd: Vector3<f64>,
    hit: bool,
    dead: bool,
    hit_timer: f32,
}

#[inline(always)]
fn frand() -> f32 {
    rand::gen_range(0, u32::MAX) as f32 * (1.0 / u32::MAX as f32)
}

#[inline(always)]
fn spawn_missile(tx: f32, ty: f32, color: Color) -> TrackedMissile {
    let sw = screen_width();
    let sh = screen_height();
    let edge = rand::gen_range(0, 4);
    let (mx, my) = match edge {
        0 => (frand() * sw, 0.0),
        1 => (frand() * sw, sh),
        2 => (0.0, frand() * sh),
        _ => (sw, frand() * sh),
    };
    let dx = (tx - mx) as f64;
    let dy = (ty - my) as f64;
    let len = (dx * dx + dy * dy).sqrt();
    let (vx, vy) = if len > 0.0 {
        (dx / len * MISSILE_SPEED, dy / len * MISSILE_SPEED)
    } else {
        (0.0, MISSILE_SPEED)
    };
    TrackedMissile {
        missile: Missile {
            state: State3D {
                position: Vector3::new(mx as f64, my as f64, 0.0),
                velocity: Vector3::new(vx, vy, 0.0),
            },
            max_acceleration: MISSILE_ACCEL,
            navigation_constant: NAV,
            max_closing_speed: MAX_CLOSING,
        },
        trail: Vec::with_capacity(TRAIL_LEN),
        age: 0.0,
        color,
        accel_cmd: Vector3::zeros(),
        hit: false,
        dead: false,
        hit_timer: 0.0,
    }
}

#[macroquad::main("Missile Guidance Debug")]
async fn main() {
    let laws: [&dyn GuidanceLaw; 5] = [
        &PureProportionalNavigation,
        &TrueProportionalNavigation,
        &AugmentedProportionalNavigation::new(1.115),
        &PurePursuit,
        &LeadPursuit::new(1.155),
    ];
    let law_names = ["PPN", "TPN", "APN", "PP", "LP"];

    let mut law_idx = 0usize;
    let mut target = vec2(screen_width() / 2.0, screen_height() / 2.0);
    let mut missiles: Vec<TrackedMissile> = Vec::new();
    let mut color_idx = 0usize;
    let mut show_trails = true;
    let mut show_vel = true;
    let mut show_accel = true;
    let mut fired = 0u32;
    let mut hits = 0u32;
    let mut misses = 0u32;

    loop {
        let dt = get_frame_time() as f64;
        let sw = screen_width();
        let sh = screen_height();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::Tab) {
            law_idx = (law_idx + 1) % laws.len();
        }
        if is_key_pressed(KeyCode::Key1) {
            law_idx = 0;
        }
        if is_key_pressed(KeyCode::Key2) {
            law_idx = 1;
        }
        if is_key_pressed(KeyCode::Key3) {
            law_idx = 2;
        }
        if is_key_pressed(KeyCode::Key4) {
            law_idx = 3;
        }
        if is_key_pressed(KeyCode::Key5) {
            law_idx = 4;
        }
        if is_key_pressed(KeyCode::T) {
            show_trails = !show_trails;
        }
        if is_key_pressed(KeyCode::V) {
            show_vel = !show_vel;
        }
        if is_key_pressed(KeyCode::A) {
            show_accel = !show_accel;
        }
        if is_key_pressed(KeyCode::R) {
            missiles.clear();
            fired = 0;
            hits = 0;
            misses = 0;
            color_idx = 0;
        }
        if is_key_pressed(KeyCode::Space) {
            let c = PALETTE[color_idx % PALETTE.len()];
            missiles.push(spawn_missile(target.x, target.y, c));
            color_idx += 1;
            fired += 1;
        }

        let (mx, my) = mouse_position();
        let prev = target;
        target.x += (mx - target.x) * LERP;
        target.y += (my - target.y) * LERP;
        let tvx = (target.x - prev.x) / dt as f32;
        let tvy = (target.y - prev.y) / dt as f32;

        let tgt = Target {
            state: State3D {
                position: Vector3::new(target.x as f64, target.y as f64, 0.0),
                velocity: Vector3::new(tvx as f64, tvy as f64, 0.0),
            },
            acceleration: Vector3::zeros(),
        };

        let law = laws[law_idx];
        let sf = sw as f64;
        let shf = sh as f64;

        // Update missiles
        for m in &mut missiles {
            if m.hit {
                m.hit_timer -= dt as f32;
                continue;
            }
            if m.dead {
                continue;
            }

            let accel = law.calculate_acceleration(&m.missile, &tgt);
            m.accel_cmd = accel;
            m.missile.update(accel, dt);
            m.age += dt as f32;

            let p = &m.missile.state.position;
            m.trail.push([p.x as f32, p.y as f32]);
            if m.trail.len() > TRAIL_LEN {
                m.trail.remove(0);
            }

            let dist = (p - tgt.state.position).norm();
            if dist < HIT_THRESHOLD {
                m.hit = true;
                m.hit_timer = HIT_FREEZE;
                hits += 1;
                continue;
            }

            if p.x < -500.0 || p.x > sf + 500.0 || p.y < -500.0 || p.y > shf + 500.0 {
                m.dead = true;
                misses += 1;
            }
        }

        missiles.retain(|m| {
            if m.hit {
                return m.hit_timer > 0.0;
            }
            !m.dead
        });

        clear_background(Color::new(0.08, 0.08, 0.12, 1.0));

        // Grid
        let grid = Color::new(0.13, 0.13, 0.18, 1.0);
        let mut gx = 0.0;
        while gx < sw {
            draw_line(gx, 0.0, gx, sh, 1.0, grid);
            gx += 100.0;
        }
        let mut gy = 0.0;
        while gy < sh {
            draw_line(0.0, gy, sw, gy, 1.0, grid);
            gy += 100.0;
        }

        // Trails
        if show_trails {
            for m in &missiles {
                let n = m.trail.len();
                if n < 2 {
                    continue;
                }
                for i in 1..n {
                    let t = i as f32 / n as f32;
                    let mut c = m.color;
                    c.a = t * 0.6;
                    let w = if m.hit { 1.0 } else { 2.0 };
                    draw_line(
                        m.trail[i - 1][0],
                        m.trail[i - 1][1],
                        m.trail[i][0],
                        m.trail[i][1],
                        w,
                        c,
                    );
                }
            }
        }

        // Missiles
        for m in &missiles {
            let p = m.missile.state.position;
            let (px, py) = (p.x as f32, p.y as f32);

            if m.hit {
                let progress = 1.0 - (m.hit_timer / HIT_FREEZE);
                let r = 10.0 + progress * 40.0;
                let mut c = WHITE;
                c.a = 1.0 - progress;
                draw_circle_lines(px, py, r, 2.0, c);
                draw_circle(px, py, 4.0, WHITE);
            } else {
                draw_circle(px, py, 5.0, m.color);

                if show_vel {
                    let v = m.missile.state.velocity;
                    let spd = v.norm();
                    if spd > 1.0 {
                        let len = (spd * VEL_SCALE).min(150.0) as f32;
                        draw_line(
                            px,
                            py,
                            px + (v.x / spd * len as f64) as f32,
                            py + (v.y / spd * len as f64) as f32,
                            2.0,
                            YELLOW,
                        );
                    }
                }

                if show_accel {
                    let a = m.accel_cmd;
                    let mag = a.norm();
                    if mag > 1.0 {
                        let len = (mag * ACCEL_SCALE).min(100.0) as f32;
                        draw_line(
                            px,
                            py,
                            px + (a.x / mag * len as f64) as f32,
                            py + (a.y / mag * len as f64) as f32,
                            2.0,
                            RED,
                        );
                    }
                }
            }
        }

        // Target
        draw_circle_lines(target.x, target.y, 15.0, 3.0, GREEN);
        draw_circle(target.x, target.y, 4.0, GREEN);
        let tv = tgt.state.velocity;
        let tv_norm = tv.norm();
        if tv_norm > 1.0 {
            let len = (tv_norm * VEL_SCALE).min(100.0) as f32;
            let mut c = GREEN;
            c.a = 0.5;
            draw_line(
                target.x,
                target.y,
                target.x + (tv.x / tv_norm * len as f64) as f32,
                target.y + (tv.y / tv_norm * len as f64) as f32,
                2.0,
                c,
            );
        }

        // HUD
        let fs = 18.0;
        let gap = fs + 6.0;

        // Top-left: stats
        draw_text(
            format!("Law [TAB/1-6]: {}", law_names[law_idx]),
            10.0,
            gap,
            fs,
            WHITE,
        );
        draw_text(
            format!("Fired: {}   Hits: {}   Evaded: {}", fired, hits, misses),
            10.0,
            gap * 2.0,
            fs,
            WHITE,
        );
        draw_text(
            format!(
                "Active: {}",
                missiles.iter().filter(|m| !m.hit && !m.dead).count()
            ),
            10.0,
            gap * 3.0,
            fs,
            WHITE,
        );

        // Top-right: metrics panel
        let pw = 260.0;
        let ph = 190.0;
        let px = sw - pw - 10.0;
        let py = 10.0;
        draw_rectangle(
            px - 5.0,
            py - 5.0,
            pw + 10.0,
            ph,
            Color::new(0.0, 0.0, 0.0, 0.6),
        );
        draw_rectangle_lines(
            px - 5.0,
            py - 5.0,
            pw + 10.0,
            ph,
            1.0,
            Color::new(0.3, 0.3, 0.3, 1.0),
        );

        let displayed = missiles
            .iter()
            .rfind(|m| !m.hit && !m.dead)
            .or_else(|| missiles.last());
        if let Some(m) = displayed {
            let p = m.missile.state.position;
            let dist = (p - tgt.state.position).norm();
            let closing = calculate_closing_speed(
                &p,
                &m.missile.state.velocity,
                &tgt.state.position,
                &tgt.state.velocity,
            );
            let los = calculate_los_rate(
                &p,
                &m.missile.state.velocity,
                &tgt.state.position,
                &tgt.state.velocity,
            )
            .norm();
            let speed = m.missile.state.speed();
            let a_mag = m.accel_cmd.norm();

            draw_text("MISSILE METRICS", px, py + 16.0, 20.0, WHITE);
            draw_text(
                format!("Distance:    {:.1} m", dist),
                px,
                py + 44.0,
                fs,
                WHITE,
            );
            draw_text(
                format!("Speed:       {:.1} m/s", speed),
                px,
                py + 66.0,
                fs,
                WHITE,
            );
            draw_text(
                format!("Closing V:   {:.1} m/s", closing),
                px,
                py + 88.0,
                fs,
                WHITE,
            );
            draw_text(
                format!("Accel cmd:   {:.1} m/s²", a_mag),
                px,
                py + 110.0,
                fs,
                WHITE,
            );
            draw_text(
                format!("LOS rate:    {:.4} rad/s", los),
                px,
                py + 132.0,
                fs,
                WHITE,
            );
            draw_text(
                format!("Nav const N: {:.1}", m.missile.navigation_constant),
                px,
                py + 154.0,
                fs,
                WHITE,
            );
            draw_text(
                format!("Age:         {:.1} s", m.age),
                px,
                py + 176.0,
                fs,
                WHITE,
            );
        } else {
            draw_text("MISSILE METRICS", px, py + 16.0, 20.0, WHITE);
            draw_text(
                "No missiles launched",
                px,
                py + 44.0,
                fs,
                Color::new(0.5, 0.5, 0.5, 1.0),
            );
        }

        // Bottom-left: controls
        let cy = sh - 100.0;
        let ctrl = Color::new(0.6, 0.6, 0.6, 1.0);
        draw_text("[SPACE]  Fire missile", 10.0, cy, fs, ctrl);
        draw_text("[TAB/1-6]  Select guidance law", 10.0, cy + gap, fs, ctrl);
        draw_text(
            "[T] Trails   [V] Velocity   [A] Accel",
            10.0,
            cy + gap * 2.0,
            fs,
            ctrl,
        );
        draw_text("[R]  Clear all", 10.0, cy + gap * 3.0, fs, ctrl);

        next_frame().await;
    }
}
