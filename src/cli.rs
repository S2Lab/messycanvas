use std::result;
use std::str as std_str;

use clap;

type Result<T> = result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "message displayed and the program should exit with status 0")]
    MessageDisplayed,
    #[fail(display = "invalid argument")]
    InvalidArgument,
    #[fail(display = "invalid command")]
    InvalidCommand,
}

impl Error {
    pub fn exit_code(&self) -> i32 {
        match *self {
            Error::MessageDisplayed => 0,
            _ => 1,
        }
    }
}

impl From<clap::Error> for Error {
    fn from(e: clap::Error) -> Error {
        if e.use_stderr() {
            eprintln!("{}", e.message);
            Error::InvalidArgument
        } else {
            println!("{}", e.message);
            Error::MessageDisplayed
        }
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

impl std_str::FromStr for CanvasCtlCommand {
    type Err = Error;
    fn from_str(s: &str) -> Result<CanvasCtlCommand> {
        match s {
            "cfgtest" => Ok(CanvasCtlCommand::CfgTest),
            _ => Err(Error::InvalidCommand),
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

        Ok(CanvasCtlArgs {
            command: matches
                .subcommand_name()
                .ok_or(Error::InvalidCommand)
                .and_then(|m| m.parse())
                .map_err(|e| {
                    eprintln!("{}", matches.usage());
                    e
                })?,
            config_path: matches
                .value_of("config-path")
                .unwrap_or(get_default_config_path())
                .into(),
            verbose: matches.is_present("verbose"),
        })
    }
}
