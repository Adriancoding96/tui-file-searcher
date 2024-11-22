use ratatui::{layout::{Constraint, Direction, Layout, Rect}, Frame};

use crate::app::App;


/*
* Function initiates rendering of tui elements
*
* @param frame: mutable reference of a Frame from the ratatui crate
* @param app: refernece to App struct containing application state
*/
pub fn ui(frame: &mut Frame, app: &App) {
    let chuncks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());
}

/*
* Helper function used to draw tui element, the function produces
* a popup rectangle split in to three sections stacked vertically
*
* @param percent_x: contains value used to split tui Horizontaly
* @param percent_y: contains value used to split tui vertically
* @param rect: contains Rect from ratatui crate
* @return rect: returns Rect from ratatui crate
*/
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),

        ])
        .split(popup_layout[1])[1]
}
