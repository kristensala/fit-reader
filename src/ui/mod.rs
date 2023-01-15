//https://github.com/fdehau/tui-rs/blob/master/examples/layout.rs
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Table, Tabs, Dataset, GraphType, Chart, Axis, Paragraph, Wrap},
    Frame, text::{Spans, Span}, style::{Style, Color}, symbols::{DOT, self},
};

use crate::app::App;

pub mod lib;

pub fn main_layout<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(f.size());

    let block = Block::default().title("Block").borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
    let block = Block::default().title("Block 2").borders(Borders::ALL);
    f.render_widget(block, chunks[2]);
}

pub fn draw_tabs<B: Backend>(f: &mut Frame<B>, layout: Rect) {
    let titles = ["Dashboard", "Sessions"].iter().cloned().map(Spans::from).collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().title("Tabs").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(DOT);

    f.render_widget(tabs, layout);
}

pub fn draw_dashboard<B: Backend>(f: &mut Frame<B>, app: &App) {
    let parent_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(10)
        ].as_ref())
        .margin(1)
        .split(f.size());

    draw_tabs(f, parent_layout[0]);

    draw_account_stats(f, parent_layout[1]);

    draw_current_year_summary(f, parent_layout[2]);

    //draw_last_workout_section(f, parent_layout[3]);

    draw_session_chart(f, parent_layout[3], app);

}

/// Account name
/// Age
/// Weight
/// FTP
/// Power zones
fn draw_account_stats<B: Backend>(f: &mut Frame<B>, layout: Rect) {
    let user_data = vec![
        Spans::from(Span::styled("Name: Kristen Sala", Style::default())),
        Spans::from(Span::styled("FTP: 264", Style::default())),
    ];

    let playing_paragraph = Paragraph::new(user_data)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    "User",
                    Style::default(),
                ))
                .border_style(Style::default()),
        );

    f.render_widget(playing_paragraph, layout);
}

/// Total time
/// Total distance
/// total mtb time and distance
/// total road time and distance
/// total indoor time and distance
/// bar/pie-charts or some sort of charts
fn draw_current_year_summary<B: Backend>(f: &mut Frame<B>, layout: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Summary");

    f.render_widget(block, layout);
}

/// Latest workout graph
/// and general data about that session
fn draw_last_workout_section<B: Backend>(f: &mut Frame<B>, layout: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Latest workout");

    f.render_widget(block, layout);
}

fn draw_session_chart<B: Backend>(f: &mut Frame<B>, layout: Rect, app: &App) {
    let dataset = lib::build_session_records_dataset(app);   

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
        .block(Block::default().title("Chart"))
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
