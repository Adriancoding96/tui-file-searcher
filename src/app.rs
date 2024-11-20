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
    * @param self: contains a mutable reference to itslef
    */
    pub fn save_key_value(&mut self) {
        self.pairs.insert(self.key_input.clone(), self.value_input.clone());
        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
    }

    /*
    * Helper function to to check if values are currently being edited
    *
    * @param self: contains a mutable reference to itslef
    */
    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Key => self.currently_editing = Some(CurrentlyEditing::Value),
                CurrentlyEditing::Value => self.currently_editing = Some(CurrentlyEditing::Key),
            };
        } else {
            self.currently_editing = Some(CurrentlyEditing::Key);
        }
    }

    /*
    * Function to convert hashmap values to json, and
    * print it to the screen.
    * 
    * @return serde_json::Result: returns a empty result if successfull, if not returns an Error
    */
    pub fn print_json(&self) -> serde_json::Result<()> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{}", output);
        Ok(())
    }
}
