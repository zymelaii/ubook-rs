use super::*;
use crate::{
    share::{WorkSearchResult, WorkType},
    BackendCore, UbookContext,
};
use colored::Colorize;
use prettytable::{row, Table};
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

pub fn handle_subcmd_backend(context: &mut UbookContext, args: BackendArgs) -> crate::Result<()> {
    let manager = &mut context.backend_manager;
    let backend_list = manager.get_backend_list();

    // TODO:
    // 1. -R, --clean: clean-up local storage
    // 2. --init: perform initialization

    if args.list {
        backend_list
            .into_iter()
            .map(|id| {
                let enabled = manager.is_enabled(id.as_str());
                let active = manager
                    .active_backend_id()
                    .map_or(false, |active_id| active_id == id);
                if !enabled {
                    format!("  {id}").white()
                } else {
                    format!("{} {id}", if active { '*' } else { ' ' }).green()
                }
            })
            .for_each(|row| println!("{}", row.bold()));
        return Ok(());
    }

    let backends = args
        .backends
        .into_iter()
        .filter(|id| {
            if manager.is_available(id) {
                true
            } else {
                let msg = format!("WARNING: backend `{id}` is not available");
                println!("{}", msg.yellow());
                false
            }
        })
        .collect::<Vec<String>>();
    if args.disable {
        for id in backends {
            manager.disable(id.as_str())?;
        }
    } else if args.enable {
        for id in backends {
            manager.enable(id.as_str())?;
        }
    }

    if let Some(id) = args.r#use {
        manager.activate(id.as_str())?;
    }

    Ok(())
}

pub async fn handle_subcmd_api(context: &UbookContext, args: APIArgs) -> crate::Result<()> {
    use reqwest::header::{CONTENT_TYPE, COOKIE};

    // NOTE: backend priority
    // 1. the active backend
    // 2. the first enabled backend
    // 3. the first available backend
    // 4. report error
    let manager = &context.backend_manager;
    let backend_list = manager.get_backend_list();
    if backend_list.is_empty() {
        anyhow::bail!("no backend available")
    }
    let backend_id = if let Some(active_id) = manager.active_backend_id() {
        active_id
    } else {
        let enabled_backends = backend_list
            .iter()
            .filter(|id| manager.is_enabled(id))
            .collect::<Vec<&String>>();
        enabled_backends.first().map_or(
            backend_list.first().expect("unexpected error").to_owned(),
            |e| e.to_owned().to_owned(),
        )
    };

    let backend = manager.try_get_instance_of(backend_id.as_str())?;

    let backend = backend.lock().await;
    let request = match args.method {
        RequestMethod::GET => backend.api_get(args.url.as_ref()),
        RequestMethod::POST => backend.api_post(args.url.as_ref()),
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

pub fn handle_subcmd_auth(_context: &mut UbookContext, _args: AuthArgs) -> crate::Result<()> {
    todo!()
}

pub async fn handle_subcmd_search(context: &UbookContext, args: SearchArgs) -> crate::Result<()> {
    let manager = &context.backend_manager;
    let enabled_backends = manager
        .get_backend_list()
        .into_iter()
        .filter_map(|id| {
            if manager.is_enabled(id.as_str()) {
                manager.try_get_instance_of(id.as_str()).ok()
            } else {
                None
            }
        })
        .collect::<Vec<Arc<async_mutex::Mutex<Box<dyn BackendCore>>>>>();
    if enabled_backends.is_empty() {
        let msg = format!("WARNING: no backends available for search");
        println!("{}", msg.yellow());
        return Ok(());
    }

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

    let work_list: Arc<Mutex<Vec<(String, WorkSearchResult)>>> = Default::default();
    for backend in enabled_backends {
        let mut backend = backend.lock().await;
        for work_type in target.iter() {
            let list = work_list.clone();
            let backend_id = backend.backend_id();
            backend
                .search(
                    &args.keyword,
                    args.limit,
                    *work_type,
                    Box::new(move |works: Vec<WorkSearchResult>| {
                        list.lock()
                            .expect("mutex panicked unexpectedly")
                            .extend(works.into_iter().map(|e| (backend_id.into(), e)));
                        true
                    }),
                )
                .await?;
        }
    }

    let mut work_list = Arc::into_inner(work_list)
        .expect("`work_list` is expected to be strongly held")
        .into_inner()
        .expect("mutex panicked unexpectedly");
    work_list.sort_by(|lhs, rhs| rhs.1.popularity.cmp(&lhs.1.popularity));
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
        table.set_titles(
            row![bc => "Backend", "Type", "ID" , "Name", "Author", "Tags", "Popularity"],
        );
        for (backend_id, work) in work_list {
            table.add_row(row![
                cbFg -> backend_id,
                lb -> format!("{:?}", work.r#type).as_str(),
                lb -> work.work_id.to_string().as_str(),
                cbFy -> work.work_name.as_str(),
                cb -> work.author_name.as_str(),
                rbFm -> work.tags.join(" ").as_str(),
                rbiFr -> conv(work.popularity).as_str(),
            ]);
        }
        table.printstd();
    }

    Ok(())
}
