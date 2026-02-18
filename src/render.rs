use std::f32::consts::PI;
use std::io::{Stdout, Write};

use crossterm::cursor::MoveTo;
use crossterm::queue;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};

use crate::game::{terrain_char, terrain_color, Game, HEIGHT, WIDTH};

pub fn draw(stdout: &mut Stdout, game: &Game) -> std::io::Result<()> {
    queue!(stdout, MoveTo(0, 0), Clear(ClearType::All))?;

    if game.on_green() {
        draw_zoomed_course(stdout, game)?;
    } else {
        draw_full_course(stdout, game)?;
    }

    draw_hud(stdout, game)?;
    queue!(stdout, ResetColor)?;
    stdout.flush()?;
    Ok(())
}

fn draw_full_course(stdout: &mut Stdout, game: &Game) -> std::io::Result<()> {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            draw_tile(stdout, x, y, x, y)?;
        }
    }
    draw_entities(stdout, game, 0, 0, 1)?;
    Ok(())
}

fn draw_zoomed_course(stdout: &mut Stdout, game: &Game) -> std::io::Result<()> {
    let zoom = 2_i32;
    let view_w = WIDTH / zoom;
    let view_h = HEIGHT / zoom;

    let center_x = ((game.ball.x + game.hole.x) * 0.5).round() as i32;
    let center_y = ((game.ball.y + game.hole.y) * 0.5).round() as i32;

    let left = (center_x - view_w / 2).clamp(0, WIDTH - view_w);
    let top = (center_y - view_h / 2).clamp(0, HEIGHT - view_h);

    for sy in 0..HEIGHT {
        for sx in 0..WIDTH {
            let wx = left + sx / zoom;
            let wy = top + sy / zoom;
            draw_tile(stdout, sx, sy, wx, wy)?;
        }
    }

    draw_entities(stdout, game, left, top, zoom)?;
    Ok(())
}

fn draw_tile(stdout: &mut Stdout, sx: i32, sy: i32, wx: i32, wy: i32) -> std::io::Result<()> {
    let tile = terrain_char(wx, wy);
    let color = terrain_color(wx, wy);
    queue!(
        stdout,
        MoveTo(sx as u16, sy as u16),
        SetForegroundColor(color),
        Print(tile)
    )?;
    Ok(())
}

fn draw_entities(
    stdout: &mut Stdout,
    game: &Game,
    left: i32,
    top: i32,
    zoom: i32,
) -> std::io::Result<()> {
    for (i, p) in game.trail.iter().enumerate() {
        if let Some((sx, sy)) = world_to_screen(p.x, p.y, left, top, zoom) {
            let fade = i as f32 / (game.trail.len().max(1) as f32);
            let ch = if fade < 0.34 {
                'o'
            } else if fade < 0.68 {
                '*'
            } else {
                '.'
            };
            let shade = (210.0 - fade * 130.0) as u8;
            queue!(
                stdout,
                MoveTo(sx as u16, sy as u16),
                SetForegroundColor(Color::Rgb {
                    r: shade,
                    g: shade,
                    b: shade,
                }),
                Print(ch)
            )?;
        }
    }

    if let Some((hx, hy)) = world_to_screen(game.hole.x, game.hole.y, left, top, zoom) {
        queue!(
            stdout,
            MoveTo(hx as u16, hy as u16),
            SetForegroundColor(Color::Blue),
            Print('◉')
        )?;
    }

    if let Some(air) = game.airborne {
        let ground = air.ground_pos();
        let arc = air.arc_height();
        let air_y = (ground.y - arc).max(0.0);

        if let Some((gx, gy)) = world_to_screen(ground.x, ground.y, left, top, zoom) {
            queue!(
                stdout,
                MoveTo(gx as u16, gy as u16),
                SetForegroundColor(Color::DarkGrey),
                Print('◌')
            )?;
        }

        if let Some((ax, ay)) = world_to_screen(ground.x, air_y, left, top, zoom) {
            queue!(
                stdout,
                MoveTo(ax as u16, ay as u16),
                SetForegroundColor(Color::White),
                Print('●')
            )?;
        }
    } else if let Some((bx, by)) = world_to_screen(game.ball.x, game.ball.y, left, top, zoom) {
        queue!(
            stdout,
            MoveTo(bx as u16, by as u16),
            SetForegroundColor(Color::White),
            Print('●')
        )?;
    }

    if game.can_shoot() || game.swing_active {
        draw_golfer(stdout, game, left, top, zoom)?;
    }
    if game.can_shoot() {
        let aim_len = if game.on_green() { 9 } else { 6 };
        for i in 1..=aim_len {
            let ax = game.ball.x + game.angle.cos() * i as f32;
            let ay = game.ball.y + game.angle.sin() * i as f32;
            if let Some((sx, sy)) = world_to_screen(ax, ay, left, top, zoom) {
                queue!(
                    stdout,
                    MoveTo(sx as u16, sy as u16),
                    SetForegroundColor(Color::Yellow),
                    Print('·')
                )?;
            }
        }
    }

    Ok(())
}

fn draw_golfer(
    stdout: &mut Stdout,
    game: &Game,
    left: i32,
    top: i32,
    zoom: i32,
) -> std::io::Result<()> {
    let back_x = game.golfer_anchor.x - game.angle.cos() * 1.6;
    let back_y = game.golfer_anchor.y - game.angle.sin() * 1.6;

    if let Some((hx, hy)) = world_to_screen(back_x, back_y, left, top, zoom) {
        queue!(
            stdout,
            MoveTo(hx as u16, hy as u16),
            SetForegroundColor(Color::Rgb {
                r: 240,
                g: 225,
                b: 190
            }),
            Print('●')
        )?;
    }

    if let Some((bx, by)) = world_to_screen(back_x, back_y + 0.8, left, top, zoom) {
        queue!(
            stdout,
            MoveTo(bx as u16, by as u16),
            SetForegroundColor(Color::White),
            Print('█')
        )?;
    }

    // Methodical swing path: backswing -> downswing -> follow-through.
    let phase_offsets = [1.45_f32, 1.05, 0.6, 0.2, -0.35, -0.8];
    let frame = game.swing_frame.min(phase_offsets.len() - 1);
    let club_angle = game.angle + phase_offsets[frame];
    let shaft_dx = club_angle.cos();
    let shaft_dy = club_angle.sin();

    let arm_x = back_x + game.angle.cos() * 0.45;
    let arm_y = back_y + game.angle.sin() * 0.45;
    if let Some((cx, cy)) = world_to_screen(arm_x + shaft_dx, arm_y + shaft_dy, left, top, zoom) {
        queue!(
            stdout,
            MoveTo(cx as u16, cy as u16),
            SetForegroundColor(Color::DarkGrey),
            Print('/')
        )?;
    }

    if let Some((cx2, cy2)) = world_to_screen(
        arm_x + shaft_dx * 1.8,
        arm_y + shaft_dy * 1.8,
        left,
        top,
        zoom,
    ) {
        queue!(
            stdout,
            MoveTo(cx2 as u16, cy2 as u16),
            SetForegroundColor(Color::Grey),
            Print('─')
        )?;
    }

    Ok(())
}

fn world_to_screen(wx: f32, wy: f32, left: i32, top: i32, zoom: i32) -> Option<(i32, i32)> {
    let lx = wx - left as f32;
    let ly = wy - top as f32;
    if lx < 0.0 || ly < 0.0 {
        return None;
    }

    let sx = (lx * zoom as f32).round() as i32;
    let sy = (ly * zoom as f32).round() as i32;
    if sx < 0 || sy < 0 || sx >= WIDTH || sy >= HEIGHT {
        None
    } else {
        Some((sx, sy))
    }
}

fn draw_hud(stdout: &mut Stdout, game: &Game) -> std::io::Result<()> {
    let panel_x = WIDTH as u16 + 2;

    let score = game.strokes as i32 - game.par as i32;
    let score_label = if score < 0 {
        format!("{} under", -score)
    } else if score > 0 {
        format!("{} over", score)
    } else {
        "even".to_string()
    };

    let angle_deg = (game.angle * 180.0 / PI) as i32;
    let status = if game.hole_done {
        "SUNK"
    } else if game.airborne.is_some() {
        "BALL IN AIR"
    } else if game.rolling {
        "BALL ROLLING"
    } else {
        "READY"
    };

    let dx = game.hole.x - game.ball.x;
    let dy = game.hole.y - game.ball.y;
    let to_hole_deg = dy.atan2(dx) * 180.0 / PI;
    let putt_hint = normalize_angle_deg(to_hole_deg - angle_deg as f32);

    let club = game.current_club();

    let lines = vec![
        "TERMINAL GOLF".to_string(),
        "-------------".to_string(),
        "Controls:".to_string(),
        "A/D or <-/-> : Aim (360)".to_string(),
        "W/S or ^/v    : Club +/-".to_string(),
        "E             : Swing Type".to_string(),
        "C             : Auto Caddie".to_string(),
        "Space/Enter   : Hit".to_string(),
        "R             : Restart".to_string(),
        "Q/Esc         : Quit".to_string(),
        "".to_string(),
        format!("Strokes: {}", game.strokes),
        format!("Par: {} ({})", game.par, score_label),
        format!("Distance: {:.0} yd", game.distance_to_hole_yd()),
        format!("Lie: {}", game.current_surface().name()),
        format!("Club: {}", club.name),
        format!("Shot: {}", game.selected_shot.name()),
        format!("Play: {:.0} yd", game.selected_shot_distance_yd()),
        format!(
            "Caddie: {}",
            if game.auto_caddie { "AUTO" } else { "MANUAL" }
        ),
        format!("Aim: {:+} deg", angle_deg),
        format!("Cup Dir: {:+.0} deg", to_hole_deg),
        format!("Aim Err: {:+.0} deg", putt_hint),
        format!("Wind: {:+.1} mph", game.wind * 12.0),
        format!(
            "View: {}",
            if game.on_green() {
                "GREEN ZOOM"
            } else {
                "FULL HOLE"
            }
        ),
        format!("Status: {}", status),
    ];

    for (i, line) in lines.iter().enumerate() {
        queue!(
            stdout,
            MoveTo(panel_x, i as u16),
            SetForegroundColor(Color::Cyan),
            Print(line)
        )?;
    }

    if game.hole_done {
        let msg = if game.strokes == 1 {
            "Hole in one! Press R"
        } else {
            "Hole complete. Press R"
        };

        queue!(
            stdout,
            MoveTo(panel_x, 24),
            SetForegroundColor(Color::Green),
            Print(msg)
        )?;
    }

    Ok(())
}

fn normalize_angle_deg(mut angle: f32) -> f32 {
    while angle <= -180.0 {
        angle += 360.0;
    }
    while angle > 180.0 {
        angle -= 360.0;
    }
    angle
}
