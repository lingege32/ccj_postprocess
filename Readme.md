customize the compile_command.json.
------
ccj_postprocess 1.7.1
Toby Lin
compile_commands.json postprocess for zebu

USAGE:
    ccj_postprocess [OPTIONS] --input <input>

OPTIONS:
    -a, --append <append>
            append a file after input file

    -h, --help
            Print help information

    -i, --input <input>
            a compile_commands.json generated from vgbuild

        --keep-duplicated <keep_duplicated_file>
            keep duplicated file in the command line. Default: false [default: false]

    -p, --post_conf <postprocess_config>
            a json format config to tell ccj_postprocess how to postprocess

    -V, --version
            Print version information

For more information try --help
