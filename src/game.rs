use std::f32::consts::PI;

use crossterm::style::Color;
use rand::Rng;

pub const WIDTH: i32 = 72;
pub const HEIGHT: i32 = 24;
pub const TICK_MS: u64 = 33;
pub const TRAIL_LEN: usize = 18;
pub const AIM_STEP_RAD: f32 = 0.08;
pub const YARDS_PER_TILE: f32 = 5.0;
pub const SWING_FRAMES: usize = 6;

#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalized(&self) -> Self {
        let len = self.length();
        if len < 0.0001 {
            Self::new(0.0, 0.0)
        } else {
            Self::new(self.x / len, self.y / len)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Surface {
    Green,
    Fairway,
    Rough,
    Bunker,
}

impl Surface {
    pub fn drag_strength(self) -> f32 {
        match self {
            Surface::Green => 2.35,
            Surface::Fairway => 2.0,
            Surface::Rough => 4.2,
            Surface::Bunker => 9.0,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Surface::Green => "Green",
            Surface::Fairway => "Fairway",
            Surface::Rough => "Rough",
            Surface::Bunker => "Bunker",
        }
    }
}

#[derive(Clone, Copy)]
pub struct ClubSpec {
    pub name: &'static str,
    pub carry_yd: f32,
    pub rollout_yd: f32,
    pub air_time: f32,
    pub apex: f32,
    pub dispersion: f32,
    pub putter: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ShotType {
    Full,
    ThreeQuarter,
    Half,
    Pitch,
    Chip,
}

impl ShotType {
    pub const NON_PUTTER: [ShotType; 5] = [
        ShotType::Full,
        ShotType::ThreeQuarter,
        ShotType::Half,
        ShotType::Pitch,
        ShotType::Chip,
    ];

    pub fn name(self) -> &'static str {
        match self {
            ShotType::Full => "Full",
            ShotType::ThreeQuarter => "3/4",
            ShotType::Half => "Half",
            ShotType::Pitch => "Pitch",
            ShotType::Chip => "Chip",
        }
    }

    pub fn carry_mult(self) -> f32 {
        match self {
            ShotType::Full => 1.0,
            ShotType::ThreeQuarter => 0.75,
            ShotType::Half => 0.50,
            ShotType::Pitch => 0.35,
            ShotType::Chip => 0.20,
        }
    }

    pub fn roll_mult(self) -> f32 {
        match self {
            ShotType::Full => 1.0,
            ShotType::ThreeQuarter => 0.85,
            ShotType::Half => 0.70,
            ShotType::Pitch => 0.32,
            ShotType::Chip => 0.18,
        }
    }

    pub fn arc_mult(self) -> f32 {
        match self {
            ShotType::Full => 1.0,
            ShotType::ThreeQuarter => 0.9,
            ShotType::Half => 0.75,
            ShotType::Pitch => 0.62,
            ShotType::Chip => 0.4,
        }
    }
}

pub const CLUBS: [ClubSpec; 16] = [
    ClubSpec {
        name: "Driver",
        carry_yd: 255.0,
        rollout_yd: 14.0,
        air_time: 1.0,
        apex: 4.1,
        dispersion: 0.035,
        putter: false,
    },
    ClubSpec {
        name: "3 Wood",
        carry_yd: 235.0,
        rollout_yd: 20.0,
        air_time: 0.95,
        apex: 3.8,
        dispersion: 0.032,
        putter: false,
    },
    ClubSpec {
        name: "5 Wood",
        carry_yd: 215.0,
        rollout_yd: 16.0,
        air_time: 0.90,
        apex: 3.7,
        dispersion: 0.030,
        putter: false,
    },
    ClubSpec {
        name: "3 Hybrid",
        carry_yd: 205.0,
        rollout_yd: 12.0,
        air_time: 0.85,
        apex: 3.5,
        dispersion: 0.028,
        putter: false,
    },
    ClubSpec {
        name: "4 Hybrid",
        carry_yd: 195.0,
        rollout_yd: 10.0,
        air_time: 0.82,
        apex: 3.3,
        dispersion: 0.027,
        putter: false,
    },
    ClubSpec {
        name: "4 Iron",
        carry_yd: 185.0,
        rollout_yd: 10.0,
        air_time: 0.80,
        apex: 3.2,
        dispersion: 0.026,
        putter: false,
    },
    ClubSpec {
        name: "5 Iron",
        carry_yd: 175.0,
        rollout_yd: 8.0,
        air_time: 0.78,
        apex: 3.0,
        dispersion: 0.024,
        putter: false,
    },
    ClubSpec {
        name: "6 Iron",
        carry_yd: 165.0,
        rollout_yd: 7.0,
        air_time: 0.74,
        apex: 2.8,
        dispersion: 0.022,
        putter: false,
    },
    ClubSpec {
        name: "7 Iron",
        carry_yd: 155.0,
        rollout_yd: 6.0,
        air_time: 0.70,
        apex: 2.6,
        dispersion: 0.021,
        putter: false,
    },
    ClubSpec {
        name: "8 Iron",
        carry_yd: 145.0,
        rollout_yd: 5.0,
        air_time: 0.67,
        apex: 2.4,
        dispersion: 0.019,
        putter: false,
    },
    ClubSpec {
        name: "9 Iron",
        carry_yd: 135.0,
        rollout_yd: 4.0,
        air_time: 0.64,
        apex: 2.2,
        dispersion: 0.018,
        putter: false,
    },
    ClubSpec {
        name: "Pitching Wedge",
        carry_yd: 120.0,
        rollout_yd: 3.0,
        air_time: 0.60,
        apex: 2.0,
        dispersion: 0.016,
        putter: false,
    },
    ClubSpec {
        name: "Gap Wedge",
        carry_yd: 105.0,
        rollout_yd: 2.0,
        air_time: 0.55,
        apex: 1.8,
        dispersion: 0.015,
        putter: false,
    },
    ClubSpec {
        name: "Sand Wedge",
        carry_yd: 90.0,
        rollout_yd: 1.0,
        air_time: 0.50,
        apex: 1.7,
        dispersion: 0.014,
        putter: false,
    },
    ClubSpec {
        name: "Lob Wedge",
        carry_yd: 75.0,
        rollout_yd: 1.0,
        air_time: 0.45,
        apex: 1.6,
        dispersion: 0.013,
        putter: false,
    },
    ClubSpec {
        name: "Putter",
        carry_yd: 0.0,
        rollout_yd: 32.0,
        air_time: 0.0,
        apex: 0.0,
        dispersion: 0.0035,
        putter: true,
    },
];

#[derive(Clone, Copy)]
pub struct AirState {
    pub start: Vec2,
    pub landing: Vec2,
    pub elapsed: f32,
    pub duration: f32,
    pub apex: f32,
    pub rollout_speed: f32,
}

impl AirState {
    pub fn progress(self) -> f32 {
        (self.elapsed / self.duration.max(0.001)).clamp(0.0, 1.0)
    }

    pub fn ground_pos(self) -> Vec2 {
        let t = self.progress();
        Vec2::new(
            self.start.x + (self.landing.x - self.start.x) * t,
            self.start.y + (self.landing.y - self.start.y) * t,
        )
    }

    pub fn arc_height(self) -> f32 {
        let t = self.progress();
        4.0 * self.apex * t * (1.0 - t)
    }
}

pub struct Game {
    pub ball: Vec2,
    pub velocity: Vec2,
    pub trail: Vec<Vec2>,
    pub hole: Vec2,
    pub angle: f32,
    pub selected_club_idx: usize,
    pub selected_shot: ShotType,
    pub auto_caddie: bool,
    pub strokes: u32,
    pub par: u32,
    pub hole_done: bool,
    pub rolling: bool,
    pub wind: f32,
    pub roll_time: f32,
    pub airborne: Option<AirState>,
    pub swing_frame: usize,
    pub swing_active: bool,
    swing_timer: f32,
    pub golfer_anchor: Vec2,
}

impl Game {
    pub fn new() -> Self {
        Self {
            ball: Vec2::new(8.0, (HEIGHT / 2) as f32),
            velocity: Vec2::new(0.0, 0.0),
            trail: Vec::with_capacity(TRAIL_LEN),
            hole: Vec2::new((WIDTH - 8) as f32, (HEIGHT / 2 - 5) as f32),
            angle: 0.0,
            selected_club_idx: 0,
            selected_shot: ShotType::Full,
            auto_caddie: true,
            strokes: 0,
            par: 4,
            hole_done: false,
            rolling: false,
            wind: 0.0,
            roll_time: 0.0,
            airborne: None,
            swing_frame: 0,
            swing_active: false,
            swing_timer: 0.0,
            golfer_anchor: Vec2::new(8.0, (HEIGHT / 2) as f32),
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn can_shoot(&self) -> bool {
        !self.rolling && self.airborne.is_none() && !self.hole_done
    }

    pub fn current_surface(&self) -> Surface {
        terrain_surface(self.ball.x as i32, self.ball.y as i32)
    }

    pub fn on_green(&self) -> bool {
        self.current_surface() == Surface::Green
    }

    pub fn aim_step(&self) -> f32 {
        if self.on_green() {
            AIM_STEP_RAD * 0.45
        } else {
            AIM_STEP_RAD
        }
    }

    pub fn current_club(&self) -> ClubSpec {
        CLUBS[self.selected_club_idx]
    }

    pub fn selected_shot_distance_yd(&self) -> f32 {
        let club = self.current_club();
        if club.putter {
            self.putter_rollout_target_yd(club)
        } else {
            club.carry_yd * self.selected_shot.carry_mult()
                + club.rollout_yd * self.selected_shot.roll_mult()
        }
    }

    pub fn cycle_club(&mut self, delta: i32) {
        if !self.can_shoot() {
            return;
        }
        let len = CLUBS.len() as i32;
        let mut idx = self.selected_club_idx as i32 + delta;
        if idx < 0 {
            idx += len;
        }
        if idx >= len {
            idx -= len;
        }
        self.selected_club_idx = idx as usize;
        self.selected_shot = ShotType::Full;
        self.auto_caddie = false;
    }

    pub fn cycle_shot_type(&mut self) {
        if !self.can_shoot() || self.current_club().putter {
            return;
        }
        let mut idx = ShotType::NON_PUTTER
            .iter()
            .position(|s| *s == self.selected_shot)
            .unwrap_or(0);
        idx = (idx + 1) % ShotType::NON_PUTTER.len();
        self.selected_shot = ShotType::NON_PUTTER[idx];
        self.auto_caddie = false;
    }

    pub fn toggle_auto_caddie(&mut self) {
        self.auto_caddie = !self.auto_caddie;
        if self.auto_caddie && self.can_shoot() {
            self.auto_select_shot();
        }
    }

    pub fn distance_to_hole_yd(&self) -> f32 {
        let dx = self.hole.x - self.ball.x;
        let dy = self.hole.y - self.ball.y;
        (dx * dx + dy * dy).sqrt() * YARDS_PER_TILE
    }

    pub fn update(&mut self, dt_secs: f32) {
        self.update_swing(dt_secs);

        if self.hole_done {
            return;
        }

        if let Some(mut air) = self.airborne {
            air.elapsed += dt_secs;
            if air.elapsed >= air.duration {
                self.ball = Vec2::new(
                    air.landing.x.clamp(1.0, (WIDTH - 2) as f32),
                    air.landing.y.clamp(1.0, (HEIGHT - 2) as f32),
                );
                self.airborne = None;
                let dir = Vec2::new(air.landing.x - air.start.x, air.landing.y - air.start.y)
                    .normalized();
                self.velocity = Vec2::new(
                    dir.x * air.rollout_speed + self.wind * 0.12,
                    dir.y * air.rollout_speed,
                );
                self.rolling = true;
                self.roll_time = 0.0;
            } else {
                self.airborne = Some(air);
            }
        }

        if !self.rolling {
            if self.can_shoot() && self.auto_caddie {
                self.auto_select_shot();
            }
            return;
        }

        let substeps = (dt_secs / 0.016).ceil().max(1.0) as u32;
        let step = dt_secs / substeps as f32;
        self.roll_time += dt_secs;

        for _ in 0..substeps {
            let surface = self.current_surface();
            self.ball.x += self.velocity.x * step;
            self.ball.y += self.velocity.y * step;

            let speed = self.velocity.length();
            let drag = surface.drag_strength() * step;
            if speed > 0.0001 {
                let drag_scale = (1.0 - drag).max(0.0);
                self.velocity.x *= drag_scale;
                self.velocity.y *= drag_scale;
            }

            if self.ball.x < 1.0 || self.ball.x > (WIDTH - 2) as f32 {
                self.velocity.x *= -0.35;
                self.ball.x = self.ball.x.clamp(1.0, (WIDTH - 2) as f32);
            }
            if self.ball.y < 1.0 || self.ball.y > (HEIGHT - 2) as f32 {
                self.velocity.y *= -0.35;
                self.ball.y = self.ball.y.clamp(1.0, (HEIGHT - 2) as f32);
            }

            let dx = self.ball.x - self.hole.x;
            let dy = self.ball.y - self.hole.y;
            let distance_to_hole = (dx * dx + dy * dy).sqrt();
            let now_speed = self.velocity.length();
            let on_green = self.current_surface() == Surface::Green;

            let sink_radius = if on_green { 0.56 } else { 0.42 };
            let soft_sink_radius = if on_green { 1.0 } else { 0.82 };
            let soft_sink_speed = if on_green { 1.45 } else { 1.15 };

            if distance_to_hole < sink_radius
                || (distance_to_hole < soft_sink_radius && now_speed < soft_sink_speed)
            {
                self.ball = self.hole;
                self.velocity = Vec2::new(0.0, 0.0);
                self.rolling = false;
                self.hole_done = true;
                self.roll_time = 0.0;
                break;
            }

            if distance_to_hole < 1.12 && now_speed >= soft_sink_speed {
                let nx = dx / distance_to_hole.max(0.001);
                let ny = dy / distance_to_hole.max(0.001);
                self.velocity.x = self.velocity.x * -0.2 + nx * 0.45;
                self.velocity.y = self.velocity.y * -0.2 + ny * 0.45;
            }

            if self.trail.len() >= TRAIL_LEN {
                self.trail.remove(0);
            }
            self.trail.push(self.ball);

            if now_speed < 0.12 || self.roll_time > 12.0 {
                self.velocity = Vec2::new(0.0, 0.0);
                self.rolling = false;
                self.roll_time = 0.0;
                break;
            }
        }

        if self.can_shoot() && self.auto_caddie {
            self.auto_select_shot();
        }
    }

    pub fn hit_ball(&mut self) {
        if !self.can_shoot() {
            return;
        }

        self.golfer_anchor = self.ball;
        self.start_swing_animation();

        self.strokes += 1;
        self.trail.clear();

        let mut rng = rand::thread_rng();
        self.wind = (self.wind + rng.gen_range(-0.14..0.14)).clamp(-0.5, 0.5);

        let lie = self.current_surface();
        let (lie_carry, lie_roll, lie_dispersion) = self.lie_modifiers(lie);

        let club = self.current_club();
        let shot = if club.putter {
            ShotType::Full
        } else {
            self.selected_shot
        };

        let dispersion = if club.putter && self.on_green() {
            0.0025
        } else {
            club.dispersion + lie_dispersion
        };
        let launch_angle = wrap_angle_rad(self.angle + rng.gen_range(-dispersion..dispersion));
        let dir = Vec2::new(launch_angle.cos(), launch_angle.sin()).normalized();

        if club.putter {
            let rollout_yd = self.putter_rollout_target_yd(club);
            let rollout_tiles = (rollout_yd * lie_roll) / YARDS_PER_TILE;
            let rollout_speed = (rollout_tiles * 2.2).max(0.85);
            self.velocity = Vec2::new(
                dir.x * rollout_speed + self.wind * 0.035,
                dir.y * rollout_speed,
            );
            self.rolling = true;
            self.roll_time = 0.0;
            return;
        }

        let carry_tiles = (club.carry_yd * shot.carry_mult() * lie_carry) / YARDS_PER_TILE;
        let rollout_tiles = (club.rollout_yd * shot.roll_mult() * lie_roll) / YARDS_PER_TILE;
        let rollout_speed = rollout_tiles * 2.0;
        let wind_push_tiles = self.wind * (club.carry_yd / YARDS_PER_TILE) * 0.08;

        let landing = Vec2::new(
            self.ball.x + dir.x * carry_tiles + wind_push_tiles,
            self.ball.y + dir.y * carry_tiles,
        );

        self.airborne = Some(AirState {
            start: self.ball,
            landing,
            elapsed: 0.0,
            duration: club.air_time * shot.arc_mult(),
            apex: club.apex * shot.arc_mult(),
            rollout_speed,
        });
    }

    fn start_swing_animation(&mut self) {
        self.swing_active = true;
        self.swing_frame = 0;
        self.swing_timer = 0.0;
    }

    fn update_swing(&mut self, dt_secs: f32) {
        if self.swing_active {
            self.swing_timer += dt_secs;
            if self.swing_timer >= 0.07 {
                self.swing_timer = 0.0;
                if self.swing_frame + 1 < SWING_FRAMES {
                    self.swing_frame += 1;
                } else {
                    self.swing_active = false;
                    self.swing_frame = 0;
                }
            }
        } else if self.can_shoot() {
            self.swing_frame = 0;
            self.golfer_anchor = self.ball;
        }
    }

    fn putter_rollout_target_yd(&self, club: ClubSpec) -> f32 {
        let target = self.distance_to_hole_yd();
        if self.on_green() {
            (target * 1.35).clamp(4.0, club.rollout_yd)
        } else {
            club.rollout_yd
        }
    }

    fn lie_modifiers(&self, lie: Surface) -> (f32, f32, f32) {
        match lie {
            Surface::Green => (1.0, 1.0, 0.002),
            Surface::Fairway => (1.0, 1.0, 0.004),
            Surface::Rough => (0.82, 0.72, 0.028),
            Surface::Bunker => (0.65, 0.46, 0.045),
        }
    }

    fn auto_select_shot(&mut self) {
        let distance = self.distance_to_hole_yd();
        let lie = self.current_surface();
        let (lie_carry, lie_roll, _) = self.lie_modifiers(lie);

        if self.on_green() {
            self.selected_club_idx = CLUBS.len() - 1;
            self.selected_shot = ShotType::Full;
            return;
        }

        let mut best_idx = self.selected_club_idx;
        let mut best_shot = self.selected_shot;
        let mut best_error = f32::MAX;

        for (i, club) in CLUBS.iter().enumerate() {
            if club.putter && distance > 70.0 {
                continue;
            }

            let mut evaluate = |shot: ShotType| {
                let expected = if club.putter {
                    club.rollout_yd
                } else {
                    club.carry_yd * shot.carry_mult() * lie_carry
                        + club.rollout_yd * shot.roll_mult() * lie_roll
                };
                let mut error = (expected - distance).abs();
                if expected < distance {
                    error += (distance - expected) * 0.08;
                }
                if error < best_error {
                    best_error = error;
                    best_idx = i;
                    best_shot = if club.putter { ShotType::Full } else { shot };
                }
            };

            if club.putter {
                evaluate(ShotType::Full);
            } else {
                for shot in ShotType::NON_PUTTER {
                    evaluate(shot);
                }
            }
        }

        self.selected_club_idx = best_idx;
        self.selected_shot = best_shot;
    }
}

pub fn wrap_angle_rad(mut angle: f32) -> f32 {
    while angle <= -PI {
        angle += 2.0 * PI;
    }
    while angle > PI {
        angle -= 2.0 * PI;
    }
    angle
}

pub fn terrain_surface(x: i32, y: i32) -> Surface {
    let xf = x as f32;
    let yf = y as f32;

    let fairway_center = HEIGHT as f32 * 0.5 + (xf / 11.0).sin() * 2.5;
    let fairway_half_width = 2.8 + xf * 0.04;
    let distance = (yf - fairway_center).abs();

    let trap_a = ((xf - WIDTH as f32 * 0.38).powi(2) + (yf - HEIGHT as f32 * 0.32).powi(2)).sqrt();
    let trap_b = ((xf - WIDTH as f32 * 0.66).powi(2) + (yf - HEIGHT as f32 * 0.73).powi(2)).sqrt();
    let green_dist =
        ((xf - (WIDTH - 8) as f32).powi(2) + (yf - (HEIGHT / 2 - 5) as f32).powi(2)).sqrt();

    if green_dist < 2.6 {
        Surface::Green
    } else if trap_a < 2.8 || trap_b < 2.8 {
        Surface::Bunker
    } else if distance < fairway_half_width {
        Surface::Fairway
    } else {
        Surface::Rough
    }
}

pub fn terrain_char(x: i32, y: i32) -> char {
    match terrain_surface(x, y) {
        Surface::Green => {
            if (x + y) % 2 == 0 {
                '■'
            } else {
                '▪'
            }
        }
        Surface::Fairway => {
            if (x + y) % 2 == 0 {
                '■'
            } else {
                '▪'
            }
        }
        Surface::Rough => {
            if (x + y) % 3 == 0 {
                '▪'
            } else {
                '·'
            }
        }
        Surface::Bunker => {
            if (x + y) % 5 == 0 {
                '□'
            } else {
                '▫'
            }
        }
    }
}

pub fn terrain_color(x: i32, y: i32) -> Color {
    match terrain_surface(x, y) {
        Surface::Green => Color::Rgb {
            r: 90,
            g: 220,
            b: 90,
        },
        Surface::Fairway => Color::Rgb {
            r: 50,
            g: 170,
            b: 50,
        },
        Surface::Rough => Color::Rgb {
            r: 30,
            g: 110,
            b: 30,
        },
        Surface::Bunker => Color::Rgb {
            r: 192,
            g: 168,
            b: 112,
        },
    }
}
