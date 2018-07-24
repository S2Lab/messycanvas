use std::process;
use std::result;

use clap;

type Result<T> = result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "{}", _0)]
    Clap(#[cause] clap::Error),
    #[fail(display = "invalid command")]
    PrintUsage(String),
}

impl Error {
    pub fn print_and_exit(&self) {
        match *self {
            Error::Clap(ref e) => e.exit(),
            Error::PrintUsage(ref s) => {
                eprintln!("{}", s);
                process::exit(1);
            }
        }
    }
}

impl From<clap::Error> for Error {
    fn from(e: clap::Error) -> Error {
        Error::Clap(e)
    }
}

fn get_default_config_path() -> &'static str {
    lazy_static! {
        static ref DEFAULT_CONFIG_PATH: &'static str = option_env!("CANVAS_DEFAULT_CONFIG_PATH")
            .unwrap_or("/usr/local/etc/messycanvas/canvasd.toml");
    }

    return DEFAULT_CONFIG_PATH.as_ref();
}

fn build_parser<'a, 'b>(binary_name: &'static str) -> clap::App<'a, 'b> {
    lazy_static! {
        static ref CONFIG_PATH_DESCRIPTION: String = format!(
            r"Reads configuration from path
    This value can also be provided by environment variable
    `CANVAS_CONFIG_PATH`. If both provided,
    only the value from command line argument will be used.
    Default: {}
    ",
            get_default_config_path()
        );
    }

    clap::App::new(binary_name)
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
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

#[derive(Debug)]
pub struct CanvasdArgs {
    pub config_path: String,
    pub verbose: bool,
}

impl CanvasdArgs {
    pub fn parse_args() -> Result<CanvasdArgs> {
        let matches = build_parser("canvasd").get_matches_safe()?;

        Ok(CanvasdArgs {
            config_path: matches
                .value_of("config-path")
                .unwrap_or(get_default_config_path())
                .into(),
            verbose: matches.is_present("verbose"),
        })
    }
}

#[derive(Debug)]
pub enum CanvasCtlCommand {
    CfgTest,
}

impl CanvasCtlCommand {
    pub fn maybe_from_str(s: &str) -> Option<CanvasCtlCommand> {
        match s {
            "cfgtest" => Some(CanvasCtlCommand::CfgTest),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct CanvasCtlArgs {
    pub command: CanvasCtlCommand,
    pub config_path: String,
    pub verbose: bool,
}

impl CanvasCtlArgs {
    pub fn parse_args() -> Result<CanvasCtlArgs> {
        let matches = build_parser("canvasctl")
            .subcommand(clap::SubCommand::with_name("cfgtest").about("Tests configuration"))
            .get_matches_safe()?;

        let get_usage_err = || Error::PrintUsage(matches.usage().to_owned());

        Ok(CanvasCtlArgs {
            command: CanvasCtlCommand::maybe_from_str(matches
                .subcommand_name()
                .ok_or_else(get_usage_err)?)
                .ok_or_else(get_usage_err)?,
            config_path: matches
                .value_of("config-path")
                .unwrap_or(get_default_config_path())
                .into(),
            verbose: matches.is_present("verbose"),
        })
    }
}
