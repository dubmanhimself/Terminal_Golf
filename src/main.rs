use std::io::{stdout, Stdout};
use std::thread;
use std::time::{Duration, Instant};

use crossterm::cursor::{Hide, Show};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};

mod game;
mod render;

use game::{wrap_angle_rad, Game, TICK_MS};

fn main() -> std::io::Result<()> {
    let mut stdout = stdout();
    setup_terminal(&mut stdout)?;

    let mut game = Game::new();
    let result = run_game_loop(&mut stdout, &mut game);

    restore_terminal(&mut stdout)?;
    result
}

fn setup_terminal(stdout: &mut Stdout) -> std::io::Result<()> {
    execute!(stdout, EnterAlternateScreen, Hide)?;
    terminal::enable_raw_mode()?;
    Ok(())
}

fn restore_terminal(stdout: &mut Stdout) -> std::io::Result<()> {
    terminal::disable_raw_mode()?;
    execute!(stdout, Show, LeaveAlternateScreen)?;
    Ok(())
}

fn run_game_loop(stdout: &mut Stdout, game: &mut Game) -> std::io::Result<()> {
    let mut last_tick = Instant::now();

    loop {
        while event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc | KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('r') => game.reset(),
                        KeyCode::Left | KeyCode::Char('a') => {
                            if game.can_shoot() {
                                game.angle = wrap_angle_rad(game.angle - game.aim_step());
                            }
                        }
                        KeyCode::Right | KeyCode::Char('d') => {
                            if game.can_shoot() {
                                game.angle = wrap_angle_rad(game.angle + game.aim_step());
                            }
                        }
                        KeyCode::Char('w') | KeyCode::Up => game.cycle_club(1),
                        KeyCode::Char('s') | KeyCode::Down => game.cycle_club(-1),
                        KeyCode::Char('e') => game.cycle_shot_type(),
                        KeyCode::Char('c') => game.toggle_auto_caddie(),
                        KeyCode::Enter | KeyCode::Char(' ') => game.hit_ball(),
                        _ => {}
                    }
                }
            }
        }

        let now = Instant::now();
        let dt = now.duration_since(last_tick);
        if dt.as_millis() >= TICK_MS as u128 {
            game.update(dt.as_secs_f32());
            render::draw(stdout, game)?;
            last_tick = now;
        } else {
            thread::sleep(Duration::from_millis(1));
        }
    }
}
