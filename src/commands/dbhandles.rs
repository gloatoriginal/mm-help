use mysql::prelude::*;
use crate::sqlpool;
pub struct SoftCommands {
    pub command_name: String,
    pub command_response: String,
}
impl SoftCommands {
    fn create_server_entry(guild_id: &String) -> Vec<SoftCommands> {
        sqlpool::return_sql_connection()
            .query_drop(format!(
                    "CREATE TABLE `{}` (command_name char(50), command_response varchar(1000))",
                    guild_id)
            ).expect("Unable to create table for server"); 
            let empty_commands = SoftCommands {
                command_name: "".to_string(),
                command_response: "".to_string(),
            };
            let empty_vec: Vec<SoftCommands> = vec![empty_commands];
            empty_vec
    }
    pub fn get_soft_commands_sql(guild_id: &String) -> Vec<SoftCommands> { 
        let server_table_exists: Option<String> = sqlpool::return_sql_connection()
            .query_first(format!("SHOW TABLES LIKE '{}'", guild_id)).expect("Error showing tables"); 
        let soft_commands: Vec<SoftCommands> = match server_table_exists {
            Some(_val) => {//table exists for this server
                //println!("{}", value);
                sqlpool::return_sql_connection()
                    .query_map(
                    format!("SELECT command_name, command_response FROM `{}`", guild_id),
                    |(command_name, command_response)| {
                        SoftCommands { command_name, command_response}
                }, ).expect("Not sure what happened query_map crashed") 
            },
            None => Self::create_server_entry(&guild_id), //table doesn't exist yet
        }; 
        soft_commands
    }
}
