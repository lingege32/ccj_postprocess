use clap::{Command, Arg};
use std::path::Path;


#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug)]
struct CompileCommand {
    command: String,
    directory: String,
    file: String,
}

impl CompileCommand {
    fn postprocess(&mut self) {
        let mut arguments = self.command.split(' ').map(|x| x.into()).collect::<Vec<String>>();
        let mut hs = std::collections::HashSet::new();
        arguments.retain(|x| hs.insert(x.clone()));

    }
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
    let mut compile_commands :Vec<CompileCommand> = serde_json::from_str(&context).expect("[Error] json file parser fail!");
    for cc in &mut compile_commands {
        cc.postprocess();
    }

    println!("{:#?}", compile_commands);
}
