

pub mod managers;
pub mod app;
pub mod ui;

use app::App;
use managers::{pacman::Pacman, flatpak::Flatpak};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{error::Error, io::{self, BufRead, BufReader}, process::Stdio, sync::mpsc, thread, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(Box::new(Pacman));

    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::ui(f, app))?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                
                
                if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    return Ok(());
                }

                match app.input_mode {
                    app::InputMode::Normal => match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(()),
                        KeyCode::Down => app.next(),
                        KeyCode::Up => app.previous(),
                        KeyCode::Char('s') | KeyCode::Char('S') => app.input_mode = app::InputMode::Search,
                        KeyCode::Char('c') | KeyCode::Char('C') => {
                            
                            if !key.modifiers.contains(KeyModifiers::CONTROL) {
                                app.search_query.clear();
                            }
                        },
                        KeyCode::Char('p') | KeyCode::Char('P') => {
                            app.manager = Box::new(Pacman);
                            app.refresh_packages();
                            app.selected_for_removal.clear();
                        }
                        KeyCode::Char('f') | KeyCode::Char('F') => {
                            app.manager = Box::new(Flatpak);
                            app.refresh_packages();
                            app.selected_for_removal.clear();
                        }
                        KeyCode::Char(' ') => {
                            let filtered: Vec<_> = app.packages.iter()
                                .filter(|p| p.name.to_lowercase().contains(&app.search_query.to_lowercase()))
                                .collect();
                            
                            if let Some(i) = app.state.selected() {
                                if let Some(pkg) = filtered.get(i) {
                                    let id = pkg.id.clone();
                                    if app.selected_for_removal.contains(&id) {
                                        app.selected_for_removal.retain(|x| x != &id);
                                    } else {
                                        app.selected_for_removal.push(id);
                                    }
                                }
                            }
                        }
                        KeyCode::Enter => {
                            if !app.selected_for_removal.is_empty() {
                                app.input_mode = app::InputMode::Executing;
                                app.logs.clear();
                                
                                
                                app.logs.push("[sudo] password for uninstaller: ".to_string());
                                app.logs.push("".to_string());
                                
                                let mut cmd = app.manager.build_remove_command(&app.selected_for_removal);
                                
                                cmd.stdout(Stdio::piped());
                                cmd.stderr(Stdio::piped());
                                
                                if let Ok(mut child) = cmd.spawn() {
                                    let stdout = child.stdout.take().unwrap();
                                    let stderr = child.stderr.take().unwrap();
                                    
                                    let (tx, rx) = mpsc::channel();
                                    let tx_err = tx.clone();
                                    
                                    thread::spawn(move || {
                                        let reader = BufReader::new(stdout);
                                        for line in reader.lines() {
                                            if let Ok(l) = line { let _ = tx.send(l); }
                                        }
                                    });
                                    
                                    thread::spawn(move || {
                                        let reader = BufReader::new(stderr);
                                        for line in reader.lines() {
                                            if let Ok(l) = line { let _ = tx_err.send(l); }
                                        }
                                    });
                                    
                                    loop {
                                        terminal.draw(|f| ui::ui(f, app))?;
                                        
                                        while let Ok(line) = rx.try_recv() {
                                            app.logs.push(line);
                                        }
                                        
                                        if let Ok(Some(status)) = child.try_wait() {
                                            app.logs.push(format!("\n[Process Completed with: {}]", status));
                                            app.input_mode = app::InputMode::Done;
                                            break;
                                        }
                                        thread::sleep(Duration::from_millis(30));
                                    }
                                } else {
                                    app.logs.push("ERROR: Failed to start process!".to_string());
                                    app.input_mode = app::InputMode::Done;
                                }
                            }
                        }
                        _ => {}
                    },
                    app::InputMode::Search => match key.code {
                        KeyCode::Enter | KeyCode::Esc => app.input_mode = app::InputMode::Normal,
                        KeyCode::Char(c) => app.search_query.push(c),
                        KeyCode::Backspace => { app.search_query.pop(); },
                        _ => {}
                    },
                    app::InputMode::Done => match key.code {
                        KeyCode::Enter => {
                            app.input_mode = app::InputMode::Normal;
                            app.logs.clear();
                            app.refresh_packages();
                            app.selected_for_removal.clear();
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }
}
