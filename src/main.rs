use std::{error::Error, io, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use ratatui::{backend::CrosstermBackend, Terminal};

mod app;
mod models;
mod physics;
mod ui;

use app::{ActivePanel, App, MaterialsFocus};

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (cols, rows) = size()?;
    let sidebar_width = 38usize;
    let world_width = cols.saturating_sub(sidebar_width as u16 + 4).max(20) as usize;
    let world_height = rows.saturating_sub(2).max(12) as usize;

    let mut app = App::new(world_width, world_height);
    let mut tick = 0u64;

    loop {
        terminal.draw(|frame| {
            ui::simulation::draw(frame, &app);
        })?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('s') => app.active_panel = ActivePanel::Simulation,
                    KeyCode::Char('m') => app.active_panel = ActivePanel::Materials,
                    KeyCode::Esc => {
                        if app.active_panel == ActivePanel::Materials
                            && app.materials_focus == MaterialsFocus::Items
                        {
                            app.back_to_categories();
                        }
                    }
                    KeyCode::Char('c') if app.active_panel == ActivePanel::Simulation => {
                        app.clear_world();
                    }
                    KeyCode::Enter | KeyCode::Char(' ') => match app.active_panel {
                        ActivePanel::Simulation => app.spawn_selected(),
                        ActivePanel::Materials => {
                            if app.materials_focus == MaterialsFocus::Categories {
                                app.enter_category();
                            } else {
                                app.select_current_material();
                            }
                        }
                    },
                    KeyCode::Up => match app.active_panel {
                        ActivePanel::Simulation => app.move_cursor(0, -1),
                        ActivePanel::Materials => {
                            if app.materials_focus == MaterialsFocus::Categories {
                                app.select_prev_category();
                            } else {
                                app.select_prev_material();
                            }
                        }
                    },
                    KeyCode::Down => match app.active_panel {
                        ActivePanel::Simulation => app.move_cursor(0, 1),
                        ActivePanel::Materials => {
                            if app.materials_focus == MaterialsFocus::Categories {
                                app.select_next_category();
                            } else {
                                app.select_next_material();
                            }
                        }
                    },
                    KeyCode::Left => match app.active_panel {
                        ActivePanel::Simulation => app.move_cursor(-1, 0),
                        ActivePanel::Materials => app.back_to_categories(),
                    },
                    KeyCode::Right => match app.active_panel {
                        ActivePanel::Simulation => app.move_cursor(1, 0),
                        ActivePanel::Materials => {
                            if app.materials_focus == MaterialsFocus::Categories {
                                app.enter_category();
                            } else {
                                app.select_current_material();
                            }
                        }
                    },
                    _ => {}
                }
            }
        }

        physics::rules::step(&mut app.world, &app.materials, tick);
        tick = tick.wrapping_add(1);
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}