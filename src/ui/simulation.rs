use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use crate::{
    app::{ActivePanel, App, MaterialsFocus},
    models::material::Category,
};

pub fn draw(frame: &mut Frame<'_>, app: &App) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(10), Constraint::Length(38)])
        .split(area);

    render_world(frame, chunks[0], app);
    render_sidebar(frame, chunks[1], app);
}

fn render_world(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let title = if app.active_panel == ActivePanel::Simulation {
        "simulation"
    } else {
        "simulation"
    };

    let block = Block::default().borders(Borders::ALL).title(title);
    frame.render_widget(block, area);

    let inner = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };

    if inner.width == 0 || inner.height == 0 {
        return;
    }

    let w = inner.width.min(app.world.width as u16) as usize;
    let h = inner.height.min(app.world.height as u16) as usize;

    let mut lines = Vec::with_capacity(h);

    for y in 0..h {
        let mut spans = Vec::with_capacity(w);

        for x in 0..w {
            let idx = app.world.idx(x, y);
            let mut span = match app.world.cells[idx] {
                Some(cell) => {
                    let material = &app.materials[cell.material];
                    Span::styled(
                        material.symbol.to_string(),
                        Style::default().fg(material.color),
                    )
                }
                None => Span::raw(" "),
            };

            if app.active_panel == ActivePanel::Simulation
                && x == app.cursor_x.min(w.saturating_sub(1))
                && y == app.cursor_y.min(h.saturating_sub(1))
            {
                span = match app.world.cells[idx] {
                    Some(cell) => {
                        let material = &app.materials[cell.material];
                        Span::styled(
                            material.symbol.to_string(),
                            Style::default()
                                .fg(material.color)
                                .add_modifier(Modifier::REVERSED),
                        )
                    }
                    None => Span::styled(
                        "·",
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::REVERSED),
                    ),
                };
            }

            spans.push(span);
        }

        lines.push(Line::from(spans));
    }

    frame.render_widget(Paragraph::new(lines), inner);
}

fn render_sidebar(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let block = Block::default().borders(Borders::ALL).title("materials");
    frame.render_widget(block, area);

    let inner = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };

    let categories = App::categories();
    let current_category = categories[app.selected_category];
    let current_items = app.materials_in_category_indices(app.selected_category);

    let mut lines = Vec::new();

    lines.push(Line::from(vec![
        Span::styled(
            "panel: ",
            Style::default().add_modifier(Modifier::DIM),
        ),
        Span::styled(
            match app.active_panel {
                ActivePanel::Simulation => "simulation",
                ActivePanel::Materials => "materials",
            },
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ]));
    lines.push(Line::from(""));

    lines.push(Line::from(vec![Span::styled(
        "categories",
        Style::default().add_modifier(Modifier::BOLD),
    )]));

    for (i, category) in categories.iter().enumerate() {
        let selected = i == app.selected_category;
        let style = if selected && app.active_panel == ActivePanel::Materials && app.materials_focus == MaterialsFocus::Categories {
            Style::default().add_modifier(Modifier::REVERSED | Modifier::BOLD)
        } else if selected {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        lines.push(Line::from(vec![Span::styled(category.title(), style)]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![Span::styled(
        current_category.title(),
        Style::default().add_modifier(Modifier::BOLD),
    )]));

    for (i, material_idx) in current_items.iter().enumerate() {
        let material = &app.materials[*material_idx];
        let selected = *material_idx == app.selected_material;
        let focused = app.active_panel == ActivePanel::Materials
            && app.materials_focus == MaterialsFocus::Items
            && i == app.selected_in_category;

        let style = if focused {
            Style::default()
                .fg(material.color)
                .add_modifier(Modifier::REVERSED | Modifier::BOLD)
        } else if selected {
            Style::default().fg(material.color).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(material.color)
        };

        lines.push(Line::from(vec![
            Span::styled(if selected { ">" } else { " " }, style),
            Span::raw(" "),
            Span::styled(material.name, style),
            Span::raw(" "),
            Span::styled(material.symbol.to_string(), style),
            Span::raw(" ρ"),
            Span::raw(format!("{:.1}", material.density)),
            Span::raw(" μ"),
            Span::raw(format!("{}", material.viscosity)),
        ]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from("s simulation"));
    lines.push(Line::from("m materials"));
    lines.push(Line::from("enter/space spawn"));
    lines.push(Line::from("arrows move"));

    if app.active_panel == ActivePanel::Simulation {
        lines.push(Line::from(format!(
            "cursor {}:{}",
            app.cursor_x, app.cursor_y
        )));
    }

    frame.render_widget(Paragraph::new(lines), inner);
}