use clap::{arg, ArgAction, Command};

pub fn get_cli_parser() -> Command {
    let auth = Command::new("auth")
        .about("Authenticate ubook with boluobao")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("login")
                .about("Authenticate with a boluobao host")
                .arg(arg!(--username -U <USERNAME> "The user to authenticate"))
                .arg(
                    arg!(--account -u <ACCOUNT> "The account to authenticate with")
                        .required_unless_present("username"),
                )
                .arg(
                    arg!(--password -p <PASSWORD> "The password to authenticate with")
                        .required_unless_present("username"),
                )
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("logout")
                .about("Log out of a boluobao host")
                .arg(arg!(--all -a "Logout all authenticated users"))
                .arg(arg!([USER]... "Users to logout"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("status")
                .about("View authentication status")
                .subcommand(Command::new("list").about("List authenticated users"))
                .subcommand(
                    Command::new("view")
                        .about("Display the information of an authenticated user")
                        .arg(arg!(<USER> "The user to display"))
                        .arg_required_else_help(true),
                )
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("refresh")
                .about("Refresh stored authentication credentials")
                .arg_required_else_help(true),
        );

    let query = Command::new("query")
        .arg_required_else_help(true)
        .arg(arg!(<URL>))
        .arg(arg!([EXPAND]))
        .arg(arg!(--params <PARAMS>));

    let api = Command::new("api")
        .about("Make an API request of the active backend")
        .arg_required_else_help(true)
        .arg(arg!(<METHOD> "Request method to use"))
        .arg(arg!(<URL> "API url"))
        .arg(arg!(--params -p <PARAMS> "Params for GET request"))
        .arg(arg!(--data -d <DATA> "Data for POST request"))
        .arg(arg!(--cookies -c <COOKIES> "Cookies to use"))
        .arg(arg!(--head -I "Show document info only").action(ArgAction::SetTrue));

    Command::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(auth)
        .subcommand(query)
        .subcommand(api)
}
