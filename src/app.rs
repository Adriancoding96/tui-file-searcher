use std::collections::HashMap;

/*
* Enum defines the screens of the application
*/
pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

/*
* Enum defines the states of the application
*/
pub enum CurrentlyEditing {
    Key,
    Value,
}

pub struct App {
    pub key_input: String,
    pub value_input: String,
    pub pairs: HashMap<String, String>,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
}
