#![feature(crate_in_paths)]
#![feature(extern_prelude)]

extern crate reqwest;
extern crate serde_json;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate clap;

mod api;
mod config;

use std::{thread, time};
//use std::io::{self, Write};

use clap::App as ClapApp;
use failure::Error;
use rand::Rng;

use crate::api::Api;
use crate::config::UserConfig;

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
    }
}

fn run() -> Result<(), Error> {
    // print!("正在检测签到课程");
    // io::stdout().flush().unwrap();
    // print!("\r正在检测签到课程.");
    // io::stdout().flush().unwrap();
    // print!("\r正在检测签到课程..");
    // io::stdout().flush().unwrap();
    // print!("\r正在检测签到课程...");
    // io::stdout().flush().unwrap();

    let yaml = load_yaml!("cli.yml");
    let matches = ClapApp::from_yaml(yaml).get_matches();
    match matches.subcommand() {
        ("init", Some(_)) => {
            println!("Generate app config");

            let config = UserConfig::default();
            config
                .gnerate_config()
                .expect("Generate app config file failed.");
        }
        ("run", Some(_)) => {
            let config_info =
                UserConfig::load_config().expect("Config file is error,please reinit it !");

            let api = Api::new(config_info.user_info);

            println!("{}", "正在检测签到课程...");
            loop {
                let courses = api.get_courses()?;

                for course in courses {
                    let rollcall = api.get_rollcall(course.course_id)?;
                    if let Some(rollcall_id) = rollcall {
                        println!("{} 课程有签到！准备开始签到 ...", &course.name);

                        let status = api.mark_rollcall(rollcall_id)?;
                        if status == true {
                            print!("{}", "签到成功！");
                        } else {
                            print!("{}", "签到失败！");
                        }
                    }
                }

                let s_number: u64 = 15 + rand::thread_rng().gen_range(0, 10);
                let sleep_time = time::Duration::from_millis(s_number);
                thread::sleep(sleep_time);
            }
        }
        _ => {
            ClapApp::from_yaml(yaml).get_matches_from(&["", "-h"]);
        }
    }

    Ok(())
}
