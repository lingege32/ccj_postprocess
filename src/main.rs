use clap::{Arg, Command};
use relative_path::RelativePath;
use std::path::Path;
use std::collections::HashSet;
#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug)]
struct CompileCommand {
    command: String,
    directory: String,
    file: String,
}

impl CompileCommand {
    pub fn postprocess(&mut self) {
        let mut arguments = self
            .command
            .split(' ')
            .map(|x| x.into())
            .collect::<Vec<String>>();

        Self::remove_duplicate_option(&mut arguments);
        Self::handle_include_path(&mut arguments, &self.directory);

        let insert_option = vec!["-D__GNUC__=10", "-I/remote/vgrnd106/chielin/local/boost/boost_1_78_0"]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        Self::insert_needed_option(&mut arguments, insert_option);
        
        let remove_option: HashSet<_> = vec!["-fconcepts"]
        .iter()
        .map(|x| x.to_string())
        .collect();
        Self::remove_option(&mut arguments, remove_option);

        Self::remove_duplicate_option(&mut arguments);

        // join the arguments to command
        self.command = arguments.join(" ");
    }

    fn remove_duplicate_option(arguments: &mut Vec<String>) {
        let mut hs = std::collections::HashSet::new();
        // remove the duplicate arguments
        arguments.retain(|x| hs.insert(x.clone()));
    }

    fn handle_include_path(arguments: &mut Vec<String>, base_directory: &str) {
        // handle the relative path in -I option
        for option in arguments {
            if option.len() > 2 && &option[0..2] == "-I" {
                let relative_path = RelativePath::new(&option[2..]);
                let full_path = if &option[2..3] != "/" {
                    relative_path.to_logical_path(base_directory)
                } else {
                    relative_path.to_logical_path("")
                };
                let start = if full_path.has_root() { 1 } else { 0 };
                *option = format!("-I/{}", &full_path.to_str().unwrap()[start..]);

                // println!("dir: {:?}", *option);
            }
        }
    }

    fn insert_needed_option(arguments: &mut Vec<String>, mut insert_options: Vec<String>) {
        // insert the specified option after first g++ command
        // original: g++ -o main main.cpp
        // after:    g++ -D__GNU__=10 -o main main.cpp
        let len = insert_options.len();
        arguments.append(&mut insert_options);
        arguments[1..].rotate_right(len);
    }

    fn remove_option(arguments: &mut Vec<String>, remove_options: HashSet<String>) {
        arguments.retain(|x| !remove_options.contains(x));
    }

    fn dump_ccj(&self) {
        println!("{}",serde_json::to_string_pretty(self).unwrap());
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

    println!("[");
    compile_commands[0].dump_ccj();
    for cc in &compile_commands[1..] {
        println!(",");
        cc.dump_ccj();
    }
    println!("]");

}
