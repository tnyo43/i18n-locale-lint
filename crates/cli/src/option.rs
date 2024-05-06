use std::process::exit;
use getopts::Options;
use glob::{glob, Pattern};
use once_cell::sync::OnceCell;

pub struct CliOption {
    pub files: Vec<String>,
    pub silent: bool,
    pub skip_top_level: bool,
    pub grouped_by: Option<String>,
    pub group_size: Option<usize>,
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
            "Group locale file by a regular expression. (`-g \"^(.*/)([^/]+)$\"` by default)",
            "",
        );
        opts.optopt(
            "",
            "group-size",
            "If it is specified, fails when the size of a group is not equal to it.",
            "",
        );
        opts.optmulti(
            "",
            "ignore",
            "Ignore some files by globs. You can set this value multiple times.",
            "",
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
        let group_size = matches.opt_str("group-size").map(|s| {
            s.parse::<usize>()
                .unwrap_or_else(|_| panic!("invalid group_size value: {}", s))
        });

        let free = matches.free.clone();
        let mut files = match free.len() {
            // if the length of "free" is 1, assume it is a glob pattern
            1 => {
                let pattern = free[0].as_str();
                let files: Vec<String> = glob(pattern)
                    .unwrap_or_else(|_| panic!("Failed to read glob pattern: {}", pattern))
                    .map(|entry| match entry {
                        Ok(path) => path.to_str().unwrap().to_string(),
                        Err(e) => panic!("Error: {:?}", e),
                    })
                    .collect();
                files
            }
            _ => free,
        };
        let ignore_patterns = matches.opt_strs("ignore");
        for ignore in ignore_patterns {
            let pattern = Pattern::new(&ignore).unwrap();
            files.retain(|f| !pattern.matches(f));
        }

        Self {
            files,
            silent,
            skip_top_level,
            grouped_by,
            group_size,
        }
    }
}
