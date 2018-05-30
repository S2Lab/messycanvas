use clap;

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
            name: env!("CARGO_PKG_NAME").to_owned(),
            version: env!("CARGO_PKG_VERSION").to_owned(),
            authors: env!("CARGO_PKG_AUTHORS").replace(":", ",\n").to_owned(),
            description: env!("CARGO_PKG_DESCRIPTION").to_owned(),
        }
    }
}

fn get_default_config_path() -> &'static str {
    lazy_static! {
        static ref DEFAULT_CONFIG_PATH: &'static str = option_env!("CANVAS_DEFAULT_CONFIG_PATH").unwrap_or("/usr/local/etc/messycanvas/canvasd.toml");
    }

    return DEFAULT_CONFIG_PATH.as_ref();
}

fn build_parser<'a, 'b>(crate_info_ref: &'b CrateInfo) -> clap::App<'a, 'b> {
    lazy_static! {
        static ref CONFIG_PATH_DESCRIPTION: String = format!(
        r"Reads configuration from path
    This value can also be provided by environment variable
    `CANVAS_CONFIG_PATH`. If both being provided,
    only the value provided by command line argument will be used.
    Default: {}
    ", get_default_config_path());
    }

    clap::App::new(crate_info_ref.name.as_str())
        .version(crate_info_ref.version.as_str())
        .author(crate_info_ref.authors.as_str())
        .about(crate_info_ref.description.as_str())
        .arg(
            clap::Arg::with_name("config-path")
                .long("config-path")
                .short("c")
                .env("CANVAS_CONFIG_PATH")
                .takes_value(true)
                .long_help(&CONFIG_PATH_DESCRIPTION),
        )
        .arg(
            clap::Arg::with_name("verbose")
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
            config_path: matches
                .value_of("config-path")
                .unwrap_or(get_default_config_path())
                .into(),
            verbose: matches.is_present("verbose"),
        }
    }
}

pub enum CanvasCtlCommand {
    CfgTest,
}

pub struct CanvasCtlArgs {
    pub command: CanvasCtlCommand,
    pub config_path: String,
    pub verbose: bool,
}

impl CanvasCtlArgs {
    pub fn parse_args(crate_info_ref: &CrateInfo) -> CanvasCtlArgs {
        let parser = build_parser(crate_info_ref)
            .subcommand(clap::SubCommand::with_name("cfgtest").about("Tests configuration"));
        let matches = parser.get_matches();

        CanvasCtlArgs {
            command: CanvasCtlCommand::CfgTest,
            config_path: matches
                .value_of("config-path")
                .unwrap_or(get_default_config_path())
                .into(),
            verbose: matches.is_present("verbose"),
        }
    }
}
