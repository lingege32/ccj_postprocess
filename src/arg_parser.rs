use clap::{Arg, ArgMatches, Command};
pub struct ArgParser {
    matches: ArgMatches,
}

impl ArgParser {
    pub fn parse() -> Self {
        Self {
            matches: Command::new("ccj_postprocess")
            .version("1.7.5")
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
                Arg::new("skip_nonexisted_file")
                    .long("skip_nonexisted_file")
                    .help("Skip the non-existed transunit file.")
                    .required(false)
                    .action(clap::ArgAction::SetTrue),
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
            .get_matches()
        }
    }

    pub fn get_input_file(&self) -> Option<&String> {
        self.matches.get_one::<String>("input_file")
    }

    pub fn get_postprocess_config(&self) -> Option<&String> {
        self.matches.get_one::<String>("postprocess_config")
    }

    pub fn get_append_files(&self) -> Option<&String> {
        self.matches.get_one::<String>("append_file")
    }

    pub fn get_keep_duplicated(&self) -> Option<&String> {
        self.matches.get_one::<String>("keep_duplicated_file")
    }
    pub fn is_dump_transunit_list(&self) -> bool {
        self.matches
            .get_one::<bool>("dump_TransUnit_list")
            .map(|x| *x)
            .unwrap_or(false)
    }

    pub fn find_the_command(&self) -> Option<&String> {
        self.matches.get_one::<String>("FindCommand")
    }

    pub fn skip_nonexisted_file(&self) -> bool {
        self.matches
            .get_one::<bool>("skip_nonexisted_file")
            .map(|x| *x)
            .unwrap_or(false)
    }
}
