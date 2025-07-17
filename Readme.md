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
| `--append <append>` | `-a` | Append additional compile_commands.json files (comma-separated) |
| `--post_conf <config>` | `-p` | JSON configuration file specifying postprocessing rules |
| `--keep-duplicated <mode>` | | How to handle duplicate files: keep all, retain first occurrence, or retain last occurrence [default: retain_first] |
| `--skip_nonexisted_file` | | Skip source files that don't exist on the filesystem |
| `--dump_list` | | List all source files (translation units) found in compile commands |
| `--find_command <file>` | | Find and display the compile command for specified files (comma-separated) |
| `--select_file` | `-s` | **NEW**: Launch interactive fuzzy finder to select C++ source files from compile commands |
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
