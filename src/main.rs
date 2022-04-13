use clap::{Arg, Command};
use rayon::prelude::*;
use relative_path::RelativePath;
use std::path::Path;
#[macro_use]
extern crate serde_derive;
#[derive(Serialize, Deserialize, Debug)]
struct CompileCommand {
    command: String,
    directory: String,
    file: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct PostProcessConfig {
    remove: Vec<String>,
    insert: Vec<String>,
    replace: Vec<String>,
}

impl CompileCommand {
    pub fn postprocess(&mut self, pp_config: &Option<PostProcessConfig>) {
        let mut arguments = self
            .command
            .split(' ')
            .map(|x| x.into())
            .collect::<Vec<String>>();

        Self::remove_duplicate_option(&mut arguments);
        Self::handle_include_path(&mut arguments, &self.directory);

        // remove the unnessesary options
        let remove_option = pp_config
            .as_ref()
            .map(|x| x.remove.clone())
            .unwrap_or(Vec::new());

        Self::remove_option(&mut arguments, remove_option);

        // replace the string
        let replace_config = pp_config
            .as_ref()
            .map(|x| x.replace.clone())
            .unwrap_or(Vec::new());
        Self::replace_option(&mut arguments, replace_config);

        // insert needed options
        let insert_option = pp_config
            .as_ref()
            .map(|x| x.insert.clone())
            .unwrap_or(Vec::new());
        Self::insert_needed_option(&mut arguments, insert_option);

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

    fn remove_option(arguments: &mut Vec<String>, remove_options: Vec<String>) {
        use regex::Regex;
        let remove_regex = remove_options
            .into_iter()
            .map(|x| Regex::new(&x).unwrap())
            .collect::<Vec<_>>();
        arguments.retain(|x| remove_regex.iter().all(|regex| !regex.is_match(x)));
        // arguments.retain(|x| !remove_options.contains(x));
    }
    fn replace_option(arguments: &mut Vec<String>, remove_options: Vec<String>) {
        for arg in arguments {
            for ro in &remove_options {
                let ro_token = ro.split(',').collect::<Vec<_>>();
                if ro_token.len() == 2 {
                    *arg = arg.replace(ro_token[0], ro_token[1]);
                }
            }
        }
    }

    fn dump_ccj(&self) {
        println!("{}", serde_json::to_string_pretty(self).unwrap());
    }
}

fn main() {
    let matches = Command::new("ccj_postprocess")
        .version("1.3")
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
        .arg(
            Arg::new("append_file")
                .short('a')
                .value_name("append")
                .long("append")
                .help("append a file after input file")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::new("postprocess_config")
                .short('p')
                .value_name("postprocess_config")
                .long("post_conf")
                .help("a json format config to tell ccj_postprocess how to postprocess")
                .takes_value(true)
                .required(false),
        )
        .get_matches();
    let input_file = matches.value_of("input_file").unwrap();
    let append_file = matches.value_of("append_file");
    let postprocess_config = matches.value_of("postprocess_config").map(|file| {
        let pp = Path::new(file);
        let context =
            std::fs::read_to_string(pp).expect(&format!("cannot open the append file: {}", file));
        let pp_config: PostProcessConfig = serde_json::from_str(&context)
            .expect("[Error] json fileparser fail for postprocess config");
        pp_config
    });
    let path = Path::new(input_file);
    let context = std::fs::read_to_string(path).expect(&format!("cannot open the file {:?}", path));
    let mut compile_commands: Vec<CompileCommand> =
        serde_json::from_str(&context).expect("[Error] json file parser fail!");

    if let Some(append_path) = append_file {
        let ap = Path::new(append_path);
        let context =
            std::fs::read_to_string(ap).expect(&format!("cannot open the append file: {:?}", path));
        let append_compile_commands: Vec<CompileCommand> =
            serde_json::from_str(&context).expect("[Error] json file parser fail for append file!");
        for acc in append_compile_commands {
            match compile_commands
                .iter_mut()
                .find(|x| x.directory == acc.directory && x.file == acc.file)
            {
                Some(cc) => {
                    *cc = acc;
                }
                None => {
                    compile_commands.push(acc);
                }
            }
        }
    }

    compile_commands
        .par_iter_mut()
        .for_each(|x| x.postprocess(&postprocess_config));
    // for cc in &mut compile_commands {
    //     println!("i: {}",i);
    //     i+=1;
    //     cc.postprocess();
    // }

    println!("[");
    compile_commands[0].dump_ccj();
    for cc in &compile_commands[1..] {
        println!(",");
        cc.dump_ccj();
    }
    println!("]");
}
