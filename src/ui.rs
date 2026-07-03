

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{block::{Block, Position, Title}, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};
use crate::app::{App, InputMode};

pub fn ui(f: &mut Frame, app: &mut App) {
    
    if app.input_mode == InputMode::Executing || app.input_mode == InputMode::Done {
        let block_title = if app.input_mode == InputMode::Executing {
            format!(" {} packages uninstalling... ", app.selected_for_removal.len())
        } else {
            format!(" {} packages uninstalled! (Press ENTER to return) ", app.selected_for_removal.len())
        };

        let log_block = Block::default()
            .title(block_title)
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White)); 

        let logs_text = app.logs.join("\n");
        let line_count = app.logs.len() as u16;
        let area_height = f.size().height.saturating_sub(2);
        let scroll = if line_count > area_height { line_count - area_height } else { 0 };

        let log_paragraph = Paragraph::new(logs_text)
            .block(log_block)
            .wrap(Wrap { trim: false })
            .scroll((scroll, 0));

        
        f.render_widget(log_paragraph, f.size());
        return;
    }

    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(f.size());

    
    let header_text = format!(
        " Mode: {} | Pacman [P] | Flatpak [F] | Search [S] | Select [Space] | Uninstall [Enter] | Quit [Q] ",
        app.manager.name()
    );
    let header = Paragraph::new(header_text)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title(" UNINSTALLER "));
    f.render_widget(header, chunks[0]);

    let search_query = app.search_query.to_lowercase();
    let filtered_packages: Vec<_> = app.packages.iter()
        .filter(|p| p.name.to_lowercase().contains(&search_query))
        .collect();

    if let Some(selected) = app.state.selected() {
        if selected >= filtered_packages.len() && !filtered_packages.is_empty() {
            app.state.select(Some(filtered_packages.len() - 1));
        } else if filtered_packages.is_empty() {
            app.state.select(None);
        }
    }

    let items: Vec<ListItem> = filtered_packages
        .iter()
        .map(|pkg| {
            let is_selected = app.selected_for_removal.contains(&pkg.id);
            let prefix = if is_selected { "[X]" } else { "[ ]" };
            let content = format!("{} {} (Version: {})", prefix, pkg.name, pkg.version);
            
            
            let style = if is_selected {
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Packages "))
        
        .highlight_style(Style::default().bg(Color::DarkGray).fg(Color::White).add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, chunks[1], &mut app.state);

    let (footer_title, footer_text) = match app.input_mode {
        InputMode::Normal => {
            let text = if !app.search_query.is_empty() { format!(" Filter active: '{}' (Press 'C' to clear) ", app.search_query) } 
            else { format!(" {} packages found. ", filtered_packages.len()) };
            (" Status ", text)
        }
        InputMode::Search => {
            (" Search (Press Enter/Esc to stop) ", format!(" {}_", app.search_query))
        }
        _ => (" Status ", " ... ".to_string())
    };

    let footer_block = Block::default()
        .borders(Borders::ALL)
        .title(footer_title)
        .title(
            Title::from(" Made by Neuwj - Neuwj@linuxmail.org ")
                .alignment(Alignment::Right)
                .position(Position::Bottom)
        )
        .style(Style::default().fg(Color::White));

    let footer = Paragraph::new(footer_text).style(Style::default().fg(Color::White)).block(footer_block);
    f.render_widget(footer, chunks[2]);
}
