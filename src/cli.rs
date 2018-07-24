use std::process;

use clap;

static CANVASD_NAME: &'static str = "cavasd";
static CANVASCTL_NAME: &'static str = "cavasctl";

fn get_default_config_path() -> &'static str {
    lazy_static! {
        static ref DEFAULT_CONFIG_PATH: &'static str = option_env!("CANVAS_DEFAULT_CONFIG_PATH")
            .unwrap_or("/usr/local/etc/messycanvas/canvasd.toml");
    }

    return DEFAULT_CONFIG_PATH.as_ref();
}

enum ParserType {
    Canvasd,
    CanvasCtl,
}

impl ParserType {
    fn build<'a, 'b>(&self) -> clap::App<'a, 'b> {
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

        let app = clap::App::new(match *self {
            ParserType::Canvasd => CANVASD_NAME,
            ParserType::CanvasCtl => CANVASCTL_NAME,
        }).version(crate_version!())
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
            );
        match *self {
            ParserType::Canvasd => app,
            ParserType::CanvasCtl => {
                app.subcommand(clap::SubCommand::with_name("cfgtest").about("Tests configuration"))
            }
        }
    }
}

pub struct CanvasdArgs {
    pub config_path: String,
    pub verbose: bool,
}

impl CanvasdArgs {
    pub fn parse_args() -> CanvasdArgs {
        let matches = ParserType::Canvasd.build().get_matches();

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

impl CanvasCtlCommand {
    pub fn maybe_from_str(s: Option<&str>) -> Option<CanvasCtlCommand> {
        match s {
            Some("cfgtest") => Some(CanvasCtlCommand::CfgTest),
            _ => None,
        }
    }
}

pub struct CanvasCtlArgs {
    pub command: CanvasCtlCommand,
    pub config_path: String,
    pub verbose: bool,
}

impl CanvasCtlArgs {
    pub fn parse_args() -> CanvasCtlArgs {
        let parser = ParserType::CanvasCtl.build();
        let matches = parser.get_matches();

        CanvasCtlArgs {
            command: match CanvasCtlCommand::maybe_from_str(matches.subcommand_name()) {
                Some(m) => m,
                None => {
                    println!("{}", matches.usage());
                    process::exit(1);
                }
            },
            config_path: matches
                .value_of("config-path")
                .unwrap_or(get_default_config_path())
                .into(),
            verbose: matches.is_present("verbose"),
        }
    }
}
