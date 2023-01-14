//https://github.com/fdehau/tui-rs/blob/master/examples/layout.rs
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Table, Tabs, Dataset, GraphType, Chart, Axis},
    Frame, text::{Spans, Span}, style::{Style, Color}, symbols::{DOT, self},
};

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


pub fn draw_dashboard<B: Backend>(f: &mut Frame<B>) {
    let parent_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(20),
            Constraint::Percentage(70)
        ].as_ref())
        .margin(2)
        .split(f.size());

    draw_tabs(f, parent_layout[0]);

    draw_current_year_summary(f, parent_layout[1]);

    draw_last_workout_section(f, parent_layout[2]);

}

/// Account name
/// Age
/// Weight
/// FTP
/// Power zones
fn draw_account_stats<B: Backend>(f: &mut Frame<B>, layout: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("User");

    f.render_widget(block, layout);
}

/// Total time
/// Total distance
/// total mtb time and distance
/// total road time and distance
/// total indoor time and distance
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


fn chart_example<B: Backend>(f: &mut Frame<B>, layout: Rect) {
    let datasets = vec![
        Dataset::default()
            .name("data1")
            .marker(symbols::Marker::Dot)
            .graph_type(GraphType::Scatter)
            .style(Style::default().fg(Color::Cyan))
            .data(&[(0.0, 5.0), (1.0, 6.0), (1.5, 6.434)]),
        Dataset::default()
            .name("data2")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Magenta))
            .data(&[(4.0, 5.0), (5.0, 8.0), (7.66, 13.5)]),
    ];
    let chart = Chart::new(datasets)
        .block(Block::default().title("Chart"))
        .x_axis(Axis::default()
            .title(Span::styled("X Axis", Style::default().fg(Color::Red)))
            .style(Style::default().fg(Color::White))
            .bounds([0.0, 10.0])
            .labels(["0.0", "5.0", "10.0"].iter().cloned().map(Span::from).collect()))
        .y_axis(Axis::default()
            .title(Span::styled("Y Axis", Style::default().fg(Color::Red)))
            .style(Style::default().fg(Color::White))
            .bounds([0.0, 10.0])
            .labels(["0.0", "5.0", "10.0"].iter().cloned().map(Span::from).collect()));

    f.render_widget(chart, layout);
}
