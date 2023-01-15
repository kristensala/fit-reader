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
            Constraint::Percentage(20),
            Constraint::Percentage(50)
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
    // move this to lib.rs
    let records = app.latest_session.to_owned().unwrap().records;
   
    let mut power: [(f64, f64); 200] = [(0.0, 0.0); 200];
    let mut heart_rate: [(f64, f64); 200] = [(0.0, 0.0); 200];

    for (idx, item) in records.iter().enumerate() {
        power[idx] = (60.0 * idx as f64, item.power as f64);  
        heart_rate[idx] = (60.0 * idx as f64, item.heart_rate as f64);  
    }

    // get bounds
    let min_value_y = records.iter()
        .map(|x| x.heart_rate)
        .collect::<Vec<i64>>()
        .iter()
        .min()
        .unwrap()
        .to_owned() as f64;

    let max_value_y = records.iter()
        .map(|x| x.power)
        .collect::<Vec<i64>>()
        .iter()
        .max()
        .unwrap()
        .to_owned() as f64;

    let datasets = vec![
        Dataset::default()
            .name("power")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Cyan))
            .data(&power),
        Dataset::default()
            .name("heart rate")
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::Magenta))
            .data(&heart_rate),
    ];
    let chart = Chart::new(datasets)
        .block(Block::default().title("Chart"))
        .x_axis(Axis::default()
            .title(Span::styled("Time", Style::default().fg(Color::Red)))
            .style(Style::default().fg(Color::White))
            .bounds([0.0, 9000.0])
            .labels(["0.0", "9000.0"].iter().cloned().map(Span::from).collect()))
        .y_axis(Axis::default()
            .title(Span::styled("power/heart rate", Style::default().fg(Color::Red)))
            .style(Style::default().fg(Color::White))
            .bounds([min_value_y, max_value_y])
            .labels([min_value_y.to_string(), max_value_y.to_string()].iter().cloned().map(Span::from).collect()));

    f.render_widget(chart, layout);
}
