use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Dataset, GraphType, Chart, Axis, ListItem, List, ListState, Paragraph},
    Frame, text::{Span, Spans}, style::{Style, Color, Modifier}, symbols::{self},
};

use crate::app::App;

pub mod util;

pub fn draw_dashboard<B: Backend>(f: &mut Frame<B>, app: &App) {
    let parent_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(40),
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
/// bar chart of last 5-10 weeks in hours
fn draw_summary<B: Backend>(f: &mut Frame<B>, layout: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(70),
            Constraint::Percentage(30),
        ].as_ref())
        .margin(1)
        .split(layout);

    //f.render_widget(block, chunks[0]);
    draw_overview_section(f, chunks[0], &app);

    draw_session_list(f, chunks[1], &app);
}

fn draw_overview_section<B: Backend>(f: &mut Frame<B>, layout: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ].as_ref())
        .split(layout);

    let last_weeks_bar_chart = Block::default()
        .borders(Borders::ALL)
        .title("Last 7 weeks");

    draw_summary_section(f, chunks[0], &app);
    f.render_widget(last_weeks_bar_chart, chunks[1]);
}

fn draw_summary_section<B: Backend>(f: &mut Frame<B>, layout: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25)
        ].as_ref())
        .split(layout);

    let overall_summary_block = Block::default()
        .borders(Borders::ALL)
        .title("Overall");

    let indoor_summary_block = Block::default()
        .borders(Borders::ALL)
        .title("Indoor");

    let road_summary_block = Block::default()
        .borders(Borders::ALL)
        .title("Road");

    let mtb_summary_block = Block::default()
        .borders(Borders::ALL)
        .title("MTB");

    let overall_summary_text = vec![
        Spans::from(""),
        Spans::from(format!("Total duration: {}", "")),
        Spans::from(format!("Total distance: {}", "")),
        Spans::from(format!("Total rides: {}", "")),
        Spans::from(""),
        Spans::from(format!("Total time w/o indoor: {}", "")),
        Spans::from(format!("Total distance w/o indoor: {}", "")),
    ];

    let indoor_summary_text = vec![
        Spans::from(format!("Total duration: {}", "")),
        Spans::from(format!("Total distance: {}", "")),
        Spans::from(format!("Total rides: {}", "")),
        Spans::from(format!("AVG session duration: {}", "")),
    ];

    let road_summary_text = vec![
        Spans::from(format!("Total duration: {}", "")),
        Spans::from(format!("Total distance: {}", "")),
        Spans::from(format!("Total rides: {}", "")),
        Spans::from(format!("AVG session duration: {}", "")),
    ];

    let mtb_summary_text = vec![
        Spans::from(format!("Total duration: {}", "")),
        Spans::from(format!("Total distance: {}", "")),
        Spans::from(format!("Total rides: {}", "")),
        Spans::from(format!("AVG session duration: {}", "")),
    ];

    let overall_summary_paragraph = Paragraph::new(overall_summary_text)
        .block(overall_summary_block);

    let indoor_summary_paragraph = Paragraph::new(indoor_summary_text)
        .block(indoor_summary_block);

    let road_summary_paragraph = Paragraph::new(road_summary_text)
        .block(road_summary_block);

    let mtb_summary_paragraph = Paragraph::new(mtb_summary_text)
        .block(mtb_summary_block);

    f.render_widget(overall_summary_paragraph, chunks[0]);
    f.render_widget(indoor_summary_paragraph, chunks[1]);
    f.render_widget(road_summary_paragraph, chunks[2]);
    f.render_widget(mtb_summary_paragraph, chunks[3]);
}

fn draw_session_list<B: Backend>(f: &mut Frame<B>, layout: Rect, app: &App) {
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

fn draw_session_chart<B: Backend>(f: &mut Frame<B>, layout: Rect, app: &App) {
    let selected_session = app.selected_session.to_owned().unwrap();
    let dataset = util::build_session_dataset(&selected_session);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(15),
            Constraint::Percentage(85)
        ].as_ref())
        .margin(1)
        .split(layout);


    let block = Block::default()
        .borders(Borders::ALL)
        .title("Data");

    let text = vec![
        Spans::from(format!("Date: {}", selected_session.timestamp_as_string())),
        Spans::from(format!("Type: {}", selected_session.sub_sport)),
        Spans::from(""),
        Spans::from(format!("Duration: {}", selected_session.moving_time_as_string())),
        Spans::from(format!("Distance: {}", selected_session.distance_as_string())),
        Spans::from(format!("AVG Heart rate: {}", selected_session.avg_heart_rate)),
        Spans::from(format!("AVG Power: {}", selected_session.avg_power)),
        Spans::from(format!("AVG Cadence: {}", selected_session.avg_cadence)),
        Spans::from(format!("Threshold power: {}", selected_session.threshold_power)),
    ];

    let paragraph = Paragraph::new(text)
        .block(block);

    let datasets = vec![
        Dataset::default()
            .name("Power")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Cyan))
            .data(&dataset.power),
        Dataset::default()
            .name("Heart rate")
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
            .labels([
                dataset.min_y.to_string(),
                dataset.max_y.to_string()
            ].iter().cloned().map(Span::from).collect()));

    f.render_widget(paragraph, chunks[0]);
    f.render_widget(chart, chunks[1]);
}

