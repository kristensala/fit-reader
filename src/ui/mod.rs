//https://github.com/fdehau/tui-rs/blob/master/examples/layout.rs
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Tabs, Dataset, GraphType, Chart, Axis, Paragraph, Wrap, Table, Row, Cell},
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
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ].as_ref())
        .margin(1)
        .split(f.size());

    draw_current_year_summary(f, parent_layout[0]);

    draw_session_chart(f, parent_layout[1], app);
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


fn draw_session_chart<B: Backend>(f: &mut Frame<B>, layout: Rect, app: &App) {
    let dataset = lib::build_session_records_dataset(app);   

    let num_of_items = 5; // this is the size of the list ex 5 sessions

    // build constraints
    let c: Vec<Constraint> = (0..num_of_items).into_iter()
        .map(|_| {
            let percen = 100 / num_of_items;
            return Constraint::Percentage(percen);
        }).collect();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(c.as_ref())
        .margin(1)
        .split(layout);

    for i in 0..5 { // iterate over the latest 3-5 sessions
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
            .block(Block::default().title("Latest session").borders(Borders::ALL))
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

        f.render_widget(chart, chunks[i]);
    }
}

