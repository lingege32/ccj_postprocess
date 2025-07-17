use clap::{Arg, ArgMatches, Command};

/// A struct for parsing command-line arguments.
pub struct ArgParser {
    matches: ArgMatches,
}

/// A builder for creating command-line arguments.
pub struct ArgBuilder;

impl ArgBuilder {
    /// Creates the argument for the input file.
    pub fn input_file_arg() -> Arg {
        Arg::new("input_file")
            .short('i')
            .value_name("input")
            .long("input")
            .help("Input compile_commands.json file generated from vgbuild")
            .action(clap::ArgAction::Set)
            .required(true)
    }

    /// Creates the argument for the append file.
    pub fn append_file_arg() -> Arg {
        Arg::new("append_file")
            .short('a')
            .value_name("append")
            .long("append")
            .help("Append additional compile_commands.json files (comma-separated)")
            .action(clap::ArgAction::Set)
            .required(false)
    }

    /// Creates the argument for the postprocess config.
    pub fn postprocess_config_arg() -> Arg {
        Arg::new("postprocess_config")
            .short('p')
            .value_name("postprocess_config")
            .long("post_conf")
            .help("JSON configuration file specifying postprocessing rules")
            .action(clap::ArgAction::Set)
            .required(false)
    }

    /// Creates the argument for the keep duplicated file option.
    pub fn keep_duplicated_file_arg() -> Arg {
        Arg::new("keep_duplicated_file")
            .long("keep-duplicated")
            .help("How to handle duplicate files: keep all, retain first occurrence, or retain last occurrence")
            .action(clap::ArgAction::Set)
            .value_parser(["keep", "retain_first", "retain_last"])
            .required(false)
            .default_value("retain_first")
    }

    /// Creates the argument for skipping non-existed files.
    pub fn skip_nonexisted_file_arg() -> Arg {
        Arg::new("skip_nonexisted_file")
            .long("skip_nonexisted_file")
            .help("Skip source files that don't exist on the filesystem")
            .required(false)
            .action(clap::ArgAction::SetTrue)
    }

    /// Creates the argument for dumping the transunit list.
    pub fn dump_transunit_list_arg() -> Arg {
        Arg::new("dump_TransUnit_list")
            .long("dump_list")
            .help("List all source files (translation units) found in compile commands")
            .required(false)
            .action(clap::ArgAction::SetTrue)
    }

    /// Creates the argument for finding a command.
    pub fn find_command_arg() -> Arg {
        Arg::new("FindCommand")
            .long("find_command")
            .help("Find and display the compile command for specified files (comma-separated)")
            .required(false)
            .action(clap::ArgAction::Set)
    }

    /// Creates the argument for interactive file selection.
    pub fn select_file_arg() -> Arg {
        Arg::new("select_file")
            .long("select_file")
            .short('s')
            .help("Launch interactive fuzzy finder to select C++ source files from compile commands")
            .action(clap::ArgAction::SetTrue)
            .required(false)
    }
}

impl ArgParser {
    /// Parses the command-line arguments and returns a new `ArgParser` instance.
    ///
    /// # Returns
    ///
    /// - `Self` - An `ArgParser` instance containing the parsed command-line arguments.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::arg_parser::ArgParser;
    /// let arg_parser = ArgParser::parse();
    /// ```
    pub fn parse() -> Self {
        let matches = Self::build_command().get_matches();
        Self { matches }
    }

    /// Builds the command-line argument parser.
    fn build_command() -> Command {
        Command::new("ccj_postprocess")
            .version("1.8.0")
            .author("Toby Lin")
            .about("compile_commands.json postprocess for zebu")
            .arg(ArgBuilder::input_file_arg())
            .arg(ArgBuilder::append_file_arg())
            .arg(ArgBuilder::postprocess_config_arg())
            .arg(ArgBuilder::keep_duplicated_file_arg())
            .arg(ArgBuilder::skip_nonexisted_file_arg())
            .arg(ArgBuilder::dump_transunit_list_arg())
            .arg(ArgBuilder::find_command_arg())
            .arg(ArgBuilder::select_file_arg())
    }

    /// Returns the input file path.
    ///
    /// # Returns
    ///
    /// - `Option<&String>` - The input file path if it exists, otherwise `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::arg_parser::ArgParser;
    /// let arg_parser = ArgParser::parse();
    /// let input_file = arg_parser.get_input_file();
    /// ```
    pub fn get_input_file(&self) -> Option<&String> {
        self.matches.get_one::<String>("input_file")
    }

    /// Returns the postprocess config file path.
    ///
    /// # Returns
    ///
    /// - `Option<&String>` - The postprocess config file path if it exists, otherwise `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::arg_parser::ArgParser;
    /// let arg_parser = ArgParser::parse();
    /// let config = arg_parser.get_postprocess_config();
    /// ```
    pub fn get_postprocess_config(&self) -> Option<&String> {
        self.matches.get_one::<String>("postprocess_config")
    }

    /// Returns the append file paths.
    ///
    /// # Returns
    ///
    /// - `Option<&String>` - The append file paths if they exist, otherwise `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::arg_parser::ArgParser;
    /// let arg_parser = ArgParser::parse();
    /// let append_files = arg_parser.get_append_files();
    /// ```
    pub fn get_append_files(&self) -> Option<&String> {
        self.matches.get_one::<String>("append_file")
    }

    /// Returns the keep duplicated option.
    ///
    /// # Returns
    ///
    /// - `Option<&String>` - The keep duplicated option if it exists, otherwise `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::arg_parser::ArgParser;
    /// let arg_parser = ArgParser::parse();
    /// let keep_duplicated = arg_parser.get_keep_duplicated();
    /// ```
    pub fn get_keep_duplicated(&self) -> Option<&String> {
        self.matches.get_one::<String>("keep_duplicated_file")
    }

    /// Returns whether to dump the transunit list.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the transunit list should be dumped, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::arg_parser::ArgParser;
    /// let arg_parser = ArgParser::parse();
    /// let is_dump = arg_parser.is_dump_transunit_list();
    /// ```
    pub fn is_dump_transunit_list(&self) -> bool {
        self.matches
            .get_one::<bool>("dump_TransUnit_list")
            .map(|x| *x)
            .unwrap_or(false)
    }

    /// Returns the file to find the command for.
    ///
    /// # Returns
    ///
    /// - `Option<&String>` - The file to find the command for if it exists, otherwise `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::arg_parser::ArgParser;
    /// let arg_parser = ArgParser::parse();
    /// let find_command = arg_parser.find_the_command();
    /// ```
    pub fn find_the_command(&self) -> Option<&String> {
        self.matches.get_one::<String>("FindCommand")
    }

    /// Returns whether to skip non-existed files.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if non-existed files should be skipped, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::arg_parser::ArgParser;
    /// let arg_parser = ArgParser::parse();
    /// let skip = arg_parser.skip_nonexisted_file();
    /// ```
    pub fn skip_nonexisted_file(&self) -> bool {
        self.matches
            .get_one::<bool>("skip_nonexisted_file")
            .map(|x| *x)
            .unwrap_or(false)
    }

    /// Returns whether to use interactive file selection.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if interactive file selection should be used, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::arg_parser::ArgParser;
    /// let arg_parser = ArgParser::parse();
    /// let select_file = arg_parser.is_select_file();
    /// ```
    pub fn is_select_file(&self) -> bool {
        self.matches
            .get_one::<bool>("select_file")
            .map(|x| *x)
            .unwrap_or(false)
    }
}
