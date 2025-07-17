# ccj_postprocess

Customize and process compile_commands.json files for zebu development.

**Version**: 1.8.0  
**Author**: Terry Lin

## Description

A powerful tool for postprocessing `compile_commands.json` files generated from vgbuild, with interactive file selection capabilities.

## Usage

```
ccj_postprocess [OPTIONS] --input <input>
```

## Options

| Option | Short | Description |
|--------|-------|-------------|
| `--input <input>` | `-i` | Input compile_commands.json file generated from vgbuild |
| `--append <append>` | `-a` | Append additional compile commands files (comma-separated) |
| `--post_conf <config>` | `-p` | JSON configuration file for postprocessing rules |
| `--keep-duplicated <mode>` | | Handle duplicate files: `keep`, `retain_first`, `retain_last` [default: retain_first] |
| `--skip_nonexisted_file` | | Skip files that don't exist on the filesystem |
| `--dump_list` | | List all translation units (source files) |
| `--find_command <file>` | | Find and display the compile command for a specific file |
| `--select_file` | `-s` | **NEW**: Interactive C++ file selector using fuzzy finder |
| `--help` | `-h` | Show help information |
| `--version` | `-V` | Display version information |

## New Features in v1.8.0

### Interactive File Selection (`--select_file`)

Launch an interactive fuzzy finder to select C++ source files from your compile commands:

```bash
ccj_postprocess -i compile_commands.json --select_file
```

Features:
- 🔍 **Fuzzy search**: Type to filter files by name
- 📁 **Full path display**: Shows complete directory + filename
- ✅ **Multi-selection**: Select multiple files using Tab/Shift-Tab
- 🎯 **C++ focused**: Automatically filters for .cpp, .cxx, .cc, .c++, .C files
- ⚡ **Fast navigation**: Arrow keys and search-as-you-type

## Examples

```bash
# Basic postprocessing
ccj_postprocess -i compile_commands.json

# Interactive file selection
ccj_postprocess -i compile_commands.json -s

# Append multiple files and apply config
ccj_postprocess -i main.json -a extra1.json,extra2.json -p config.json

# List all source files
ccj_postprocess -i compile_commands.json --dump_list

# Find specific file's compile command
ccj_postprocess -i compile_commands.json --find_command myfile.cpp
```

For more information, try `ccj_postprocess --help`
