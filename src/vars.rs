use mysql::prelude::*;
use crate::sqlpool;
pub struct Vars {
    pub start_command: String,
}
impl Vars {
    fn create_var_entry(guild_id: &String) -> Vars {
        sqlpool::return_sql_connection()
            .query_drop(
                format!("INSERT INTO vars(server_id, start_command) VALUES('{}', '.')", guild_id)
            ).expect("Table had issues being created"); 
        Vars { start_command: ".".to_string() }
    }
    pub fn get_server_vars(guild_id: &String) -> Vars {
        let vars = sqlpool::return_sql_connection()
            .query_map(
                format!("SELECT start_command FROM vars WHERE server_id like '{}'", guild_id),
                |start_command| { Vars { start_command } },
            ).expect("Not sure what happened selecting Vars:");
        if vars.len() == 0 {
            return Self::create_var_entry(&guild_id);
        } else {
            return Vars {
                start_command: vars[0].start_command.to_string(),
            }
        }

    }
    
}
