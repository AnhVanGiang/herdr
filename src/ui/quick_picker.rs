use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Clear, Paragraph},
    Frame,
};

use super::widgets::{panel_contrast_fg, render_panel_shell};
use crate::{
    app::state::{AppState, QuickPickerEntry},
    terminal::TerminalRuntimeRegistry,
};

pub(super) fn render_quick_picker_overlay(
    app: &AppState,
    terminal_runtimes: &TerminalRuntimeRegistry,
    frame: &mut Frame,
) {
    let popup = app.quick_picker_popup_rect();
    let Some(_inner) = render_panel_shell(frame, popup, app.palette.accent, app.palette.panel_bg)
    else {
        return;
    };

    render_header(app, frame, app.quick_picker_header_rect());
    render_search(app, frame, app.quick_picker_search_rect());
    render_rows(app, terminal_runtimes, frame, app.quick_picker_body_rect());
    render_footer(app, frame, app.quick_picker_footer_rect());
}

fn render_header(app: &AppState, frame: &mut Frame, area: Rect) {
    let line = Line::from(Span::styled(
        "Jump to space or agent",
        Style::default()
            .fg(app.palette.text)
            .add_modifier(Modifier::BOLD),
    ));
    frame.render_widget(Paragraph::new(line), area);
}

fn render_search(app: &AppState, frame: &mut Frame, area: Rect) {
    let query = if app.quick_picker.query.is_empty() {
        Span::styled("type to filter", Style::default().fg(app.palette.overlay0))
    } else {
        Span::styled(
            app.quick_picker.query.clone(),
            Style::default().fg(app.palette.text),
        )
    };
    let line = Line::from(vec![
        Span::styled(
            "> ",
            Style::default()
                .fg(app.palette.accent)
                .add_modifier(Modifier::BOLD),
        ),
        query,
    ]);
    frame.render_widget(Paragraph::new(line), area);
}

fn render_rows(
    app: &AppState,
    terminal_runtimes: &TerminalRuntimeRegistry,
    frame: &mut Frame,
    body: Rect,
) {
    let entries = app.filtered_quick_picker_entries_from(terminal_runtimes);
    if body.height == 0 {
        return;
    }

    if entries.is_empty() {
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(
                "No matches",
                Style::default().fg(app.palette.overlay0),
            ))),
            body,
        );
        return;
    }

    let start = app.quick_picker.scroll.min(entries.len());
    let end = entries
        .len()
        .min(start.saturating_add(body.height as usize));
    for (visible_idx, entry) in entries[start..end].iter().enumerate() {
        let idx = start + visible_idx;
        let rect = Rect::new(body.x, body.y + visible_idx as u16, body.width, 1);
        render_row(app, frame, rect, entry, idx == app.quick_picker.selected);
    }
}

fn render_row(
    app: &AppState,
    frame: &mut Frame,
    rect: Rect,
    entry: &QuickPickerEntry,
    selected: bool,
) {
    frame.render_widget(Clear, rect);
    let base_style = if selected {
        Style::default()
            .bg(app.palette.accent)
            .fg(panel_contrast_fg(&app.palette))
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
            .bg(app.palette.panel_bg)
            .fg(app.palette.text)
    };
    let dim_style = if selected {
        base_style
    } else {
        Style::default()
            .bg(app.palette.panel_bg)
            .fg(app.palette.overlay0)
    };
    let badge = if entry.is_workspace { "Space" } else { "Agent" };
    let current = if entry.is_current { "◆" } else { " " };
    let line = Line::from(vec![
        Span::styled(format!("{} ", current), dim_style),
        Span::styled(format!("[{badge}] "), base_style),
        Span::styled(entry.label.clone(), base_style),
        Span::styled(" — ", dim_style),
        Span::styled(entry.meta.clone(), dim_style),
    ]);
    frame.render_widget(Paragraph::new(line).style(base_style), rect);
}

fn render_footer(app: &AppState, frame: &mut Frame, area: Rect) {
    let key = Style::default()
        .fg(app.palette.accent)
        .add_modifier(Modifier::BOLD);
    let dim = Style::default().fg(app.palette.overlay0);
    let line = Line::from(vec![
        Span::styled("ctrl+n/ctrl+p", key),
        Span::styled(" move  ", dim),
        Span::styled("↑↓", key),
        Span::styled(" move  ", dim),
        Span::styled("enter", key),
        Span::styled(" jump  ", dim),
        Span::styled("esc", key),
        Span::styled(" close", dim),
    ]);
    frame.render_widget(Paragraph::new(line), area);
}
