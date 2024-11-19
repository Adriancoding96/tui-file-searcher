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

/*
* This structure contains the full application state
*/
pub struct App {
    pub key_input: String,
    pub value_input: String,
    pub pairs: HashMap<String, String>,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
}

/*
* Function creates a new application state
*
* @return app: returns a app structs containing the application state
*/
impl App {
    pub fn new() -> App {
        App {
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }

    /*
    * Function called when user saves json key values, after saving
    * it restores the application state;
    *
    * @param slef: contains a mutable reference to itslef
    */
    pub fn save_key_value(&mut self) {
        self.pairs.insert(self.key_input.clone(), self.value_input.clone());
        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
    } 

}









