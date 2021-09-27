pub struct Commands {
    //commands: Vec<String>,
    //message: String,
    guild_id: String,
    pub response: String,
    pub is_valid: bool,
}
mod dbhandles;

impl Commands {
    //hard commands set functions these are commands that are hard coded
    fn get_hard_commands() -> Vec<String> {
        vec!["commands".to_string(), "addcomm".to_string(), "remcomm".to_string(), "editcomm".to_string()]
    }
    fn valid_hard_command(msg: &String) -> bool {
        for command in Self::get_hard_commands() {
            match command.eq(msg) {
                true => return true,
                _ => continue,
            }
        }
        //Self::get_hard_commands().contains(msg)
        false
    }
    fn hard_coded_response(msg: &String, guild_id: &String) -> String {
        let response = match msg.as_str() {
            "commands" => {
                let mut temp_res: String = 
                    "\n\"Command name\"\t\"Command response\"\t\"Command Description\"\n".to_owned(); 
                temp_res.push_str("\"commands\"\t\"this response\"\t\"List bot and user defined commands\"");
                for command in Self::get_soft_commands(&guild_id) {
                    temp_res.push_str("\n\"");
                    temp_res.push_str(&command.command_name); temp_res.push_str("\"\t\"");
                    temp_res.push_str(&command.command_response); temp_res.push_str("\"\t\"");
                    temp_res.push_str(&command.command_description); temp_res.push_str("\"");
                }
                temp_res
            },
            _ => String::from("That's not implemented yet"),
        };
        response.to_string()
    }
    //soft commands set functions these are commands created by the user
    fn get_soft_commands(guild_id: &String) -> Vec<dbhandles::SoftCommands> {
        let soft_commands_list: Vec<dbhandles::SoftCommands> = dbhandles::
                                                SoftCommands::get_soft_commands_sql(&guild_id);
        soft_commands_list
    }
    fn valid_soft_command(msg: &String, guild_id: &String) -> (dbhandles::SoftCommands, bool) {
        let soft_commands_list = Self::get_soft_commands(&guild_id);
        let empty_soft_commands = dbhandles::SoftCommands {
                        command_name: "".to_string(),
                        command_response: "".to_string(),
                        command_description: "".to_string(),
                    };
        let return_tuple: (dbhandles::SoftCommands, bool);
        for soft_command in soft_commands_list {
            match soft_command.command_name.eq(msg) {
                true => {
                    return (soft_command, true);
                },
                _ => {
                    continue
                },
            }
        }
        return (empty_soft_commands, false);
    }


    //create a new struct to hold commands    
    pub fn new(msg: String, guild_id: String) -> Commands {
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
        //let response: String; 
        Commands {
            //commands: Self::get_commands(),
            //message: msg,
            guild_id: guild_id,
            response: response,
            is_valid: is_valid
        }
    }

}