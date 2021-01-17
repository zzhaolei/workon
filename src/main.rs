extern crate clap;
use clap::{App, Arg, ArgMatches, Shell};
use serde_json::Value;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::exit;

const CONFIG_PATH: &str = "/Users/zhaolei/Codespaces/Code/workon-rs/src";
const CONFIG_FILE: &str = "/Users/zhaolei/Codespaces/Code/workon-rs/src/conf";

#[allow(unused)]
enum ErrorCode {
    CreateFileError = 1,
    OpenFileError,
    WriteFileError,
    ReadFileError,
    NotFoundFileError,
    NotFoundConfigError,
    ParseConfigError,
    SetConfigError,
}

fn get_config() -> String {
    let mut file = match File::open(CONFIG_FILE) {
        Ok(f) => f,
        Err(_) => {
            let c = File::create(CONFIG_FILE);
            match c {
                Ok(f) => f,
                Err(e) => {
                    println!("创建配置文件失败: {}", e);
                    exit(ErrorCode::CreateFileError as i32);
                }
            }
        }
    };
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Err(e) => {
            println!("读取文件失败: {}", e);
            exit(ErrorCode::ReadFileError as i32);
        }
        Ok(_) => {}
    }
    if content.is_empty() {
        content = "{}".to_owned();
    }
    content
}

fn parse_config() -> Value {
    let config = get_config();
    match serde_json::from_str(&config) {
        Ok(v) => v,
        Err(e) => {
            println!("解析配置失败: {}", e);
            exit(ErrorCode::ParseConfigError as i32);
        }
    }
}

fn init_app() -> ArgMatches<'static> {
    // 初始化App，并返回ArgMatches实例
    let mut app = App::new("workon")
        .version("0.0.1")
        .author("zhaolei")
        .about("workon可以用来激活Python虚拟环境，设置路径和虚拟环境的映射，在使用workon激活虚拟环境是将自动进入配置的项目目录。")
        .arg(
            Arg::with_name("set_env_name")
                .long("set")
                .short("s")
                .takes_value(true)
                .help("为当前目录设置虚拟环境，激活环境或进入当前目录自动激活环境"),
        )
        .arg(
            Arg::with_name("get_env_config")
                .long("get")
                .short("g")
                .takes_value(true)
                .help("获取虚拟环境配置的路径，如果未配置则未空"),
        )
        .arg(
            Arg::with_name("remove_env_name")
                .long("remove")
                .short("r")
                .takes_value(true)
                .help("删除指定环境配置，可通过--show查看已经添加的全部配置信息"),
        )
        .arg(
            Arg::with_name("clean_env_name")
                .long("clean")
                .help("清除所有的配置信息"),
        )
        .arg(Arg::with_name("show")
            .long("show")
            .help("显示所有的配置")
        )
        .help_message("打印帮助信息")
        .version_message("显示版本信息");

    app.gen_completions("workon", Shell::Fish, CONFIG_PATH);
    app.get_matches()
}

fn dispatch(matches: ArgMatches) {
    if let Some(env) = matches.value_of("set_env_name") {
        set(env);
    } else if let Some(env) = matches.value_of("get_env_config") {
        get(env);
    } else if let Some(env) = matches.value_of("remove_env_name") {
        remove(env);
    } else if matches.is_present("clean_env_name") {
        clean();
    } else if matches.is_present("show") {
        show();
    }
}

fn set(env: &str) {
    let mut config = parse_config();
    match env::current_dir() {
        Ok(path) => {
            config
                .as_object_mut()
                .unwrap()
                .insert(env.to_owned(), Value::from(path.to_str().unwrap()));
        }
        Err(e) => {
            println!("无法为当前目录设置虚拟环境({}): {}", env, e);
            exit(ErrorCode::SetConfigError as i32);
        }
    }

    let json_string = serde_json::to_string_pretty(&config).expect("期望接收一个JSON类型的数据");
    match File::create(CONFIG_FILE) {
        Ok(mut f) => {
            let _ = f.write(json_string.as_bytes());
        }
        Err(_) => return,
    }
    println!("配置 {} 已添加", env);
}

fn get(env: &str) {
    let config = parse_config();
    if let Some(s) = config.get(env) {
        println!("{}", s.as_str().unwrap());
    } else {
        exit(ErrorCode::NotFoundFileError as i32);
    }
}

fn remove(env: &str) {
    let mut config = parse_config();
    if config.get(env).is_none() {
        println!("配置 {} 不存在!", env);
        exit(1);
    }
    config.as_object_mut().unwrap().remove(env);
    let json_string = serde_json::to_string_pretty(&config).expect("期望接收一个JSON类型的数据");
    match File::create(CONFIG_FILE) {
        Ok(mut f) => {
            let _ = f.write(json_string.as_bytes());
        }
        Err(_) => return,
    }
    println!("配置 {} 已删除", env);
}

fn clean() {
    if Path::new(CONFIG_FILE).exists() {
        match File::create(CONFIG_FILE) {
            Ok(mut f) => {
                let _ = f.write("{}".as_bytes());
            }
            Err(e) => {
                println!("异常: {}", e);
                return;
            }
        }
    }
    println!("配置已清除");
}

fn show() {
    let config = parse_config();
    for (key, value) in config.as_object().unwrap().iter() {
        println!("{}:\t{}\n", key, value.as_str().unwrap());
    }
}

fn main() {
    let matches = init_app();
    dispatch(matches);
}
