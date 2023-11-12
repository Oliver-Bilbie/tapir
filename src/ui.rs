use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, CurrentScreen, InputMode, KeyValuePair};

pub fn ui<B: Backend>(frame: &mut Frame, app: &App) {
    // Create the layout sections.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.size());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Request Headers",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    frame.render_widget(title, chunks[0]);
    let mut list_items = Vec::<ListItem>::new();

    for (index, (key, value)) in app.request_headers.iter().enumerate() {
        list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{: <25} : {}", key, value),
            if let Some(selected_index) = app.selected_index {
                if index as u8 == selected_index {
                    Style::default().fg(Color::Magenta)
                } else {
                    Style::default()
                }
            } else {
                Style::default()
            },
        ))));
    }

    let list = List::new(list_items);

    frame.render_widget(list, chunks[1]);
    let current_navigation_text = vec![
        // The first half of the text
        match app.current_screen {
            CurrentScreen::Main => Span::styled("Normal Mode", Style::default().fg(Color::Green)),
            CurrentScreen::Input(ref input_state) => match &input_state.mode {
                InputMode::Add => {
                    Span::styled("Adding a new item", Style::default().fg(Color::Red))
                }
                InputMode::Edit(edit_key) => Span::styled(
                    format!("Editing {}", edit_key),
                    Style::default().fg(Color::Red),
                ),
            },
            CurrentScreen::Submit => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
        }
        .to_owned(),
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "[a]dd / [e]dit / [d]elete / [q]uit / Enter to submit",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Input(_) => Span::styled(
                "(ESC) to cancel / (Tab) to switch boxes / Enter to submit",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Submit => Span::styled(
                "[a]dd / [e]dit / [d]elete / [q]uit / Enter to submit",
                Style::default().fg(Color::Red),
            ),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);

    if let CurrentScreen::Input(input_state) = &app.current_screen {
        let popup_block = Block::default()
            .title("Enter a new key-value pair")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));
        let area = centered_rect(60, 25, frame.size());
        frame.render_widget(popup_block, area);
        let popup_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);
        let mut key_block = Block::default().title("Key").borders(Borders::ALL);
        let mut value_block = Block::default().title("Value").borders(Borders::ALL);
        let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);
        match input_state.selected_item {
            KeyValuePair::Key => key_block = key_block.style(active_style),
            KeyValuePair::Value => value_block = value_block.style(active_style),
        };
        let key_text = Paragraph::new(input_state.key.clone()).block(key_block);
        frame.render_widget(key_text, popup_chunks[0]);
        let value_text = Paragraph::new(input_state.value.clone()).block(value_block);
        frame.render_widget(value_text, popup_chunks[1]);
    }

    if let CurrentScreen::Submit = app.current_screen {
        frame.render_widget(Clear, frame.size()); //this clears the entire screen and anything already drawn
        let popup_block = Block::default()
            .title("Y/N")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let exit_text = Text::styled(
            "Would you like to output the buffer as json? (y/n)",
            Style::default().fg(Color::Red),
        );
        // the `trim: false` will stop the text from being cut off when over the edge of the block
        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rect(60, 25, frame.size());
        frame.render_widget(exit_paragraph, area);
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
