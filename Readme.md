customize the compile_command.json.
------
ccj_postprocess 1.7.5
Toby Lin
compile_commands.json postprocess for zebu

USAGE:
    ccj_postprocess [OPTIONS] --input <input>

OPTIONS:
    -i, --input <input>
            a compile_commands.json generated from vgbuild
    -a, --append <append>
            Append files after input file; use ',' as delimiter
    -p, --post_conf <postprocess_config>
            a json format config to tell ccj_postprocess how to postprocess
        --keep-duplicated <keep_duplicated_file>
            keep duplicated file in the command line. [default: retain_first] [possible values: keep, retain_first, retain_last]
        --skip_nonexisted_file
            Skip the non-existed transunit file.
        --dump_list
            Dump the all transunit file
        --find_command <FindCommand>
            Dump the directory and the command for the specified file. Seperated by the comma.
    -h, --help
            Print help
    -V, --version
            Print version

For more information try --help
