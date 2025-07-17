use skim::prelude::*;
use std::io::Cursor;
use crate::compile_commands::CompileCommand;

/// Launches an interactive file selector for C++ files from compile commands.
/// Filters for common C++ file extensions and presents them using skim.
/// Outputs the full paths of selected files.
pub fn select_cpp_files(compile_commands: &[CompileCommand]) {
    // Filter for C++ files based on common extensions
    let cpp_files: Vec<String> = compile_commands
        .iter()
        .filter(|cc| is_cpp_file(&cc.file))
        .map(|cc| format!("{}/{}", cc.directory, cc.file))
        .collect();

    if cpp_files.is_empty() {
        eprintln!("No C++ files found in compile commands.");
        return;
    }

    // Create input for skim
    let input = cpp_files.join("\n");
    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));

    // Configure skim options
    let options = SkimOptionsBuilder::default()
        .height("50%".to_string())
        .multi(true)
        .prompt("Select C++ files: ".to_string())
        .build()
        .unwrap();

    // Run skim
    let selected_items = Skim::run_with(&options, Some(items));
    
    match selected_items {
        Some(out) => {
            if out.is_abort {
                eprintln!("Selection cancelled.");
                return;
            }
            
            // Output selected file paths
            for item in out.selected_items {
                println!("{}", item.output());
            }
        }
        None => {
            eprintln!("No files selected.");
        }
    }
}

/// Checks if a file has a C++ extension.
fn is_cpp_file(filename: &str) -> bool {
    let cpp_extensions = [".cpp", ".cxx", ".cc", ".c++", ".C"];
    cpp_extensions.iter().any(|ext| filename.ends_with(ext))
}
