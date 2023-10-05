use prettytable::{row, Table};
use std::collections::HashSet;
use ubook::{
    api::SearchAPI,
    backend::BoluobaoHost,
    cli,
    share::{WorkSearchResult, WorkType},
};

fn handle_subcmd_backend(args: cli::BackendArgs) -> ubook::Result<()> {
    let _ = args;
    todo!()
}

async fn handle_subcmd_api(args: cli::APIArgs) -> ubook::Result<()> {
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

    let resp = request.send().await?;

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
    } else if let Ok(raw) = resp.text().await {
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

async fn handle_subcmd_search(args: cli::SearchArgs) -> ubook::Result<()> {
    let target = if args.target != "all" {
        args.target
            .chars()
            .filter_map(|c| match c {
                'n' => Some(Some(WorkType::Novel)),
                'c' => Some(Some(WorkType::Comic)),
                'a' => Some(Some(WorkType::Audiobook)),
                's' => Some(Some(WorkType::ShortStory)),
                _ => None,
            })
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>()
    } else {
        vec![None]
    };

    let mut host = BoluobaoHost::new();
    let mut work_list: Vec<WorkSearchResult> = Default::default();
    for work_type in target {
        host.search(
            &args.keyword,
            args.limit,
            work_type,
            Box::new(|works: Vec<WorkSearchResult>| {
                work_list.extend(works);
                true
            }),
        )
        .await?;
    }
    work_list.sort_by(|lhs, rhs| rhs.popularity.cmp(&lhs.popularity));
    if args.limit > 0 && work_list.len() > args.limit {
        work_list.drain(args.limit..);
    }

    fn conv(x: usize) -> String {
        if x > 100000 {
            format!("{:.2}w", x as f64 / 1e4)
        } else if x > 1000 {
            format!("{:.2}k", x as f64 / 1e3)
        } else {
            x.to_string()
        }
    }

    if args.json {
        println!("{}", serde_json::to_string(&work_list)?)
    } else if !work_list.is_empty() {
        let mut table = Table::new();
        table.set_format(*prettytable::format::consts::FORMAT_NO_COLSEP);
        table.set_titles(row![bc => "Type", "ID" , "Name", "Author", "Tags", "Popularity"]);
        for work in work_list {
            table.add_row(row![
                lb -> format!("{:?}", work.r#type).as_str(),
                lb -> work.work_id.to_string().as_str(),
                c -> work.work_name.as_str(),
                c -> work.author_name.as_str(),
                r -> work.tags.join(" ").as_str(),
                r -> conv(work.popularity).as_str(),
            ]);
        }
        table.printstd();
    }

    Ok(())
}

#[tokio::main]
async fn main() -> ubook::Result<()> {
    use clap::Parser;
    match cli::Cli::parse().subcmd {
        cli::SubCli::Backend(args) => handle_subcmd_backend(args),
        cli::SubCli::API(args) => handle_subcmd_api(args).await,
        cli::SubCli::Auth(args) => handle_subcmd_auth(args),
        cli::SubCli::Search(args) => handle_subcmd_search(args).await,
    }
}
