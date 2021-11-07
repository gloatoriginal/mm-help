use mysql::*;
//use mysql::prelude::*;
use std::env;

pub fn return_sql_connection() -> PooledConn {
    let url = env::var("DBURL").expect("what happened");
    let url = Opts::from_url(&url).unwrap();
    let pool = Pool::new(url);
    let pool = pool.expect("Not sure what happened, pool crashed");
    pool.get_conn().expect("Not sure what happened connection crashed")
}