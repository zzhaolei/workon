extern crate clap;
use clap::{App, Arg, ErrorKind, Shell, SubCommand};
use serde_json::Value;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::{env, fs, io};
use workon_rs::init;

#[allow(unused)]
const RETURN_OK: i32 = 0;
const RETURN_ERROR: i32 = 1;

fn main() {
    let app = init_app();
    dispatch(app.clone());
}

fn init_app() -> App<'static, 'static> {
    let shell_arg = Arg::with_name("shell")
        .value_name("SHELL")
        .help("当前运行的shell，支持: Fish")
        .required(true);

    let env_arg = Arg::with_name("env_name")
        .value_name("ENV_NAME")
        .help("虚拟环境名称")
        .required(true);

    // 初始化App，并返回App实例
    App::new("workon")
        .version("0.0.1")
        .author("im.zhaolei@foxmail.com")
        .about("workon可以用来激活Python虚拟环境，设置路径和虚拟环境的映射，在使用workon激活虚拟环境是将自动进入配置的项目目录。")
        .subcommand(
            SubCommand::with_name("--init")
                .about("打印用于执行workon的shell函数")
                .arg(&shell_arg)
        )
        .subcommand(
            SubCommand::with_name("--completions")
                .about("在标准输出中打印生成的workon补全信息")
                .arg(
                    Arg::with_name("shell")
                        .takes_value(true)
                        .possible_values(&Shell::variants())
                        .help("生成补全")
                        .value_name("SHELL")
                        .required(true)
                ),
        )
        .subcommand(
            SubCommand::with_name("--set")
                .about("为当前目录设置虚拟环境，激活环境或进入当前目录自动激活环境")
                .arg(&env_arg)
        )
        .subcommand(
            SubCommand::with_name("--get")
                .about("获取虚拟环境配置的路径，如果未配置则未空")
                .arg(&env_arg)
        )
        .subcommand(
            SubCommand::with_name("--remove")
                .about("删除指定环境配置，可通过 show 子命令查看已经添加的全部配置信息")
                .arg(&env_arg)
        )
        .subcommand(
            SubCommand::with_name("--clean")
                .about("清除所有的配置信息")
        )
        .subcommand(
            SubCommand::with_name("--show")
                .about("显示所有的配置")
        )
        .help_message("打印帮助信息")
        .version_message("显示版本信息")
}

fn get_config_file_and_dir() -> (PathBuf, String) {
    let home_dir = dirs::home_dir().unwrap();
    let config_dir = home_dir.as_path().join(".config");

    let work_dir = config_dir.join("workon");
    let work_dir = work_dir.as_path();

    let config_file = work_dir.join("config");
    let config_file = config_file.to_str().unwrap();
    (work_dir.to_owned(), config_file.to_owned())
}

fn get_config() -> String {
    let (work_dir, config_file) = get_config_file_and_dir();
    let config_file = config_file.as_str();
    let mut file = match File::open(config_file) {
        Ok(f) => f,
        Err(_) => {
            if !work_dir.exists() {
                let _ = fs::create_dir(work_dir.to_str().unwrap());
            }
            let c = File::create(config_file);
            match c {
                Ok(f) => f,
                Err(e) => {
                    println!("创建配置文件失败: {}", e);
                    exit(RETURN_ERROR);
                }
            }
        }
    };
    let mut content = String::new();
    if let Err(e) = file.read_to_string(&mut content) {
        println!("读取文件异常: {}", e);
        exit(RETURN_ERROR);
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
            exit(RETURN_ERROR);
        }
    }
}

fn dispatch(mut app: App) {
    let matches_safe = app.clone().get_matches_safe();
    let matches = match matches_safe {
        Ok(m) => m,
        Err(e) => {
            match e.kind {
                ErrorKind::MissingRequiredArgument => {
                    println!("必需参数未指定，尝试使用--help获取帮助");
                }
                _ => {
                    println!("{}", e);
                }
            }
            exit(RETURN_ERROR);
        }
    };
    match matches.subcommand() {
        ("--init", Some(sub_m)) => {
            let shell_name = sub_m.value_of("shell").expect("指定shell");
            init::init_main(shell_name);
        }
        ("--completions", Some(sub_m)) => {
            let shell: Shell = sub_m
                .value_of("shell")
                .expect("shell名称未指定")
                .parse()
                .expect("shell不可用");

            app.gen_completions_to("workon", shell, &mut io::stdout().lock());
            match shell {
                Shell::Fish => {
                    let complete = r#"complete -c workon -x -a "(ls -D $HOME/{.virtualenvs,Library/Caches/pypoetry/virtualenvs}/ 2> /dev/null | grep -v ':')""#;
                    println!("{}", complete);
                }
                Shell::Zsh => {}
                _ => {}
            }
        }
        ("--set", Some(sub_m)) => {
            let env_name = sub_m.value_of("env_name").expect("请指定虚拟环境名称");
            set(env_name);
        }
        ("--get", Some(sub_m)) => {
            let env_name = sub_m.value_of("env_name").expect("请指定虚拟环境名称");
            get(env_name);
        }
        ("--remove", Some(sub_m)) => {
            let env_name = sub_m.value_of("env_name").expect("请指定虚拟环境名称");
            remove(env_name);
        }
        ("--clean", Some(_)) => {
            clean();
        }
        ("--show", Some(_)) => {
            show();
        }
        (_command, _) => {
            // unreachable!("Invalid subcommand: {}", command)
            let _ = app.print_help();
        }
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
            exit(RETURN_ERROR);
        }
    }

    let json_string = serde_json::to_string_pretty(&config).expect("期望接收一个JSON类型的数据");
    let (_, config_file) = get_config_file_and_dir();
    match File::create(config_file) {
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
        if let Ok(current_dir) = env::current_dir() {
            let config_dir = s.as_str().unwrap();
            if config_dir != current_dir.to_str().unwrap() {
                println!("{}", s.as_str().unwrap());
            }
        }
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
    let (_, config_file) = get_config_file_and_dir();
    match File::create(config_file) {
        Ok(mut f) => {
            let _ = f.write(json_string.as_bytes());
        }
        Err(_) => return,
    }
    println!("配置 {} 已删除", env);
}

fn clean() {
    let (_, config_file) = get_config_file_and_dir();
    let config_file = config_file.as_str();
    if Path::new(config_file).exists() {
        match File::create(config_file) {
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
    println!("配置:");
    println!("{{");
    for (key, value) in config.as_object().unwrap().iter() {
        println!("    \"{}\": \"{}\",", key, value.as_str().unwrap());
    }
    println!("}}");
}
