use clap::{App, Arg};

#[derive(Debug, Clone)]
pub struct CrateInfo {
    pub name: String,
    pub version: String,
    pub authors: String,
    pub description: String,
}

#[macro_export]
macro_rules! canvas_crate_info {
    () => {
        $crate::cli::CrateInfo {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            authors: env!("CARGO_PKG_AUTHORS").to_string(),
            description: env!("CARGO_PKG_DESCRIPTION").to_string(),
        }
    }
}

fn build_parser<'a, 'b>(crate_info_ref: &'b CrateInfo) -> App<'a, 'b> {
    App::new(crate_info_ref.name.as_str())
        .version(crate_info_ref.version.as_str())
        .author(crate_info_ref.authors.as_str())
        .about(crate_info_ref.description.as_str())
        .arg(
            Arg::with_name("config-path")
                .long("config-path")
                .short("c")
                .env("CANVAS_CONFIG_PATH")
                .takes_value(true)
                .required(true)
                .long_help(
                    "Reads configuration from path
This value can also be provided by environment variable
`CANVAS_CONFIG_PATH`. If both being provided,
only the value provided by command line argument will be used.
",
                ),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Enters verbose mode"),
        )
}

pub struct CanvasdArgs {
    pub config_path: String,
    pub verbose: bool,
}

impl CanvasdArgs {
    pub fn parse_args(crate_info_ref: &CrateInfo) -> CanvasdArgs {
        let matches = build_parser(crate_info_ref).get_matches();

        CanvasdArgs {
            config_path: matches.value_of("config-path").unwrap().into(),
            verbose: matches.is_present("verbose"),
        }
    }
}
