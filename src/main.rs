#![allow(unused)]
extern crate clap;
use clap::{App, Arg, ArgMatches};
use std::fs::File;
use std::io::Read;
use std::panic::resume_unwind;

fn get_config() -> String {
    /// 获取文件中的配置数据
    let mut file = match File::open("./conf") {
        Ok(f) => f,
        Err(_) => {
            let c = File::create("./conf");
            match c {
                Ok(f) => f,
                Err(e) => panic!(format!("创建配置文件失败，{}", e)),
            }
        }
    };
    let mut content = String::new();
    let result = file.read_to_string(&mut content);
    if let Err(e) = result {
        panic!(format!("读取文件失败, {}", e))
    }
    return content;
}

fn init_app() -> ArgMatches<'static> {
    // 初始化App，并返回ArgMatches实例
    App::new("workon")
        .version("0.0.1")
        .author("zhaolei")
        .about("workon可以用来激活Python虚拟环境，设置路径和虚拟环境的映射，在使用workon激活虚拟环境是将自动进入配置的项目目录。")
        .arg(
            Arg::with_name("set env name")
                .long("set")
                .short("s")
                .takes_value(true)
                .help("为当前目录设置虚拟环境，激活环境或进入当前目录自动激活环境"),
        )
        .arg(
            Arg::with_name("remove env name")
                .long("remove")
                .short("r")
                .takes_value(true)
                .help("删除指定环境配置，可通过--show查看已经添加的全部配置信息"),
        )
        .arg(
            Arg::with_name("clean env name")
                .long("clean")
                .short("c")
                .takes_value(true)
                .help("清除所有的配置信息"),
        )
        .arg(Arg::with_name("show")
            .long("show")
            .help("显示所有的配置")
        )
        .help_message("打印帮助信息")
        .version_message("显示版本信息")
        .get_matches()
}

fn dispatch_arg(matches: ArgMatches) {
    if let Some(r) = matches.value_of("set env name") {
        println!("set env name: {}", r);
    } else if let Some(r) = matches.value_of("remove env name") {
        println!("remove env name: {}", r);
    } else if let Some(r) = matches.value_of("clean env name") {
        println!("clean env name: {}", r);
    } else if matches.is_present("show") {
        println!("show config");
    }
}

fn main() {
    let matches = init_app();
    dispatch_arg(matches);
}
