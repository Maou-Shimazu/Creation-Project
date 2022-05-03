mod cppm;
use colored::Colorize;
use std::env;

const OPTIONS: &str = r#"OPTIONS:
    -h, --help      Displays this help message.
    -v, --version   Displays the version of this program.
    -l, --list      Lists all configured projects.

COMMANDS:
    *config         Configures cppm.
    new             Creates a new project.
    init            initializes a project in the current directory.
    open            Opens a project that was created with cppm.
    *build          Builds the project to a dist directory.
    *run            Build and Runs the project.
    *clean          Cleans the project dist.
    *remove         Removes a project from configuration.

Note: Many commands have not been implemented yet. This is a minor release, more features will be added in the future. commands with * are not yet implemented.
"#;

fn man() {
    println!("C++ Project Manager\n");
    println!("USAGE:\n     cppm [COMMANDS] [+SUBCOMMANDS] [+NESTED-SC]\n");
    println!("{}", OPTIONS);
}

fn main() {
    let _args: Vec<String> = env::args().collect();

    // note: `cppm list projects` is also a possible implimentation.
    // note: human panic
    match _args.len() {
        1 => {
            man();
        }
        2 | 3 | 4 => {
            match _args[1].as_str() {
                "-v" | "--version" => {
                    println!("{}", cppm::misc::version());
                }
                "new" => {
                    // possibly add minimal support for C
                    if _args.len() > 3 {
                        cppm::Cppm::new(_args[2].clone(), _args[3].clone());
                    } else if _args.len() > 2 {
                        cppm::Cppm::new(_args[2].clone(), "null".to_string());
                    } else {
                        println!("{}", "Error: You must provide a project name.".red());
                        return;
                    }
                    println!(
                        "    {} {} `{}`",
                        "Created".bright_green(),
                        "C++ project",
                        _args[2]
                    );
                }
                "init" => {
                    cppm::Cppm::init().ok();
                }
                "run" => (),
                "build" => (),
                "clean" => (),
                "release" => (),
                "remove" => (),
                "open" => {
                    let editor = env::var("EDITOR").unwrap_or("".to_string());
                    if _args.len() > 3 {
                        cppm::Cppm::open(_args[2].clone(), _args[3].clone());
                    } else {
                        if _args.len() == 2 {
                            println!("{}", "Error: You need to provide a project name.".red());
                            return;
                        }
                        if editor.is_empty() {
                            println!("   {}", "Please provide a text editor.".bright_red());
                            return;
                        }
                        cppm::Cppm::open(_args[2].clone(), editor.clone());
                    }
                }
                "config" => (), // todo: urgent
                "ini" => println!("{}", cppm::misc::configfile()),
                "test" => {
                }
                "--help" | "-h" | _ => man(),
            }
        }
        _ => {
            println!(
                "   {}",
                "Argument not supported, please use `cppm --help` for more info.".bright_red()
            );
        }
    }
}
