use clap::ArgMatches;
use ubook::backend::BoluobaoHost;

use ubook::cli::get_cli_parser;

fn handle_auth_subcmd(matches: &ArgMatches) -> ubook::Result<()> {
    let _ = matches;
    todo!()
}

fn handle_status_subcmd(matches: &ArgMatches) -> ubook::Result<()> {
    let _ = matches;
    todo!()
}

fn handle_api_subcmd(matches: &ArgMatches) -> ubook::Result<()> {
    let host = BoluobaoHost::new();
    let url = matches.get_one::<String>("URL").unwrap();
    let method = matches.get_one::<String>("METHOD").unwrap().to_owned();

    let mut request = match method.as_str() {
        "GET" => host.api_get(url),
        "POST" => host.api_post(url),
        _ => anyhow::bail!("unsupported method {}", method),
    };

    if let Some(params) = matches.get_one::<String>("params") {
        request = request.query(
            &params
                .split("&")
                .map(|pair| pair.split_once("=").unwrap())
                .collect::<Vec<(&str, &str)>>(),
        )
    };

    if let Some(data) = matches.get_one::<String>("data") {
        request = request.body(data.to_owned())
    };

    if let Some(cookies) = matches.get_one::<String>("cookies") {
        request = request.header(reqwest::header::COOKIE, cookies)
    };

    let resp = request.send()?;
    println!("{:#?}", resp);

    let data = resp.text()?.parse::<serde_json::Value>()?;
    println!("{:#?}", data);

    Ok(())
}

fn main() -> ubook::Result<()> {
    match get_cli_parser().get_matches().subcommand() {
        Some(("auth", matches)) => handle_auth_subcmd(matches)?,
        Some(("status", matches)) => handle_status_subcmd(matches)?,
        Some(("api", matches)) => handle_api_subcmd(matches)?,
        _ => unreachable!(),
    }
    Ok(())
}
