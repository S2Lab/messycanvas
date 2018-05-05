use clap::{App, Arg};

pub struct ParsedArgs {
    pub config_path: String,
    pub verbose: bool,
}

pub fn parse_args() -> ParsedArgs {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!(",\n"))
        .about(crate_description!())
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
        .get_matches();

    ParsedArgs {
        config_path: matches.value_of("config-path").unwrap().into(),
        verbose: matches.is_present("verbose"),
    }
}
