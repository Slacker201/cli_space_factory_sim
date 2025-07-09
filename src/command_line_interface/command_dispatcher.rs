use crate::command_line_interface::command_struct::Command;
use phf::phf_map;


static COMMAND_MAP: phf::Map<&'static str, &'static [&'static str]> = phf_map! {
    "add_recipe" => &["cmd_found"],
};





pub fn parse_and_dispatch_command(cmd: &str) {
    let mut command = Command::new();
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    match parts.get(0) {
        Some(name) => {
            println!("Name Found");
            command.set_name(name.to_string());
            if COMMAND_MAP.get(&command.name()).is_none() {
                println!("MASSIVE ERRORS AHEAD");
            }
        },
        None => {
            println!("Invalid Command")
        },
    }
    
}