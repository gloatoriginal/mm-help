use mysql::*;
use mysql::prelude::*;
use std::env;
pub struct Vars {
    pub start_command: String,
}
impl Vars {
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
            Err(err) => panic!("Not sure what happened, connection crashed {}", err),
        }
    }
    fn create_var_entry(guild_id: &String) -> Vars {
        match Self::return_sql_connection()
            .query_drop(
                format!("INSERT INTO vars(server_id, start_command) VALUES('{}', '.')", guild_id)
            ) {
                Ok(ok) => println!("table created successfully"),
                Err(err) => println!("Table had issues being created: {}", err),
            }
        Vars { start_command: ".".to_string() }
    }
    pub fn get_server_vars(guild_id: &String) -> Vars {
        let vars = match Self::return_sql_connection()
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
