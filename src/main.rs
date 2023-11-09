use directories::ProjectDirs;
use std::fs;
use ubook::{
    backend::BoluobaoHost,
    cli,
    storage::{BackendStatus, ContextStorage},
    UbookContext,
};

fn init_local_storage() -> ubook::Result<()> {
    let proj_dir = ProjectDirs::from("", "", "ubook").expect("unknown error");
    let data_dir = proj_dir.data_local_dir();
    if !data_dir.exists() {
        fs::create_dir_all(data_dir)?;
    }
    Ok(())
}

fn init_context_from_local_storage(context: &mut UbookContext) -> ubook::Result<()> {
    let proj_dir = ProjectDirs::from("", "", "ubook").expect("unknown error");
    let data_dir = proj_dir.data_local_dir();

    let data_file = data_dir.join("ubook.json");
    if !data_file.exists() {
        let storage = ContextStorage::default();
        let raw = serde_json::to_string_pretty(&storage)?;
        fs::write(data_file.clone(), raw)?;
    }

    let raw = fs::read(data_file).map(String::from_utf8)??;
    let storage = serde_json::from_str::<ContextStorage>(raw.as_str())?;
    let backends = &storage.backends;

    let manager = &mut context.backend_manager;
    for backend in backends {
        if !manager.is_available(&backend.id) {
            continue;
        }
        if backend.active {
            manager.activate(&backend.id)?;
            continue;
        }
        if backend.enabled {
            manager.enable(&backend.id)?;
        } else {
            manager.disable(&backend.id)?;
        }
    }

    Ok(())
}

fn dump_context_to_local_storage(context: &UbookContext) -> ubook::Result<()> {
    let manager = &context.backend_manager;
    let active_id = manager.active_backend_id();

    let storage = ContextStorage {
        backends: manager
            .get_backend_list()
            .into_iter()
            .map(|id| BackendStatus {
                id: id.clone(),
                active: active_id
                    .as_ref()
                    .map_or(false, |active_id| *active_id == id),
                enabled: manager.is_enabled(&id),
            })
            .collect(),
    };

    let raw = serde_json::to_string_pretty(&storage)?;

    let proj_dir = ProjectDirs::from("", "", "ubook").expect("unknown error");
    let data_dir = proj_dir.data_local_dir();

    let data_file = data_dir.join("ubook.json");
    fs::write(data_file, raw)?;

    Ok(())
}

#[tokio::main]
async fn main() -> ubook::Result<()> {
    let mut context = UbookContext::new();
    let manager = &mut context.backend_manager;
    manager.add_backend(Box::new(BoluobaoHost::new()));

    init_local_storage()?;
    init_context_from_local_storage(&mut context)?;

    use clap::Parser;
    let cli_result = match cli::Cli::parse().subcmd {
        cli::SubCli::Backend(args) => cli::handle_subcmd_backend(&mut context, args),
        cli::SubCli::API(args) => cli::handle_subcmd_api(&context, args).await,
        cli::SubCli::Auth(args) => cli::handle_subcmd_auth(&mut context, args),
        cli::SubCli::Search(args) => cli::handle_subcmd_search(&context, args).await,
    };

    let dump_result = dump_context_to_local_storage(&context);

    cli_result?;
    dump_result?;

    Ok(())
}
