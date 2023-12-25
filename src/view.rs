use std::time::Duration;

use anyhow::{Ok, Result};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    prelude::{Style, Terminal},
    style::{Color, Modifier},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::model::{App, Goal, Message, Screen, State, Task};

pub fn ran_app(mut app: App, mut terminal: Terminal<impl Backend>) -> Result<(), anyhow::Error> {
    {
        app.add_goal(Goal {
            name: String::from("Build alfred"),
        });
        app.add_goal(Goal {
            name: String::from("Work on nvim config"),
        });
        app.add_goal(Goal {
            name: String::from("Review my personal website"),
        });
        app.add_goal(Goal {
            name: String::from("Build alfred"),
        });
        app.add_task(Task {
            name: String::from("Meeting with client"),
        });
        app.add_task(Task {
            name: String::from("Dentist appointment"),
        });
    }

    Ok(while app.state != State::Quit {
        // Render the current view
        terminal.draw(|f| view(&mut app, f))?;

        // Handle events and map to a Message
        let mut current_msg = handle_event(&mut app)?;

        // Process updates as long as they return a non-None message
        while current_msg.is_some() {
            current_msg = update(&mut app, current_msg.unwrap());
        }
    })
}

fn handle_event(app: &mut App) -> Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(match app.screen {
                    Screen::Dashboard => handle_dash(key),
                    Screen::Edit => handle_edit(key),
                    Screen::Init => todo!(),
                    Screen::Quit => todo!(),
                });
                // return Ok(handle_key(key));
            };
        }
    }
    Ok(None)
}

fn handle_dash(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => Some(Message::Up),
        KeyCode::Down | KeyCode::Char('j') => Some(Message::Down),
        KeyCode::Esc | KeyCode::Char('q') => Some(Message::Quit),
        _ => None,
    }
}

fn handle_edit(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => Some(Message::Done),
        _ => None,
    }
}

fn update(model: &mut App, msg: Message) -> Option<Message> {
    match msg {
        Message::Quit => {
            // You can handle cleanup and exit here
            model.state = State::Quit;
        }
        Message::Edit => todo!(),
        Message::Add => todo!(),
        Message::Init => todo!(),
        Message::Done => todo!(),
        Message::Cancel => todo!(),
        _ => todo!(),
    };
    None
}

fn view(app: &mut App, f: &mut Frame) {
    let today = activities(app);

    let main_layout = Layout::new(
        Direction::Vertical,
        [
            // Today
            Constraint::Length(1),
            Constraint::Min((today.len() + 1).try_into().unwrap()),
            // Tomorrow
            Constraint::Length(1),
            Constraint::Min(0),
            // This week
            Constraint::Length(1),
            Constraint::Min(0),
            // This month
            Constraint::Length(1),
            Constraint::Min(0),
            // Status bar
            Constraint::Length(1),
            // Commands
            Constraint::Length(1),
        ],
    )
    .split(f.size());

    // TODAY
    f.render_widget(
        Block::new().borders(Borders::TOP).title("Today"),
        main_layout[0],
    );

    f.render_widget(Paragraph::new(today), main_layout[1]);

    // TOMORROW
    f.render_widget(
        Block::new().borders(Borders::TOP).title("Tomorrow"),
        main_layout[2],
    );
    f.render_widget(Paragraph::new("-"), main_layout[3]);

    // THIS WEEK
    f.render_widget(
        Block::new().borders(Borders::TOP).title("This week"),
        main_layout[4],
    );
    f.render_widget(Paragraph::new("-"), main_layout[5]);

    // THIS MONTH
    f.render_widget(
        Block::new().borders(Borders::TOP).title("This month"),
        main_layout[6],
    );
    f.render_widget(Paragraph::new("-"), main_layout[7]);

    // STATUS BAR
    f.render_widget(
        Block::new().borders(Borders::TOP).title("Status Bar"),
        main_layout[8],
    );

    // COMMANDS
    f.render_widget(
        Block::new()
            .borders(Borders::TOP)
            .title(format!("Commands | Screen {}", app.screen)),
        main_layout[9],
    );
}

fn activities(m: &mut App) -> Vec<Line<'_>> {
    let style = Style::default().add_modifier(Modifier::BOLD);
    let on_time = style.fg(Color::Green);
    let task = style.fg(Color::Cyan);
    // let overdue = style.fg(Color::Magenta);
    let mut today: Vec<Line> = m
        .goals
        .iter()
        .map(|g| Line::styled(g.name.clone(), on_time))
        .collect();
    today.append(
        &mut m
            .tasks
            .iter()
            .map(|t| Line::styled(t.name.clone(), task))
            .collect(),
    );
    today
}
