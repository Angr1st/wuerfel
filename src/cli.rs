use getargs::{Arg, Options};
use std::env::args;

pub enum CliOptions {
    Exit,
    Text,
    TUI,
    GUI,
    ThreeDimensional,
}

pub fn get_cli_options() -> CliOptions {
    let mut args = args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        args.push(String::from("--help")); // help the user out :)
    }

    let mut opts = Options::new(args.iter().map(String::as_str));

    while let Some(arg) = opts.next_arg().expect("argument parsing error") {
        match arg {
            Arg::Short('h') | Arg::Long("help") => {
                eprintln!(
                    r"Usage: wuerfel.exe [OPTIONS/ARGS]...
  -h, --help       display this help and exit
  -c, --text       use raw text output
  -t, --tui        use tui output
  -g, --gui        use gui output
  -3, --3d         use 3d output"
                );

                return CliOptions::Exit;
            }

            Arg::Short('c') | Arg::Long("text") => {
                return CliOptions::Text;
            }

            Arg::Short('t') | Arg::Long("tui") => {
                return CliOptions::TUI;
            }

            Arg::Short('g') | Arg::Long("gui") => {
                return CliOptions::GUI;
            }

            Arg::Short('3') | Arg::Long("3d") => {
                return CliOptions::ThreeDimensional;
            }
            _ => {
                return CliOptions::Exit;
            }
        }
    }
    return CliOptions::Exit;
}
