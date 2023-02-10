use chrono::{NaiveDateTime, Datelike, format};
use itertools::Itertools;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Dataset, GraphType, Chart, Axis, ListItem, List, ListState, Paragraph, BarChart},
    Frame, text::{Span, Spans}, style::{Style, Color, Modifier}, symbols::{self},
};

use crate::{app::App, summary::Summary};

pub mod util;

struct Total {
    overall_distance: f64,
    overall_duration: f64,
    overall_rides_count: i64,
    indoor_distance: f64,
    indoor_duration: f64,
    indoor_rides_count: i64,
    road_distance: f64,
    road_duration: f64,
    road_rides_count: i64,
    mtb_distance: f64,
    mtb_duration: f64,
    mtb_rides_count: i64,
}

impl Total {
    pub fn new() -> Self {
        //todo: get current year
        let overall = Summary::overall(2022);
        let detailed = Summary::detailed(2022);

        let mut total_overall_distance = 0.0;
        let mut total_overall_duration = 0.0;
        let mut total_overall_rides = 0;

        let mut total_indoor_distance = 0.0;
        let mut total_indoor_duration = 0.0;
        let mut total_indoor_rides = 0;

        let mut total_road_distance = 0.0;
        let mut total_road_duration = 0.0;
        let mut total_road_rides = 0;
        
        let mut total_mtb_distance = 0.0;
        let mut total_mtb_duration = 0.0;
        let mut total_mtb_rides = 0;

        if overall.is_ok() {
            let overall_data = overall.unwrap();
            total_overall_distance = overall_data.total_distance;
            total_overall_duration = overall_data.total_time;
            total_overall_rides = overall_data.rides_count;
        }

        if detailed.is_ok() {
            let details = detailed.unwrap();

            let indoor_summary = details.iter().find(|x| x.sub_sport == Some("indoor_cycling".to_string()));
            let road_summary = details.iter().find(|x| x.sub_sport == Some("road_cycling".to_string())); // don't know the string in the file
            let mtb_summary = details.iter().find(|x| x.sub_sport == Some("mountain_bike_ride".to_string())); // don't know the string in the file

            match indoor_summary {
                Some(value) => {
                    total_indoor_duration = value.total_time;
                    total_indoor_distance = value.total_distance;
                    total_indoor_rides = value.rides_count;
                },
                None => ()
            };

            match road_summary {
                Some(value) => {
                    total_road_duration = value.total_time;
                    total_road_distance = value.total_distance;
                    total_road_rides = value.rides_count;
                },
                None => ()
            };

            match mtb_summary {
                Some(value) => {
                    total_mtb_duration = value.total_time;
                    total_mtb_distance = value.total_distance;
                    total_mtb_rides = value.rides_count;
                },
                None => ()
            };
        }
        
        return Self {
            overall_distance: total_overall_distance,
            overall_duration: total_overall_duration,
            overall_rides_count: total_overall_rides,
            indoor_distance: total_indoor_distance,
            indoor_duration: total_indoor_duration,
            indoor_rides_count: total_indoor_rides,
            road_distance: total_road_distance,
            road_duration: total_road_duration,
            road_rides_count: total_road_rides,
            mtb_distance: total_mtb_distance,
            mtb_duration: total_mtb_duration, 
            mtb_rides_count: total_mtb_rides
        };
    }
}

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

fn draw_summary<B: Backend>(f: &mut Frame<B>, layout: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(70),
            Constraint::Percentage(30),
        ].as_ref())
        .margin(1)
        .split(layout);

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

    let data: Vec<(String, u64)> = app.sessions.iter()
        .map(|session| {
            let naive_datetime = NaiveDateTime::from_timestamp_opt(session.start_time, 0).unwrap();
            let duration = session.total_moving_time as u64;
            return (format!("W{}", naive_datetime.iso_week().week().to_string()), duration);
        }).collect();

    let summed: Vec<(String, u64)> = data.iter()
        .group_by(|(week, _)| week)
        .into_iter()
        .map(|(week, duration)| {
            let sum: u64  = duration.into_iter()
                .map(|(_, b)| *b)
                .collect::<Vec<u64>>()
                .iter()
                .sum();
            return (format!("{} ({})", week, util::moving_time_to_hour_minute_string(sum as f64)), sum);
        }).collect();

    let result: Vec<(&str, u64)> = summed.iter()
        .map(|(a, b)| (a.as_str(), *b))
        .collect();

    let bar_chart = BarChart::default()
        .block(Block::default().title("Weekly data by duration").borders(Borders::ALL))
        .bar_width(10)
        .bar_gap(1)
        .data(&result);

    draw_summary_section(f, chunks[0]);
    f.render_widget(bar_chart, chunks[1]);
}

fn draw_summary_section<B: Backend>(f: &mut Frame<B>, layout: Rect) {
    let total = Total::new();

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
        Spans::from(format!("Threshold power: {}", "")),
        Spans::from(""),
        Spans::from(format!("Total duration: {}", util::moving_time_to_hour_minute_string(total.overall_duration))),
        Spans::from(format!("Total distance: {}", util::distance_as_string(total.overall_distance))),
        Spans::from(format!("Total rides: {}", total.overall_rides_count)),
        Spans::from(""),
        Spans::from(format!("Total time w/o indoor: {}", "")),
        Spans::from(format!("Total distance w/o indoor: {}", "")),
    ];

    let indoor_summary_text = vec![
        Spans::from(format!("Total duration: {}", util::moving_time_to_hour_minute_string(total.indoor_duration))),
        Spans::from(format!("Total distance: {}", util::distance_as_string(total.indoor_distance))),
        Spans::from(format!("Total rides: {}", total.indoor_rides_count)),
        Spans::from(format!("AVG session duration: {}", "")),
    ];

    let road_summary_text = vec![
        Spans::from(format!("Total duration: {}", total.road_duration)),
        Spans::from(format!("Total distance: {}", total.road_distance)),
        Spans::from(format!("Total rides: {}", total.road_rides_count)),
        Spans::from(""),
        Spans::from(format!("AVG session duration: {}", "")),
        Spans::from(format!("AVG session distance: {}", "")),
    ];

    let mtb_summary_text = vec![
        Spans::from(format!("Total duration: {}", total.mtb_duration)),
        Spans::from(format!("Total distance: {}", total.mtb_distance)),
        Spans::from(format!("Total rides: {}", total.mtb_rides_count)),
        Spans::from(""),
        Spans::from(format!("AVG session duration: {}", "")),
        Spans::from(format!("AVG session distance: {}", "")),
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
        .map(|x| ListItem::new(Span::raw(util::sessio_to_string(x))))
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
        Spans::from(format!("Date: {}", util::timestamp_as_string(selected_session.start_time))),
        Spans::from(format!("Type: {}", selected_session.sub_sport)),
        Spans::from(""),
        Spans::from(format!("Duration: {}", util::moving_time_to_hour_minute_string(selected_session.total_moving_time))),
        Spans::from(format!("Distance: {}", util::distance_as_string(selected_session.total_distance))),
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

