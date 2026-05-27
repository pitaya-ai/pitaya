use std::sync::Mutex;

use pitaya_engine::EngineHandle;
use serde::Serialize;
use specta::Type;
use tauri_specta::{collect_commands, Builder};

struct AppState {
    engine: EngineHandle,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "snake_case")]
struct EngineStatusResponse {
    state: String,
    version: String,
}

#[tauri::command]
#[specta::specta]
fn get_engine_status(state: tauri::State<'_, Mutex<AppState>>) -> EngineStatusResponse {
    let dto = state.lock().expect("app state poisoned").engine.status();
    EngineStatusResponse {
        state: format!("{:?}", dto.state),
        version: dto.version,
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let specta_builder =
        Builder::<tauri::Wry>::new().commands(collect_commands![get_engine_status]);

    #[cfg(debug_assertions)]
    specta_builder
        .export(
            specta_typescript::Typescript::default(),
            "../src/bindings.ts",
        )
        .expect("failed to export typescript bindings");

    let engine = EngineHandle::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(specta_builder.invoke_handler())
        .manage(Mutex::new(AppState {
            engine: engine.clone(),
        }))
        .setup(move |_app| {
            tauri::async_runtime::spawn(async move {
                pitaya_engine::run().await;
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running pitaya-desktop");
}
