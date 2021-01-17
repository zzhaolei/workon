use std::env;

const FISH_INIT: &str = include_str!("workon.fish");

fn get_workon_path() -> String {
    let path = env::current_exe().unwrap();
    path.to_str().unwrap().to_owned()
}

pub fn init_main(shell_name: &str) {
    let workon_path = get_workon_path();

    match shell_name {
        "fish" => print_script(FISH_INIT, workon_path.as_str()),
        _ => {
            println!(
                "printf \"Shell name detection failed on phase two init.\\n\
                 This probably indicates a bug within starship: please open\\n\
                 an issue at https://github.com/starship/starship/issues/new\\n\""
            );
        }
    }
}

fn print_script(script: &str, path: &str) {
    let workon_path_string = format!("\"{}\"", path);
    let script = script.replace("::WORKON::", &workon_path_string);
    print!("{}", script);
}
