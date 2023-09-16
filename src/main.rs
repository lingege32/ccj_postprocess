use clap::{Arg, Command};
use rayon::prelude::*;
use relative_path::RelativePath;
use std::path::Path;
#[macro_use]
extern crate serde_derive;
#[derive(Serialize, Deserialize, Debug)]
struct CompileCommand {
    #[serde(default)]
    command: String,
    #[serde(default)]
    arguments: Vec<String>,
    directory: String,
    file: String,
    #[serde(default)]
    output: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct PostProcessConfig {
    #[serde(default)]
    remove: Vec<String>,
    #[serde(default)]
    insert: Vec<String>,
    #[serde(default)]
    replace: Vec<String>,
    #[serde(default)]
    ignore_files: Vec<String>,
}

impl CompileCommand {
    pub fn postprocess(&mut self, pp_config: &Option<PostProcessConfig>) {
        self.init_arguments();
        let arguments = &mut self.arguments;

        Self::remove_duplicate_option(arguments);
        Self::handle_include_path(arguments, &self.directory);

        // remove the unnessesary options
        let remove_option = pp_config
            .as_ref()
            .map(|x| x.remove.clone())
            .unwrap_or(Vec::new());

        Self::remove_option(arguments, remove_option);

        // replace the string
        let replace_config = pp_config
            .as_ref()
            .map(|x| x.replace.clone())
            .unwrap_or(Vec::new());
        Self::replace_option(arguments, replace_config);

        // insert needed options
        let insert_option = pp_config
            .as_ref()
            .map(|x| x.insert.clone())
            .unwrap_or(Vec::new());
        Self::insert_needed_option(arguments, insert_option);

        Self::remove_duplicate_option(arguments);

        // join the arguments to command
        self.command = arguments.join(" ");
    }
    fn init_arguments(&mut self) {
        if self.arguments.is_empty() {
            self.arguments = self
                .command
                .split(' ')
                .map(|x| x.into())
                .collect::<Vec<String>>();
        }
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

    fn dump_full_path(&self) {
        println!("{}/{}", self.directory, self.file);
    }
}

fn main() {
    let matches = Command::new("ccj_postprocess")
        .version("1.7.1")
        .author("Toby Lin")
        .about("compile_commands.json postprocess for zebu")
        .arg(
            Arg::new("input_file")
                .short('i')
                .value_name("input")
                .long("input")
                .help("a compile_commands.json generated from vgbuild")
                .action(clap::ArgAction::Set)
                .required(true),
        )
        .arg(
            Arg::new("append_file")
                .short('a')
                .value_name("append")
                .long("append")
                .help("Append files after input file; use ',' as delimiter")
                .action(clap::ArgAction::Set)
                .required(false),
        )
        .arg(
            Arg::new("postprocess_config")
                .short('p')
                .value_name("postprocess_config")
                .long("post_conf")
                .help("a json format config to tell ccj_postprocess how to postprocess")
                .action(clap::ArgAction::Set)
                .required(false),
        )
        .arg(
            Arg::new("keep_duplicated_file")
                .long("keep-duplicated")
                .help("keep duplicated file in the command line.")
                .action(clap::ArgAction::Set)
                .value_parser(["keep", "retain_first", "retain_last"])
                .required(false)
                .default_value("retain_first"),
        )
        .arg(
            Arg::new("dump_TransUnit_list")
                .long("dump_list")
                .help("Dump the all transunit file")
                .required(false)
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("FindCommand")
                .long("find_command")
                .help("Dump the directory and the command for the specified file. Seperated by the comma.")
                .required(false)
                .action(clap::ArgAction::Set),
        )
        .get_matches();
    let input_file = matches.get_one::<String>("input_file").unwrap();
    let postprocess_config = matches.get_one::<String>("postprocess_config").map(|file| {
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

    if let Some(append_path) = matches.get_one::<String>("append_file") {
        for a_path in append_path.split(',') {
            let ap = Path::new(a_path);
            let context = std::fs::read_to_string(ap)
                .expect(&format!("cannot open the append file: {:?}", path));
            let mut append_compile_commands: Vec<CompileCommand> = serde_json::from_str(&context)
                .expect("[Error] json file parser fail for append file!");
            compile_commands.append(&mut append_compile_commands);
        }
    }

    fn deduplicate_with_retain_first(
        mut compile_commands: Vec<CompileCommand>,
    ) -> Vec<CompileCommand> {
        let mut tmp_compile_commands = Vec::with_capacity(compile_commands.len());
        std::mem::swap(&mut tmp_compile_commands, &mut compile_commands);
        let mut hs = std::collections::HashSet::new();
        for compile_command in tmp_compile_commands {
            let key = compile_command.directory.clone() + &compile_command.file;
            if hs.insert(key) {
                compile_commands.push(compile_command);
            }
        }
        compile_commands
    }
    match matches
        .get_one::<String>("keep_duplicated_file")
        .unwrap()
        .as_str()
    {
        "keep" => {
            // do nothing
        }
        "retain_first" => {
            compile_commands = deduplicate_with_retain_first(compile_commands);
        }
        "retain_last" => {
            compile_commands.reverse();
            compile_commands = deduplicate_with_retain_first(compile_commands);
            compile_commands.reverse();
        }
        _ => {
            unreachable!();
        }
    }

    if let Some(ppc) = &postprocess_config {
        let ignore_files = ppc.ignore_files.clone();
        if !ignore_files.is_empty() {
            use regex::Regex;
            let remove_regex = ignore_files
                .into_iter()
                .map(|x| Regex::new(&x).unwrap())
                .collect::<Vec<_>>();
            compile_commands.retain(|x| {
                let path = x.directory.clone() + "/" + &x.file;
                remove_regex.iter().all(|regex| !regex.is_match(&path))
            });
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
    let dump_transunit_list = matches.get_one::<bool>("dump_TransUnit_list").map(|x| *x).unwrap_or(false);
    if dump_transunit_list {
        for cc in compile_commands {
            cc.dump_full_path();
        }
        return;
    }

    if let Some(file) = matches.get_one::<String>("FindCommand") {
        for cc in compile_commands {
            if cc.file == *file || format!("{}/{}", cc.directory, cc.file) == *file {
                println!("{}, {}", cc.directory, cc.command);
            }
        }
        return;
    }

    println!("[");
    compile_commands[0].dump_ccj();
    for cc in &compile_commands[1..] {
        println!(",");
        cc.dump_ccj();
    }
    println!("]");
}
