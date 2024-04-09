use std::process::exit;

use getopts::Options;
use once_cell::sync::OnceCell;

pub struct CliOption {
    pub files: Vec<String>,
    pub silent: bool,
    pub skip_top_level: bool,
    pub grouped_by: Option<String>,
}
pub static INSTANCE: OnceCell<CliOption> = OnceCell::new();

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] FILES", program);
    print!("{}", opts.usage(&brief));
}

impl CliOption {
    pub fn from_cli(raw_args: &[String]) -> Self {
        let mut silent = false;
        let mut skip_top_level = false;
        let mut grouped_by = Option::None;

        let mut opts = Options::new();
        let program = &raw_args[0];
        let args = &raw_args[1..];

        opts.optflag("h", "help", "Print help menu");
        opts.optflag("s", "silent", "Don't display logs other than errors.");
        opts.optflag(
            "",
            "skip-top-level",
            "Assuming the top level is composed solely of a single key, skip it.",
        );
        opts.optopt(
            "g",
            "grouped-by",
            "Group locale file by a regular expression",
            "`-g \"^(.*/)([^/]+)$\"` by default",
        );

        let matches = match opts.parse(args) {
            Ok(m) => m,
            Err(f) => panic!("{}", f.to_string()),
        };

        if matches.opt_present("help") {
            print_usage(program, opts);
            exit(0);
        }
        if matches.opt_present("silent") {
            silent = true;
        }
        if matches.opt_present("skip-top-level") {
            skip_top_level = true;
        }
        if let Some(g) = matches.opt_str("grouped-by") {
            grouped_by = Option::Some(g);
        }

        let files = matches.free.clone();

        Self {
            files,
            silent,
            skip_top_level,
            grouped_by,
        }
    }
}
