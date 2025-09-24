#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::env;

use log::info;
use rust_inquire::init;

#[test]
fn it_env_test() {
    init();

    let my_host = env::var("MYSQL_HOST");
    info!("{}", my_host.unwrap());
    
}