use mysql::*;
use mysql::prelude::*;
use std::env;
pub struct SoftCommands {
    pub command_name: String,
    pub command_response: String,
    pub command_description: String,
}
impl SoftCommands {
    fn return_sql_connection() -> PooledConn {
        let url = env::var("DBURL").expect("what happened");
        let url = Opts::from_url(&url).unwrap();
        let pool = Pool::new(url);
        let pool = match pool {
            Ok(pool) => pool,
            Err(err) => panic!("Not sure what happened, pool crashed: {}", err),
        };
        match pool.get_conn() {
            Ok(conn) => conn,
            Err(err) => panic!("Not sure what happened, connection crashed: {}", err),
        }
    }
    fn create_server_entry(guild_id: &String) -> Vec<SoftCommands> {
        match Self::return_sql_connection()
            .query_drop(
                format!(
                    "CREATE TABLE `{}` (command_name char(50), command_response varchar(255), command_description varchar(255))"
                    , guild_id)
            ) {
                Ok(ok) => println!("Table created successfully for sever id: {}", guild_id),
                Err(err) => println!("Unable to create table for server id: {}, error: {}", guild_id, err),
            }
            let empty_commands = SoftCommands {
                command_name: "".to_string(),
                command_response: "".to_string(),
                command_description: "".to_string(),
            };
            let empty_vec: Vec<SoftCommands> = vec![empty_commands];
            empty_vec
    }
    pub fn get_soft_commands_sql(guild_id: &String) -> Vec<SoftCommands> { 
        let server_table_exists: Option<String> = match Self::return_sql_connection()
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
            Some(value) => {//table exists for this server
                //println!("{}", value);
                match Self::return_sql_connection()
                    .query_map(
                    format!("SELECT command_name, command_response, command_description FROM `{}`", guild_id),
                    |(command_name, command_response, command_description)| {
                        SoftCommands { command_name, command_response, command_description }
                },
                ) { //start of match statement
                Ok(soft_commands) => return soft_commands,
                Err(err) => panic!("Not sure what happened, query_map crashed: {}", err),
                };
            },
            None => {//table doesn't exist yet
                Self::create_server_entry(&guild_id)
            },
        };
        //println!("{:?}", server_table_exists.unwrap());
        /*let soft_commands: Vec<SoftCommands> = match Self::return_sql_connection()
                .query_map(
                format!("SELECT command_name, command_response, command_description FROM `{}`", guild_id),
                |(command_name, command_response, command_description)| {
                    SoftCommands { command_name, command_response, command_description }
            },
        ) { //start of match statement
            Ok(soft_commands) => soft_commands,
            Err(err) => panic!("Not sure what happened, query_map crashed: {}", err),
        };
        let soft_commands = if soft_commands.len() == 0 {
            Self::create_server_entry(&guild_id)
        } else { soft_commands };*/
        soft_commands
    }
}
