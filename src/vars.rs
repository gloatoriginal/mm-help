use mysql::prelude::*;
use crate::sqlpool;
pub struct Vars {
    pub start_command: String,
}
impl Vars {
    fn create_var_entry(guild_id: &String) -> Vars {
        match sqlpool::return_sql_connection()
            .query_drop(
                format!("INSERT INTO vars(server_id, start_command) VALUES('{}', '.')", guild_id)
            ) {
                Ok(ok) => println!("var row created sucessfully for {}, Mariadb response:\n{:?}", guild_id, ok),
                Err(err) => println!("Table had issues being created: {}", err),
            }
        Vars { start_command: ".".to_string() }
    }
    pub fn get_server_vars(guild_id: &String) -> Vars {
        let vars = match sqlpool::return_sql_connection()
            .query_map(
                format!("SELECT start_command FROM vars WHERE server_id like '{}'", guild_id),
                |start_command| { Vars { start_command } },
            ) { //start of match statement
                Ok(start_command) => start_command,
                Err(err) => panic!("Not sure what happened selecting Vars: {}", err),
            };
        if vars.len() == 0 {
            return Self::create_var_entry(&guild_id);
        } else {
            return Vars {
                start_command: vars[0].start_command.to_string(),
            }
        }

    }
    
}
