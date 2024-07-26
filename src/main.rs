use clap::{command, Arg, ArgAction, Command};

mod commands;
mod utils;

const VERSION_STR: &str = "v0.1a";

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("get")
                .alias("install")
                .alias("i")
                .about("Install plugin")
                .arg(Arg::new("plugin").required(true))
                .arg(
                    Arg::new("repo")
                        .short('r')
                        .long("repo")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("remove")
                .alias("uninstall")
                .about("Uninstall plugin"),
        )
        .subcommand(Command::new("info").about("Get plugin info"))
        .version(VERSION_STR)
        .arg_required_else_help(true)
        .get_matches();

    if let Some((name, submatches)) = matches.subcommand() {
        let _ = match name {
            "get" => {
                let plugin = submatches.get_one::<String>("plugin").unwrap();
                let is_repo = submatches.get_flag("repo");

                commands::get_plugin(plugin, is_repo)
            }
            "remove" => {
                todo!("implement remove")
            }
            "info" => {
                todo!("implement remove")
            }
            _ => {
                println!("none");
                Ok(())
            }
        };
    }
}
