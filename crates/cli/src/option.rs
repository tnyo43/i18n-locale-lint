use getopts::Options;
use once_cell::sync::OnceCell;

pub struct Option {
    pub files: Vec<String>,
    pub silent: bool,
    pub skip_top_level: bool,
}
pub static INSTANCE: OnceCell<Option> = OnceCell::new();

impl Option {
    pub fn from_cli(args: &[String]) -> Self {
        let mut silent = false;
        let mut skip_top_level = false;

        let mut opts = Options::new();

        opts.optflag("s", "silent", "Don't display logs other than errors.");
        opts.optflag(
            "",
            "skip-top-level",
            "Assuming the top level is composed solely of a single key, skip it.",
        );

        let matches = match opts.parse(args) {
            Ok(m) => m,
            Err(f) => panic!("{}", f.to_string()),
        };

        if matches.opt_present("s") {
            silent = true;
        }
        if matches.opt_present("skip-top-level") {
            skip_top_level = true;
        }
        let files = matches.free.clone();

        Self {
            files,
            silent,
            skip_top_level,
        }
    }
}
