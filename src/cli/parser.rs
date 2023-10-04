use clap::{
    error::{Error, ErrorKind},
    ArgMatches, Args, Command, FromArgMatches, Parser, Subcommand, ValueEnum,
};

#[derive(Parser)]
#[command(
    about = "Manage backends",
    long_about = None,
    arg_required_else_help(true),
)]
pub struct BackendArgs {
    #[arg(short, long, help = "List available backends")]
    pub list: bool,
    #[arg(short = 'u', long, help = "Enable the given backends")]
    pub enable: bool,
    #[arg(short = 'r', long, help = "Disable the given backends")]
    pub disable: bool,
    #[arg(long, help = "Perform initialization")]
    pub init: bool,
    #[arg(short = 'R', long, help = "Clean up local storages")]
    pub clean: bool,
    #[arg(
        short = 'U',
        long,
        id = "BACKEND",
        help = "Enable and activate the specific backend"
    )]
    pub r#use: Option<String>,
    #[arg(help = "Backends to work with")]
    pub backends: Vec<String>,
}

#[derive(ValueEnum, Clone)]
#[value(rename_all = "UPPER")]
pub enum RequestMethod {
    GET,
    POST,
}

#[derive(Parser)]
#[command(
    about = "Make an API request of the active backend",
    long_about = None,
    arg_required_else_help(true),
)]
pub struct APIArgs {
    #[arg(help = "Request method to use")]
    pub method: RequestMethod,
    #[arg(help = "URL path to work with")]
    pub url: String,
    #[arg(short, long, help = "Specify HTTP GET parameters")]
    pub params: Option<String>,
    #[arg(short, long, help = "Specify HTTP POST data")]
    pub data: Option<String>,
    #[arg(short, long, help = "Specify cookies to use")]
    pub cookies: Option<String>,
    #[arg(short = 'I', long, help = "Show document info only")]
    pub head: bool,
}

#[derive(Parser)]
#[command(
    about = "Authenticate with an enabled backend host",
    long_about = None,
    arg_required_else_help(true),)
]
pub struct AuthLoginArgs {
    #[arg(
        short = 'c',
        long,
        id = "BACKEND",
        help = "The backend to authenticate with"
    )]
    pub host: String,
    #[arg(short = 'U', long = "user", help = "Specify username")]
    username: Option<String>,
    #[arg(short = 'u', long, help = "Specify account")]
    account: Option<String>,
    #[arg(short, long, help = "Specify password")]
    password: Option<String>,
    #[arg(long, id = "TOKEN", help = "Specify token")]
    with_token: Option<String>,
    #[arg(short, long, help = "Force to re-authenticate")]
    force: bool,
}

#[derive(Parser)]
#[command(about = "Log out of an authenticated backend host", long_about = None)]
pub struct AuthLogoutArgs {
    #[arg(short, long, help = "Log out all backends")]
    pub all: bool,
    #[arg(help = "Backends to log out of")]
    pub backends: Vec<String>,
}

#[derive(Parser)]
#[command(about = "View authentication status", long_about = None)]
pub struct AuthStatusArgs {
    #[arg(
        short = 'c',
        long,
        id = "BACKEND",
        help = "Check a specific backend's auth status"
    )]
    host: Option<String>,
}

#[derive(Parser)]
#[command(about = "Print the auth tokens", long_about = None)]
pub struct AuthTokenArgs {
    #[arg(short, long, help = "Print all tokens")]
    all: bool,
    #[arg(
        short = 'c',
        long,
        id = "BACKEND",
        help = "Print a specific backend's auth token"
    )]
    host: Option<String>,
}

pub enum AuthSubCli {
    Login(AuthLoginArgs),
    Logout(AuthLogoutArgs),
    Status(AuthStatusArgs),
    Token(AuthTokenArgs),
}

#[derive(Parser)]
#[command(
    about = format!("Authenticate {} with available backends", env!("CARGO_PKG_NAME")),
    long_about = None,
    arg_required_else_help(true),
)]
pub struct AuthArgs {
    #[command(subcommand)]
    pub subcmd: AuthSubCli,
}

#[derive(Parser)]
#[command(
    about = "Search across available backends",
    long_about = None,
    arg_required_else_help(true),
)]
pub struct SearchArgs {
    #[arg(
        short,
        long,
        default_value = "all",
        help = "Specify targets for the search",
        long_help = r#"Specify targets for the search and possible targets are
  + all: search for all types of work
  + n: search for novels
  + c: search for comics
  + a: search for albums
  + s: search for short stories
use `all` or any combination of n/c/s/a to indicates the expected targets"#
    )]
    pub target: String,
    #[arg(
        short = 'n',
        long,
        default_value = "10",
        help = "Specify the max number of results",
    )]
    pub limit: usize,
    #[arg(help = "Specify the keywords")]
    pub keyword: String,
}

pub enum SubCli {
    Backend(BackendArgs),
    API(APIArgs),
    Auth(AuthArgs),
    Search(SearchArgs),
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub subcmd: SubCli,
}

impl FromArgMatches for AuthSubCli {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        match matches.subcommand() {
            Some(("login", args)) => Ok(Self::Login(AuthLoginArgs::from_arg_matches(args)?)),
            Some(("logout", args)) => Ok(Self::Logout(AuthLogoutArgs::from_arg_matches(args)?)),
            Some(("status", args)) => Ok(Self::Status(AuthStatusArgs::from_arg_matches(args)?)),
            Some(("token", args)) => Ok(Self::Token(AuthTokenArgs::from_arg_matches(args)?)),
            Some((_, _)) => Err(Error::raw(
                ErrorKind::InvalidSubcommand,
                "invalid subcommand",
            )),
            None => Err(Error::raw(
                ErrorKind::MissingSubcommand,
                "missing subcommand",
            )),
        }
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error> {
        match matches.subcommand() {
            Some(("login", args)) => *self = Self::Login(AuthLoginArgs::from_arg_matches(args)?),
            Some(("logout", args)) => *self = Self::Logout(AuthLogoutArgs::from_arg_matches(args)?),
            Some(("status", args)) => *self = Self::Status(AuthStatusArgs::from_arg_matches(args)?),
            Some(("token", args)) => *self = Self::Token(AuthTokenArgs::from_arg_matches(args)?),
            Some((_, _)) => {
                return Err(Error::raw(
                    ErrorKind::InvalidSubcommand,
                    "invalid subcommand",
                ))
            }
            None => (),
        };
        Ok(())
    }
}

impl Subcommand for AuthSubCli {
    fn augment_subcommands(cmd: Command) -> Command {
        cmd.subcommand(AuthLoginArgs::augment_args(Command::new("login")))
            .subcommand(AuthLogoutArgs::augment_args(Command::new("logout")))
            .subcommand(AuthStatusArgs::augment_args(Command::new("status")))
            .subcommand(AuthTokenArgs::augment_args(Command::new("token")))
    }

    fn augment_subcommands_for_update(cmd: Command) -> Command {
        Self::augment_subcommands(cmd)
    }

    fn has_subcommand(name: &str) -> bool {
        matches!(name, "login" | "logout" | "status" | "token")
    }
}

impl FromArgMatches for SubCli {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        match matches.subcommand() {
            Some(("backend", args)) => Ok(Self::Backend(BackendArgs::from_arg_matches(args)?)),
            Some(("api", args)) => Ok(Self::API(APIArgs::from_arg_matches(args)?)),
            Some(("auth", args)) => Ok(Self::Auth(AuthArgs::from_arg_matches(args)?)),
            Some(("search", args)) => Ok(Self::Search(SearchArgs::from_arg_matches(args)?)),
            Some((_, _)) => Err(Error::raw(
                ErrorKind::InvalidSubcommand,
                "invalid subcommand",
            )),
            None => Err(Error::raw(
                ErrorKind::MissingSubcommand,
                "missing subcommand",
            )),
        }
    }

    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error> {
        match matches.subcommand() {
            Some(("backend", args)) => *self = Self::Backend(BackendArgs::from_arg_matches(args)?),
            Some(("api", args)) => *self = Self::API(APIArgs::from_arg_matches(args)?),
            Some(("auth", args)) => *self = Self::Auth(AuthArgs::from_arg_matches(args)?),
            Some(("search", args)) => *self = Self::Search(SearchArgs::from_arg_matches(args)?),
            Some((_, _)) => {
                return Err(Error::raw(
                    ErrorKind::InvalidSubcommand,
                    "invalid subcommand",
                ))
            }
            None => (),
        };
        Ok(())
    }
}

impl Subcommand for SubCli {
    fn augment_subcommands(cmd: Command) -> Command {
        cmd.subcommand(BackendArgs::augment_args(Command::new("backend")))
            .subcommand(APIArgs::augment_args(Command::new("api")))
            .subcommand(AuthArgs::augment_args(Command::new("auth")))
            .subcommand(SearchArgs::augment_args(Command::new("search")))
    }

    fn augment_subcommands_for_update(cmd: Command) -> Command {
        Self::augment_subcommands(cmd)
    }

    fn has_subcommand(name: &str) -> bool {
        matches!(name, "backend" | "api" | "auth" | "search")
    }
}
