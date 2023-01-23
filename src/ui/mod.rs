//https://github.com/fdehau/tui-rs/blob/master/examples/layout.rs
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Dataset, GraphType, Chart, Axis, ListItem, List, ListState},
    Frame, text::Span, style::{Style, Color, Modifier}, symbols::{self},
};

use crate::app::App;

pub mod util;

pub fn draw_dashboard<B: Backend>(f: &mut Frame<B>, app: &App) {
    let parent_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50), Constraint::Percentage(50),
        ].as_ref())
        .margin(1)
        .split(f.size());

    draw_summary(f, parent_layout[0], &app);

    draw_session_chart(f, parent_layout[1], &app);
}

/// Total time
/// Total distance
/// total mtb time and distance
/// total road time and distance
/// total indoor time and distance
/// bar/pie-charts or some sort of charts
fn draw_summary<B: Backend>(f: &mut Frame<B>, layout: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Summary");

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(70),
            Constraint::Percentage(30),
        ].as_ref())
        .margin(1)
        .split(layout);

    f.render_widget(block, chunks[0]);

    draw_session_list(f, chunks[1], &app);
}

fn draw_session_list<B: Backend>(f: &mut Frame<B>, layout: Rect, app: &App) {
       
    //todo: show the whole list of sessions {date - session type(indoor/road/mtb) - time(HH:MM)}
    //session.to_sring() creates the strig that I want to show in list view

    // how to handle keyevent to change session selection

    let mut state = ListState::default();
    state.select(app.selected_session_index);

    let items: Vec<ListItem> = app.sessions.iter()
        .map(|x| ListItem::new(Span::raw(x.to_string())))
        .collect();

    let list = List::new(items)
        .block(Block::default().title("Workout list").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, layout, &mut state);
}

//TODO: this will be the solution and show selected session instead
fn draw_session_chart<B: Backend>(f: &mut Frame<B>, layout: Rect, app: &App) {
    let dataset = util::build_session_dataset(app.selected_session.to_owned().unwrap());

    let datasets = vec![
        Dataset::default()
            .name("power")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Cyan))
            .data(&dataset.power),
        Dataset::default()
            .name("heart rate")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Magenta))
            .data(&dataset.heart_rate),
        Dataset::default()
            .name("Threshold power")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::White))
            .data(&dataset.threshold_power),
    ];

    let chart = Chart::new(datasets)
        .block(Block::default().title("Selected session").borders(Borders::ALL))
        .x_axis(Axis::default()
            .title(Span::styled("Time", Style::default().fg(Color::Red)))
            .style(Style::default().fg(Color::White))
            .bounds([0.0, dataset.max_x])
            .labels(["0.0".to_string(), dataset.max_x.to_string()].iter().cloned().map(Span::from).collect()))
        .y_axis(Axis::default()
            .title(Span::styled("power/heart rate", Style::default().fg(Color::Red)))
            .style(Style::default().fg(Color::White))
            .bounds([dataset.min_y, dataset.max_y])
            .labels([dataset.min_y.to_string(), dataset.max_y.to_string()].iter().cloned().map(Span::from).collect()));

    f.render_widget(chart, layout);
}

