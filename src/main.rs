use anyhow::{Context, Result};
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    /// File to resolve conflicts in
    file: PathBuf,
}

#[derive(Debug, Clone)]
enum Hunk {
    Normal(String),
    Conflict {
        ours: Vec<String>,
        base: Vec<String>,
        theirs: Vec<String>,
        resolved: Option<ResolvedSide>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ResolvedSide {
    Ours,
    Theirs,
    Base,
}

struct App {
    hunks: Vec<Hunk>,
    current_hunk_idx: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let content = fs::read_to_string(&args.file).context("Failed to read file")?;
    let hunks = parse_conflicts(&content);

    let mut app = App {
        hunks,
        current_hunk_idx: 0,
    };

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(true) = res {
        let mut final_content = String::new();
        for hunk in app.hunks {
            match hunk {
                Hunk::Normal(s) => final_content.push_str(&s),
                Hunk::Conflict { ours, base, theirs, resolved } => {
                    match resolved {
                        Some(ResolvedSide::Ours) => {
                            for line in ours {
                                final_content.push_str(&line);
                                final_content.push('\n');
                            }
                        }
                        Some(ResolvedSide::Theirs) => {
                            for line in theirs {
                                final_content.push_str(&line);
                                final_content.push('\n');
                            }
                        }
                        Some(ResolvedSide::Base) => {
                            for line in base {
                                final_content.push_str(&line);
                                final_content.push('\n');
                            }
                        }
                        None => {
                            final_content.push_str("<<<<<<< HEAD\n");
                            for line in ours { final_content.push_str(&line); final_content.push('\n'); }
                            final_content.push_str("=======\n");
                            for line in theirs { final_content.push_str(&line); final_content.push('\n'); }
                            final_content.push_str(">>>>>>> incoming\n");
                        }
                    }
                }
            }
        }
        fs::write(&args.file, final_content)?;
        println!("File saved.");
    }

    Ok(())
}

fn parse_conflicts(content: &str) -> Vec<Hunk> {
    let mut hunks = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;
    let mut current_normal = String::new();

    while i < lines.len() {
        if lines[i].starts_with("<<<<<<<") {
            if !current_normal.is_empty() {
                hunks.push(Hunk::Normal(current_normal));
                current_normal = String::new();
            }
            
            let mut ours = Vec::new();
            let mut base = Vec::new();
            let mut theirs = Vec::new();
            
            i += 1;
            while i < lines.len() && !lines[i].starts_with("=======") && !lines[i].starts_with("|||||||") {
                ours.push(lines[i].to_string());
                i += 1;
            }
            
            if i < lines.len() && lines[i].starts_with("|||||||") {
                i += 1;
                while i < lines.len() && !lines[i].starts_with("=======") {
                    base.push(lines[i].to_string());
                    i += 1;
                }
            }
            
            if i < lines.len() && lines[i].starts_with("=======") {
                i += 1;
                while i < lines.len() && !lines[i].starts_with(">>>>>>>") {
                    theirs.push(lines[i].to_string());
                    i += 1;
                }
            }
            
            if i < lines.len() && lines[i].starts_with(">>>>>>>") {
                i += 1;
            }
            
            hunks.push(Hunk::Conflict { ours, base, theirs, resolved: None });
        } else {
            current_normal.push_str(lines[i]);
            current_normal.push('\n');
            i += 1;
        }
    }
    
    if !current_normal.is_empty() {
        hunks.push(Hunk::Normal(current_normal));
    }
    
    hunks
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(false),
                KeyCode::Char('s') => return Ok(true),
                KeyCode::Up => {
                    if app.current_hunk_idx > 0 {
                        app.current_hunk_idx -= 1;
                    }
                }
                KeyCode::Down => {
                    if app.current_hunk_idx < app.hunks.len() - 1 {
                        app.current_hunk_idx += 1;
                    }
                }
                KeyCode::Char('1') => {
                    if let Hunk::Conflict { ref mut resolved, .. } = app.hunks[app.current_hunk_idx] {
                        *resolved = Some(ResolvedSide::Ours);
                    }
                }
                KeyCode::Char('2') => {
                    if let Hunk::Conflict { ref mut resolved, .. } = app.hunks[app.current_hunk_idx] {
                        *resolved = Some(ResolvedSide::Theirs);
                    }
                }
                KeyCode::Char('3') => {
                    if let Hunk::Conflict { ref mut resolved, .. } = app.hunks[app.current_hunk_idx] {
                        *resolved = Some(ResolvedSide::Base);
                    }
                }
                _ => {}
            }
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(3)].as_ref())
        .split(f.size());

    let mut items = Vec::new();
    for (i, hunk) in app.hunks.iter().enumerate() {
        let style = if i == app.current_hunk_idx {
            Style::default().bg(Color::DarkGray)
        } else {
            Style::default()
        };

        match hunk {
            Hunk::Normal(s) => {
                items.push(ListItem::new(s.clone()).style(style));
            }
            Hunk::Conflict { ours, theirs, resolved, .. } => {
                let status = match resolved {
                    Some(ResolvedSide::Ours) => "[RESOLVED: OURS]",
                    Some(ResolvedSide::Theirs) => "[RESOLVED: THEIRS]",
                    Some(ResolvedSide::Base) => "[RESOLVED: BASE]",
                    None => "[CONFLICT]",
                };
                let content = format!("{} (1: ours, 2: theirs, 3: base)\nOurs: {:?}\nTheirs: {:?}", status, ours, theirs);
                items.push(ListItem::new(content).style(style.fg(Color::Yellow)));
            }
        }
    }

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Merge Tool"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    
    f.render_widget(list, chunks[0]);

    let help = Paragraph::new("Use arrows to navigate, 1/2/3 to pick side, 's' to save, 'q' to quit")
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[1]);
}
