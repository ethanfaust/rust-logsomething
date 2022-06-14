#[macro_use]
extern crate log;

use std::{thread, time};
use std::io;
use std::io::prelude::*;
use std::fs::File;

use serde_json::json;
use serde::{Deserialize, Serialize};

fn init_logging() {
    let mut builder = env_logger::Builder::from_default_env();
    builder.target(env_logger::Target::Stderr);
    builder.format_timestamp_millis();
    builder.init();
}

fn examine_os_release() {
    let path = "/etc/os-release";
    let file = File::open(path).expect("failed to open file");
    for line in io::BufReader::new(file).lines() {
        println!("{}", line.unwrap());
    }
}

fn dump_json() {
    let mut obj = json!({
        "key": "foo",
    });
    obj.as_object_mut().unwrap()
        .insert(String::from("value"), json!("world"));
    *obj.get_mut("key").unwrap() = json!("hello");
    error!("{}", obj.to_string());
}

#[derive(Serialize, Deserialize)]
struct Json2 {
    key: String,
    value: String
}

fn dump_json2() {
    let data = r#"
        {
            "key": "wow",
            "value": "cool"
        }
    "#;
    let obj: Json2 = serde_json::from_str(data).expect("failed to parse json");
    info!("read {} = {}", obj.key, obj.value);
}

fn main() {
    init_logging();  
    info!("hello, world!");
    let mut counter: usize = 0;
    loop {
        thread::sleep(time::Duration::from_millis(100));
        info!("hello there {}", counter);
        counter += 1;

        match counter {
            5 => examine_os_release(),
            7 => dump_json(),
            9 => dump_json2(),
            _ => (),
        }
    }
}
