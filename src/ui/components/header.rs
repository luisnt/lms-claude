use crate::ui::theme::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn render_header(f: &mut Frame, area: Rect, model_name: &str) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Theme::TEXT_SECONDARY))
        .style(Style::default().bg(Theme::BACKGROUND));

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Min(0),
        ])
        .split(inner_area);

    let profile = vec![
        Line::from(Span::styled(
            "LUIS CALDAS",
            Style::default()
                .fg(Theme::TEXT_PRIMARY)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "Senior Software Engineer",
            Style::default().fg(Theme::TEXT_BLUE),
        )),
        Line::from(Span::styled(
            "Software Architecture | Systems Integration | Automation",
            Style::default().fg(Theme::TEXT_SECONDARY),
        )),
        Line::from(Span::styled(
            "github.com/luisnt",
            Style::default()
                .fg(Theme::TEXT_BLUE)
                .add_modifier(Modifier::UNDERLINED),
        )),
    ];
    f.render_widget(Paragraph::new(profile), chunks[0]);

    let separator = Paragraph::new("─".repeat(inner_area.width as usize))
        .style(Style::default().fg(Theme::TEXT_SECONDARY));
    f.render_widget(separator.clone(), chunks[1]);

    let title = vec![
        Line::from(Span::styled(
            "CLAUDE CODE / LM STUDIO LOCAL",
            Style::default()
                .fg(Theme::TEXT_PRIMARY)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "Local inference · localhost:1234",
            Style::default().fg(Theme::TEXT_SECONDARY),
        )),
    ];
    f.render_widget(Paragraph::new(title), chunks[2]);

    f.render_widget(separator, chunks[3]);

    // Prevenção de wrap: calcula o tamanho exato e trunca se necessário
    let inner_width = chunks[4].width as usize;
    let max_model_len = inner_width.saturating_sub(7); // "Model: ".len() == 7
    let model_display = if model_name.len() > max_model_len {
        &model_name[..max_model_len]
    } else {
        model_name
    };
    let padded_model = format!("{:<width$}", model_display, width = max_model_len);

    let config = vec![
        Line::from(Span::styled(
            "SESSION CONFIGURATION",
            Style::default().fg(Theme::TEXT_PRIMARY),
        )),
        Line::from(vec![
            Span::styled("Model: ", Style::default().fg(Theme::TEXT_SECONDARY)),
            Span::styled(
                padded_model,
                Style::default()
                    .fg(Theme::TEXT_BLUE)
                    .bg(Theme::BACKGROUND)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ];
    f.render_widget(
        Paragraph::new(config).style(Style::default().bg(Theme::BACKGROUND)),
        chunks[4],
    );
}
