use crate::postprocess_config::PostProcessConfig;
use relative_path::RelativePath;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct CompileCommand {
    #[serde(default)]
    pub command: String,
    #[serde(default)]
    pub arguments: Vec<String>,
    pub directory: String,
    pub file: String,
    #[serde(default)]
    pub output: String,
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

        Self::handle_the_single_quote(arguments);
        // join the arguments to command
        self.command = Self::join_the_arguments_as_commands(arguments);
    }

    pub fn parse(file: &str) -> Vec<CompileCommand> {
        let path = Path::new(file);
        let context =
            std::fs::read_to_string(path).expect(&format!("cannot open the file {:?}", path));
        serde_json::from_str::<Vec<CompileCommand>>(&context)
            .expect(&format!("[Error] json file {:?} parse fail!", path))
    }

    pub fn dump_ccj(compile_commands: &[CompileCommand]) {
        println!("[");
        compile_commands[0].dump_one_ccj();
        for cc in &compile_commands[1..] {
            println!(",");
            cc.dump_one_ccj();
        }
        println!("]");
    }
    pub fn deduplicate_with_retain_first(
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
    pub fn process_config(compile_commands: &mut Vec<CompileCommand>, ppc: &PostProcessConfig) {
        let ignore_files: Vec<String> = ppc.ignore_files.clone();
        if !ignore_files.is_empty() {
            use regex::Regex;
            let remove_regex = ignore_files
                .into_iter()
                .map(|x| Regex::new(&x).unwrap())
                .collect::<Vec<_>>();
            compile_commands.retain(|x: &CompileCommand| {
                let path = x.directory.clone() + "/" + &x.file;
                remove_regex.iter().all(|regex| !regex.is_match(&path))
            });
        }
    }
    pub fn dump_full_path(&self) {
        println!("{}/{}", self.directory, self.file);
    }

    fn is_arg_with_d_and_equal(arg: &str) -> bool {
        if arg.len() < 2 || &arg[0..2] != "-D" {
            return false;
        }

        let v = arg.split('=').collect::<Vec<_>>();
        if v.len() != 2 || v[1].len() < 2 {
            return false;
        }
        true
    }
    fn is_need_to_handle_sing_quota(arg: &str) -> bool {
        if arg.len() < 2 || &arg[0..2] != "-D" {
            return false;
        }

        let v = arg.split('=').collect::<Vec<_>>();
        if v.len() != 2 || v[1].len() < 2 {
            return false;
        }
        if *v[1].as_bytes().first().unwrap() != '\'' as u8 ||
            *v[1].as_bytes().last().unwrap() != '\'' as u8
        {
            return false;
        }
        true
    }
    fn handle_the_single_quote(arg: &mut [String]) {
        // remove the single quote
        // Example: -DEXTERN='abc' to -DEXTERN=abc.
        for a in arg {
            if !Self::is_need_to_handle_sing_quota(a) {
                continue;
            }

            let v = a.split('=').collect::<Vec<_>>();
            let l = v[1].len();
            let s = format!("{}={}", v[0], &v[1][1..l - 1]);
            *a = s;
        }
    }

    fn join_the_arguments_as_commands(args: &[String]) -> String {
        args.iter()
            .map(|arg| {
                if Self::is_arg_with_d_and_equal(arg) && arg.contains(" ") {
                    let v = arg.split('=').collect::<Vec<_>>();
                    let a = format!("{}='{}'", v[0], v[1]);
                    a
                } else {
                    arg.clone()
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
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

    fn dump_one_ccj(&self) {
        println!("{}", serde_json::to_string_pretty(self).unwrap());
    }
}
