use clap::{Arg, Command};
use relative_path::RelativePath;
use std::path::{Path};

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
        let mut arguments = self
            .command
            .split(' ')
            .map(|x| x.into())
            .collect::<Vec<String>>();
        let mut hs = std::collections::HashSet::new();

        // remove the duplicate arguments
        arguments.retain(|x| hs.insert(x.clone()));

        // handle the relative path in -I option
        for option in &mut arguments {
            if option.len() > 2 && &option[0..2] == "-I" {
                let relative_path = RelativePath::new(&option[2..]);
                let full_path = if &option[2..3] != "/" {
                    relative_path.to_logical_path(&self.directory)
                } else {
                    relative_path.to_logical_path("")
                };
                let start = if full_path.has_root() { 1 } else { 0 };
                *option = format!("-I/{}", &full_path.to_str().unwrap()[start..]);

                // println!("dir: {:?}", *option);
            }
        }

        // insert the specified option after first g++ command
        // original: g++ -o main main.cpp
        // after:    g++ -D__GNU__=10 -o main main.cpp
        let mut insert_option = vec!["-D__GNU__=10"]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        let len = insert_option.len();
        arguments.append(&mut insert_option);
        arguments[1..].rotate_right(len);

        hs.clear();
        // remove the duplicate arguments again
        arguments.retain(|x| hs.insert(x.clone()));
        // join the arguments to command
        self.command = arguments.join(" ");
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
    let mut compile_commands: Vec<CompileCommand> =
        serde_json::from_str(&context).expect("[Error] json file parser fail!");
    for cc in &mut compile_commands {
        cc.postprocess();
    }

    println!("{:#?}", compile_commands);
}
