mod dbhandles;
pub struct Commands {
    //commands: Vec<String>,
    //message: String,
    pub response: String,
    pub is_valid: bool,
}


impl Commands {
    //hard commands set functions these are commands that are hard coded
    fn get_hard_commands() -> Vec<String> {
        vec!["commands".to_string(), "addcomm".to_string(), "remcomm".to_string(), "editcomm".to_string()]
    }
    fn valid_hard_command(msg: &str) -> bool {
        Self::get_hard_commands().iter().any(|command| command.eq(msg))
    }
    fn hard_coded_response(msg: &str, guild_id: &str) -> String {
        let response = match msg {
            "commands" => {
                let mut temp_res: String = 
                    "Command name\tCommand response\n".to_owned(); 
                temp_res.push_str("commands\tthis response\n");
                for command in &Self::get_soft_commands(&guild_id) {
                    temp_res.push_str(&command.command_name); temp_res.push_str("\t");
                    temp_res.push_str(&command.command_response); temp_res.push_str("\n");
                }
                temp_res
            },
            _ => String::from("That's not implemented yet"),
        };
        response.to_string()
    }
    //soft commands set functions these are commands created by the user
    fn get_soft_commands(guild_id: &str) -> Vec<dbhandles::SoftCommands> {
        let soft_commands_list: Vec<dbhandles::SoftCommands> = dbhandles::
                                                SoftCommands::get_soft_commands_sql(&guild_id);
        soft_commands_list
    }
    fn valid_soft_command(msg: &str, guild_id: &str) -> (dbhandles::SoftCommands, bool) {
        let soft_commands_list = Self::get_soft_commands(&guild_id);
        let empty_soft_commands = dbhandles::SoftCommands {
                        command_name: "".to_string(),
                        command_response: "".to_string(),
                    };
        for soft_command in soft_commands_list {
            if soft_command.command_name.eq(msg) {
                return (soft_command, true);
            } 
        }
        return (empty_soft_commands, false);
    }


    //create a new struct to hold commands    
    pub fn new(msg: &str, guild_id: &str) -> Commands {
        let is_valid: bool;
        let response = if Self::valid_hard_command(&msg) {
            is_valid = true;
            Self::hard_coded_response(&msg, &guild_id)
        } else {
            let soft_command: (dbhandles::SoftCommands, bool) = Self::valid_soft_command(&msg, &guild_id);
            if soft_command.1 { //is valid soft command, the 0 in the tuple will have the response
                is_valid = soft_command.1;
                soft_command.0.command_response
            } else {
                is_valid = false;
                "".to_string()
            } 
        };
        Commands {
            response: response,
            is_valid: is_valid
        }
    }

}