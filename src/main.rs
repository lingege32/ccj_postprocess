use rayon::prelude::*;

use ccj_postprocess::arg_parser;
use ccj_postprocess::compile_commands::CompileCommand;
use ccj_postprocess::postprocess_config::PostProcessConfig;
use ccj_postprocess::skim_utility;
use std::path::Path;
fn main() {
    let arg_parser = arg_parser::ArgParser::parse();
    let input_file = arg_parser.get_input_file().unwrap();
    let postprocess_config = arg_parser
        .get_postprocess_config()
        .map(|file| PostProcessConfig::parse_the_config(file));
    let mut compile_commands = CompileCommand::parse(input_file);
    
    if arg_parser.skip_nonexisted_file() {
        compile_commands.retain(|c| Path::new(&format!("{}/{}", c.directory, c.file)).exists());
    }

    if let Some(append_path) = arg_parser.get_append_files() {
        for a_path in append_path.split(',') {
            let mut append_compile_commands = CompileCommand::parse(a_path);
            compile_commands.append(&mut append_compile_commands);
        }
    }

    match arg_parser.get_keep_duplicated().unwrap().as_str() {
        "keep" => {
            // do nothing
        }
        "retain_first" => {
            compile_commands = CompileCommand::deduplicate_with_retain_first(compile_commands);
        }
        "retain_last" => {
            compile_commands.reverse();
            compile_commands = CompileCommand::deduplicate_with_retain_first(compile_commands);
            compile_commands.reverse();
        }
        _ => {
            unreachable!();
        }
    }

    if let Some(ppc) = &postprocess_config {
        CompileCommand::process_config(&mut compile_commands, &ppc);
    }

    compile_commands
        .par_iter_mut()
        .for_each(|x| x.postprocess(&postprocess_config));

    // Handle interactive file selection
    if arg_parser.is_select_file() {
        skim_utility::select_cpp_files(&compile_commands);
        return;
    }

    if arg_parser.is_dump_transunit_list() {
        for cc in compile_commands {
            cc.dump_full_path();
        }
        return;
    }

    if let Some(file) = arg_parser.find_the_command() {
        for cc in compile_commands {
            if cc.file == *file || format!("{}/{}", cc.directory, cc.file) == *file {
                println!("{}, {}", cc.directory, cc.command);
            }
        }
        return;
    }

    CompileCommand::dump_ccj(&compile_commands);
}
