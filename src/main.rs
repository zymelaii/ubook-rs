use ubook::{backend::BoluobaoHost, cli};

fn handle_subcmd_backend(args: cli::BackendArgs) -> ubook::Result<()> {
    let _ = args;
    todo!()
}

fn handle_subcmd_api(args: cli::APIArgs) -> ubook::Result<()> {
    use reqwest::header::{CONTENT_TYPE, COOKIE};
    use ubook::cli::RequestMethod;

    let host = BoluobaoHost::new();
    let request = match args.method {
        RequestMethod::GET => host.api_get(args.url.as_ref()),
        RequestMethod::POST => host.api_post(args.url.as_ref()),
    };
    let request = request
        .header(COOKIE, args.cookies.unwrap_or_default())
        .body(args.data.unwrap_or_default())
        .query(
            &args
                .params
                .unwrap_or_default()
                .split('&')
                .filter(|pair| !pair.is_empty())
                .map(|pair| pair.split_once('=').expect("invalid param pair"))
                .collect::<Vec<(&str, &str)>>(),
        );

    let resp = request.send()?;

    let is_json = resp
        .headers()
        .get(CONTENT_TYPE)
        .map(|e| {
            e.to_str()
                .expect("invalid content-type value")
                .contains("application/json")
        })
        .unwrap_or(false);

    if args.head {
        println!("{:?} {}", resp.version(), resp.status());
        for (name, value) in resp.headers() {
            println!("{name}: {}", value.to_str()?);
        }
    } else if let Ok(raw) = resp.text() {
        if is_json {
            let data = raw.parse::<serde_json::Value>()?;
            let json_doc = serde_json::to_string_pretty(&data)?;
            println!("{json_doc}");
        } else {
            print!("{raw}");
        }
    }

    Ok(())
}

fn handle_subcmd_auth(args: cli::AuthArgs) -> ubook::Result<()> {
    let _ = args;
    todo!()
}

fn main() -> ubook::Result<()> {
    use clap::Parser;
    match cli::Cli::parse().subcmd {
        cli::SubCli::Backend(args) => handle_subcmd_backend(args),
        cli::SubCli::API(args) => handle_subcmd_api(args),
        cli::SubCli::Auth(args) => handle_subcmd_auth(args),
    }
}
