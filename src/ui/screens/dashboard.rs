use crate::infrastructure::lm_studio::ModelInfo;
use crate::ui::components::header;
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, model_name: &str, models: &[ModelInfo], selected_index: usize) {
    let size = f.area();

    f.render_widget(
        Block::default().style(Style::default().bg(Theme::BACKGROUND)),
        size,
    );

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(16),
            Constraint::Length(13),
            Constraint::Min(0),
        ])
        .split(size);

    header::render_header(f, chunks[0], model_name);

    let block = Block::default()
        .title(" SELECIONE O MODELO ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Theme::TEXT_SECONDARY))
        .style(Style::default().bg(Theme::BACKGROUND));

    let inner_area = block.inner(chunks[1]);
    let inner_width = inner_area.width as usize;

    let mut lines = vec![];
    for (i, m) in models.iter().enumerate() {
        let is_cursor = i == selected_index;
        let is_active = m.id == model_name;

        // A bolinha verde só vai no modelo ativo (selecionado com espaço)
        let prefix = if is_active { "(●) " } else { "( ) " };

        // Trunca e preenche com espaços para limpar o fundo
        let mut text = format!("{}{}", prefix, m.id);
        if text.len() > inner_width {
            text.truncate(inner_width);
        } else {
            text = format!("{:<width$}", text, width = inner_width);
        }

        // Azul para o cursor (navegação), Verde/Bold para o modelo ativo
        let style = if is_cursor {
            Style::default()
                .fg(Theme::TEXT_BLUE)
                .bg(Theme::BACKGROUND)
                .add_modifier(Modifier::BOLD)
        } else if is_active {
            Style::default()
                .fg(Theme::ACCENT)
                .bg(Theme::BACKGROUND)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Theme::TEXT_SECONDARY)
                .bg(Theme::BACKGROUND)
        };

        lines.push(Line::from(Span::styled(text, style)));
    }

    f.render_widget(Paragraph::new(lines).block(block), chunks[1]);

    let status = vec![
        Line::from(Span::styled(
            "● LM Studio Server: ONLINE",
            Style::default().fg(Theme::ACCENT),
        )),
        Line::from(Span::styled(
            "● Claude Code CLI: READY",
            Style::default().fg(Theme::ACCENT),
        )),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Use [↑] [↓] para navegar",
            Style::default().fg(Theme::TEXT_SECONDARY),
        )]),
        Line::from(vec![
            Span::styled("Pressione ", Style::default().fg(Theme::TEXT_SECONDARY)),
            Span::styled("[ESPAÇO]", Style::default().fg(Theme::ACCENT)),
            Span::styled(
                " para selecionar o modelo",
                Style::default().fg(Theme::TEXT_SECONDARY),
            ),
        ]),
        Line::from(vec![
            Span::styled("Pressione ", Style::default().fg(Theme::TEXT_SECONDARY)),
            Span::styled("[ENTER]", Style::default().fg(Theme::ACCENT)),
            Span::styled(
                " para iniciar a sessão",
                Style::default().fg(Theme::TEXT_SECONDARY),
            ),
        ]),
        Line::from(vec![
            Span::styled("Pressione ", Style::default().fg(Theme::TEXT_SECONDARY)),
            Span::styled("[Esc]", Style::default().fg(Theme::TEXT_SECONDARY)),
            Span::styled(" para sair", Style::default().fg(Theme::TEXT_SECONDARY)),
        ]),
    ];

    f.render_widget(
        Paragraph::new(status).alignment(Alignment::Center),
        chunks[2],
    );
}
