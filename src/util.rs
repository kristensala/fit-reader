use crate::app::App;

pub fn move_down_event(app: &mut App) {
    let max_idx = app.sessions.len();
    let index = app.selected_session_index.unwrap();

    let mut new_idx = index + 1;
    if new_idx == max_idx {
        new_idx = 0;    
    }

    app.selected_session_index = Some(new_idx);
    app.select_session(new_idx);
}

pub fn move_up_event(app: &mut App) {
    let max_idx = app.sessions.len();
    let index = app.selected_session_index.unwrap();

    let mut new_idx: usize = if index == 0 { 0 } else { index - 1 };
    if new_idx == 0 && index == 0 {
        new_idx = max_idx - 1;
    }

    app.selected_session_index = Some(new_idx);
    app.select_session(new_idx);
}
