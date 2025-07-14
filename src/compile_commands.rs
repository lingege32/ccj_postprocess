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
    /// Post-process a single compile command.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The compile command to be post-processed.
    /// * `pp_config` - The post-processing configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::compile_commands::CompileCommand;
    /// use crate::postprocess_config::PostProcessConfig;
    ///
    /// let mut cc = CompileCommand {
    ///     command: "g++ -I. -DNDEBUG -o test test.cpp".to_string(),
    ///     arguments: vec![],
    ///     directory: "/path/to/project".to_string(),
    ///     file: "test.cpp".to_string(),
    ///     output: "test".to_string(),
    /// };
    /// let pp_config = Some(PostProcessConfig::default());
    /// cc.postprocess(&pp_config);
    /// ```
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

    /// Parses a `compile_commands.json` file and returns a vector of `CompileCommand` structs.
    ///
    /// # Arguments
    ///
    /// * `file` - The path to the `compile_commands.json` file.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crate::compile_commands::CompileCommand;
    ///
    /// let compile_commands = CompileCommand::parse("compile_commands.json");
    /// ```
    pub fn parse(file: &str) -> Vec<CompileCommand> {
        let path = Path::new(file);
        let context =
            std::fs::read_to_string(path).expect(&format!("cannot open the file {:?}", path));
        serde_json::from_str::<Vec<CompileCommand>>(&context)
            .expect(&format!("[Error] json file {:?} parse fail!", path))
    }

    /// Dumps a slice of `CompileCommand` structs to the console in a JSON format.
    ///
    /// # Arguments
    ///
    /// * `compile_commands` - The slice of `CompileCommand` structs to be dumped.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::compile_commands::CompileCommand;
    ///
    /// let compile_commands = vec![];
    /// CompileCommand::dump_ccj(&compile_commands);
    /// ```
    pub fn dump_ccj(compile_commands: &[CompileCommand]) {
        println!("[");
        compile_commands[0].dump_one_ccj();
        for cc in &compile_commands[1..] {
            println!(",");
            cc.dump_one_ccj();
        }
        println!("]");
    }
    /// Removes duplicate compile commands from a vector, keeping the first occurrence.
    ///
    /// # Arguments
    ///
    /// * `compile_commands` - The vector of `CompileCommand` structs to be deduplicated.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::compile_commands::CompileCommand;
    ///
    /// let compile_commands = vec![];
    /// let deduped_commands = CompileCommand::deduplicate_with_retain_first(compile_commands);
    /// ```
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
    /// Processes the compile commands based on a `PostProcessConfig`, likely filtering out ignored files.
    ///
    /// # Arguments
    ///
    /// * `compile_commands` - The vector of `CompileCommand` structs to be processed.
    /// * `ppc` - The post-processing configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::compile_commands::CompileCommand;
    /// use crate::postprocess_config::PostProcessConfig;
    ///
    /// let mut compile_commands = vec![];
    /// let pp_config = PostProcessConfig::default();
    /// CompileCommand::process_config(&mut compile_commands, &pp_config);
    /// ```
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
    /// Prints the full path of the file associated with the compile command.
    ///
    /// # Arguments
    ///
    /// * `&self` - The compile command.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::compile_commands::CompileCommand;
    ///
    /// let cc = CompileCommand {
    ///     command: "".to_string(),
    ///     arguments: vec![],
    ///     directory: "/path/to/project".to_string(),
    ///     file: "test.cpp".to_string(),
    ///     output: "".to_string(),
    /// };
    /// cc.dump_full_path();
    /// ```
    pub fn dump_full_path(&self) {
        println!("{}/{}", self.directory, self.file);
    }

    /// Checks if a command-line argument is a `-D` option with an equals sign.
    ///
    /// # Arguments
    ///
    /// * `arg` - The command-line argument.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::compile_commands::CompileCommand;
    ///
    /// assert!(CompileCommand::is_arg_with_d_and_equal("-DNDEBUG=1"));
    /// ```
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
    /// Checks if a `-D` option needs single quote handling.
    ///
    /// # Arguments
    ///
    /// * `arg` - The command-line argument.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::compile_commands::CompileCommand;
    ///
    /// assert!(CompileCommand::is_need_to_handle_sing_quota("-DEXTERN='abc'"));
    /// ```
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
    /// Removes single quotes from `-D` options.
    ///
    /// # Arguments
    ///
    /// * `arg` - The vector of command-line arguments.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::compile_commands::CompileCommand;
    ///
    /// let mut args = vec!["-DEXTERN='abc'".to_string()];
    /// CompileCommand::handle_the_single_quote(&mut args);
    /// assert_eq!(args[0], "-DEXTERN=abc");
    /// ```
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

    /// Joins a slice of arguments into a single command string.
    ///
    /// It handles arguments with spaces in `-D` options by wrapping the value in single quotes.
    ///
    /// # Arguments
    ///
    /// * `args` - A slice of command-line arguments.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::compile_commands::CompileCommand;
    ///
    /// let args = vec!["g++".to_string(), "-DVAR=a b".to_string(), "main.cpp".to_string()];
    /// let command = CompileCommand::join_the_arguments_as_commands(&args);
    /// assert_eq!(command, "g++ -DVAR='a b' main.cpp");
    ///
    /// let args_no_space = vec!["g++".to_string(), "-DVAR=ab".to_string(), "main.cpp".to_string()];
    /// let command_no_space = CompileCommand::join_the_arguments_as_commands(&args_no_space);
    /// assert_eq!(command_no_space, "g++ -DVAR=ab main.cpp");
    /// ```
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

    /// Initializes the `arguments` field from the `command` field if `arguments` is empty.
    ///
    /// It splits the `command` string by spaces.
    ///
    /// # Arguments
    ///
    /// * `&mut self` - The `CompileCommand` to initialize.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::compile_commands::CompileCommand;
    ///
    /// let mut cc = CompileCommand {
    ///     command: "g++ -o main main.cpp".to_string(),
    ///     arguments: vec![],
    ///     directory: "/".to_string(),
    ///     file: "main.cpp".to_string(),
    ///     output: "main".to_string(),
    /// };
    /// cc.init_arguments();
    /// assert_eq!(cc.arguments, vec!["g++", "-o", "main", "main.cpp"]);
    ///
    /// // Does not re-initialize if arguments already exist
    /// let mut cc_with_args = CompileCommand {
    ///     command: "g++ -o main main.cpp".to_string(),
    ///     arguments: vec!["g++".to_string()],
    ///     directory: "/".to_string(),
    ///     file: "main.cpp".to_string(),
    ///     output: "main".to_string(),
    /// };
    /// cc_with_args.init_arguments();
    /// assert_eq!(cc_with_args.arguments, vec!["g++"]);
    /// ```
    fn init_arguments(&mut self) {
        if self.arguments.is_empty() {
            self.arguments = self
                .command
                .split(' ')
                .map(|x| x.into())
                .collect::<Vec<String>>();
        }
    }

    /// Removes duplicate options from a vector of arguments, keeping the first occurrence.
    ///
    /// # Arguments
    ///
    /// * `arguments` - The vector of command-line arguments.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::compile_commands::CompileCommand;
    ///
    /// let mut args = vec!["-I.".to_string(), "-g".to_string(), "-I.".to_string()];
    /// CompileCommand::remove_duplicate_option(&mut args);
    /// assert_eq!(args, vec!["-I.", "-g"]);
    /// ```
    fn remove_duplicate_option(arguments: &mut Vec<String>) {
        let mut hs = std::collections::HashSet::new();
        // remove the duplicate arguments
        arguments.retain(|x| hs.insert(x.clone()));
    }

    /// Resolves relative paths in `-I` options to be absolute from the filesystem root.
    ///
    /// It takes an include path like `-I.` and a `base_directory` and converts it
    /// to an absolute path like `-I/path/to/project`.
    ///
    /// # Arguments
    ///
    /// * `arguments` - The vector of command-line arguments.
    /// * `base_directory` - The base directory to resolve relative paths against.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::compile_commands::CompileCommand;
    ///
    /// let mut args = vec!["-I../include".to_string()];
    /// CompileCommand::handle_include_path(&mut args, "/home/user/project/src");
    /// assert_eq!(args[0], "-I/home/user/project/include");
    ///
    /// let mut args2 = vec!["-I/usr/local/include".to_string()];
    /// CompileCommand::handle_include_path(&mut args2, "/home/user/project/src");
    /// assert_eq!(args2[0], "-I/usr/local/include");
    /// ```
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

    /// Inserts a vector of options into the arguments list after the first element (the compiler).
    ///
    /// # Arguments
    ///
    /// * `arguments` - The vector of command-line arguments.
    /// * `insert_options` - The vector of options to insert.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::compile_commands::CompileCommand;
    ///
    /// let mut args = vec!["g++".to_string(), "-o".to_string(), "main".to_string(), "main.cpp".to_string()];
    /// let insert_options = vec!["-DDEBUG".to_string(), "-Wall".to_string()];
    /// CompileCommand::insert_needed_option(&mut args, insert_options);
    /// assert_eq!(args, vec!["g++", "-DDEBUG", "-Wall", "-o", "main", "main.cpp"]);
    /// ```
    fn insert_needed_option(arguments: &mut Vec<String>, mut insert_options: Vec<String>) {
        // insert the specified option after first g++ command
        // original: g++ -o main main.cpp
        // after:    g++ -D__GNU__=10 -o main main.cpp
        let len = insert_options.len();
        arguments.append(&mut insert_options);
        arguments[1..].rotate_right(len);
    }

    /// Removes options from the arguments list that match a given list of regular expressions.
    ///
    /// # Arguments
    ///
    /// * `arguments` - The vector of command-line arguments.
    /// * `remove_options` - A vector of regex patterns to match against and remove.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::compile_commands::CompileCommand;
    ///
    /// let mut args = vec!["g++".to_string(), "-g".to_string(), "-O2".to_string(), "-Wall".to_string()];
    /// let remove_options = vec!["-g".to_string(), "-O.".to_string()];
    /// CompileCommand::remove_option(&mut args, remove_options);
    /// assert_eq!(args, vec!["g++", "-Wall"]);
    /// ```
    fn remove_option(arguments: &mut Vec<String>, remove_options: Vec<String>) {
        use regex::Regex;
        let remove_regex = remove_options
            .into_iter()
            .map(|x| Regex::new(&x).unwrap())
            .collect::<Vec<_>>();
        arguments.retain(|x| remove_regex.iter().all(|regex| !regex.is_match(x)));
        // arguments.retain(|x| !remove_options.contains(x));
    }

    /// Replaces substrings in arguments based on a configuration.
    ///
    /// The `replace_options` parameter contains comma-separated strings, e.g., "from,to".
    /// For each argument, this function replaces all occurrences of "from" with "to".
    ///
    /// # Arguments
    ///
    /// * `arguments` - The vector of command-line arguments.
    /// * `replace_options` - A vector of comma-separated "from,to" strings that define the replacements.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::compile_commands::CompileCommand;
    ///
    /// let mut args = vec!["-O2".to_string(), "--param=val1".to_string()];
    /// let replace_config = vec!["-O2,-O3".to_string(), "val1,val2".to_string()];
    /// CompileCommand::replace_option(&mut args, replace_config);
    /// assert_eq!(args, vec!["-O3", "--param=val2"]);
    /// ```
    fn replace_option(arguments: &mut Vec<String>, replace_options: Vec<String>) {
        for arg in arguments {
            for ro in &replace_options {
                let ro_token = ro.split(',').collect::<Vec<_>>();
                if ro_token.len() == 2 {
                    *arg = arg.replace(ro_token[0], ro_token[1]);
                }
            }
        }
    }

    /// Dumps a single `CompileCommand` to the console in a pretty JSON format.
    ///
    /// # Arguments
    ///
    /// * `&self` - The `CompileCommand` to dump.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::compile_commands::CompileCommand;
    ///
    /// let cc = CompileCommand {
    ///     command: "g++".to_string(),
    ///     arguments: vec!["g++".to_string()],
    ///     directory: "/".to_string(),
    ///     file: "a.cpp".to_string(),
    ///     output: "a.o".to_string(),
    /// };
    /// // This will print the CompileCommand as a JSON object to stdout.
    /// cc.dump_one_ccj();
    /// ```
    fn dump_one_ccj(&self) {
        println!("{}", serde_json::to_string_pretty(self).unwrap());
    }
}
