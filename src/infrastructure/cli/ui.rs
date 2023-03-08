use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs},
    Frame,
};

pub fn draw<B>(rect: &mut Frame<B>)
where
    B: Backend,
{
    let size = rect.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(2),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(size);

    let title = draw_title();
    let tabs = draw_tabs();
    let copyright = draw_copy_right();

    rect.render_widget(title, chunks[0]);
    rect.render_widget(tabs, chunks[1]);
    rect.render_widget(copyright, chunks[2]);
}

fn draw_title<'a>() -> Paragraph<'a> {
    Paragraph::new("Rust First Api")
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White).bg(Color::Black))
                .border_type(BorderType::Plain)
                .title("Rust First Api"),
        )
}

fn draw_copy_right<'a>() -> Paragraph<'a> {
    Paragraph::new("Rust first CLI 2023 - all rights reserved")
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White).bg(Color::Black))
                .border_type(BorderType::Plain)
                .title("Copyright"),
        )
}

fn draw_tabs<'a>() -> Tabs<'a> {
    let menu_titles = vec!["Home", "Users", "Add", "Delete", "Quit"];

    let menu = menu_titles
        .iter()
        .map(|title| {
            let (first, rest) = title.split_at(1);
            Spans::from(vec![
                Span::styled(
                    first,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        })
        .collect();

    Tabs::new(menu)
        .block(Block::default().borders(Borders::ALL).title("Menu"))
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(Span::raw("|"))
}
