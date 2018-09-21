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

use clap::App as ClapApp;
use crate::api::Api;
use crate::config::UserConfig;
use rand::Rng;
use std::{thread, time};
//use std::io::{self, Write};

fn main() {
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
            println!("{}", "Generate app config");
            let config: UserConfig = Default::default();
            config
                .gnerate_config()
                .expect("Generate app config file failed.");
        }
        ("run", Some(_)) => {
            let config: UserConfig = Default::default();
            let config_info = config.load_config().expect("Config file is error,please reinit it !");
            let user_id = config_info.user_id;
            let access_token = config_info.access_token;
            let api = Api::new(user_id, access_token);
            println!("{}", "正在检测签到课程...");
            loop {
                match api.get_courses() {
                    Err(err) => panic!("{}", err),
                    Ok(map) => {
                        for (id, name) in map {
                            match api.get_rollcall(id) {
                                Err(err) => println!("{}", err),
                                Ok(s) => {
                                    if let Some(rollcall_id) = s {
                                        println!(
                                            "{} 课程有签到！准备开始签到 ...",
                                            &name
                                        );
                                        match api.mark_rollcall(rollcall_id) {
                                            Err(err) => panic!("{}", err),
                                            Ok(status) => {
                                                if status == true {
                                                    print!("{}", "签到成功！");
                                                } else {
                                                    print!("{}", "签到失败！");
                                                }
                                            }
                                        }
                                    }
                                }
                            }
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
}
