use clap::{Command, Arg};
use std::path::Path;


#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Debug)]
struct CompileCommand {
    command: String,
    directory: String,
    file: String,
}

fn main() {
    let matches = Command::new("ccj_postprocess")
        .version("1.0")
        .author("Toby Lin")
        .about("compile_commands.json postprocess for zebu")
        .arg(
            Arg::new("input_file")
                .short('i')
                .value_name("input")
                .long("input")
                .help("a compile_commands.json generated from vgbuild")
                .takes_value(true)
                .required(true),
        )
        .get_matches();
    let input_file = matches.value_of("input_file").unwrap();
    let path = Path::new(input_file);
    let context = std::fs::read_to_string(path).expect(&format!("cannot open the file {:?}", path));
    let compile_commands :Vec<CompileCommand> = serde_json::from_str(&context).expect("[Error] json file parser fail!");


    println!("{:#?}", compile_commands);
}
