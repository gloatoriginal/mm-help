pub struct Commands {
    //commands: Vec<String>,
    pub response: String,
    pub is_valid: bool,
}

impl Commands {
    fn get_response(valid: bool) -> String {
        match valid {
            true => "This command exists".to_string(),
            _ => "What happened?".to_string(),
        }
    }
    fn valid_command(commands: Vec<String>, msg: &String) -> bool {
       commands.contains(msg)
    }
    pub fn get_commands() -> Vec<String> {
        vec!["help".to_string(), "commands".to_string(), "addcomm".to_string(), "remcomm".to_string()]
    }
    //create a new struct to hold commands    
    pub fn new(msg: String) -> Commands {
        let is_valid = Self::valid_command(Self::get_commands(), &msg);
        Commands {
            //commands: Self::get_commands(),
            response: Self::get_response(is_valid),
            is_valid: is_valid
        }
    }

}

