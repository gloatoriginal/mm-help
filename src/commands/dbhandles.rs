use mysql::prelude::*;
use crate::sqlpool;
pub struct SoftCommands {
    pub command_name: String,
    pub command_response: String,
}
impl SoftCommands {
    fn create_server_entry(guild_id: &String) -> Vec<SoftCommands> {
        match sqlpool::return_sql_connection()
            .query_drop(format!(
                    "CREATE TABLE `{}` (command_name char(50), command_response varchar(1000))",
                    guild_id)
            ) {
                Ok(ok) => println!("Table created successfully for sever id: {}, Mariadb response:\n{:?}", guild_id, ok),
                Err(err) => println!("Unable to create table for server id: {}, error: {}", guild_id, err),
            }
            let empty_commands = SoftCommands {
                command_name: "".to_string(),
                command_response: "".to_string(),
            };
            let empty_vec: Vec<SoftCommands> = vec![empty_commands];
            empty_vec
    }
    pub fn get_soft_commands_sql(guild_id: &String) -> Vec<SoftCommands> { 
        let server_table_exists: Option<String> = match sqlpool::return_sql_connection()
            .query_first(format!("SHOW TABLES LIKE '{}'", guild_id))
            {
                Ok(result) => {
                    result
                }
                Err(err) => {
                    panic!("{}", err);
                },
            };
        let soft_commands: Vec<SoftCommands> = match server_table_exists {
            Some(_val) => {//table exists for this server
                //println!("{}", value);
                match sqlpool::return_sql_connection()
                    .query_map(
                    format!("SELECT command_name, command_response FROM `{}`", guild_id),
                    |(command_name, command_response)| {
                        SoftCommands { command_name, command_response}
                },
                ) { //start of match statement
                    Ok(soft_commands) => return soft_commands,
                    Err(err) => panic!("Not sure what happened, query_map crashed: {}", err),
                };
            },
            None => Self::create_server_entry(&guild_id), //table doesn't exist yet
        }; 
        soft_commands
    }
}
