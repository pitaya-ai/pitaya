use std::sync::Mutex;

use pitaya_core::{EngineStatusDto, PitayaError};
use pitaya_engine::EngineHandle;
use tauri_specta::{collect_commands, Builder};

struct AppState {
    engine: EngineHandle,
}

#[tauri::command]
#[specta::specta]
fn get_engine_status(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<EngineStatusDto, PitayaError> {
    let app = state.lock().map_err(|_| PitayaError::Internal)?;
    Ok(app.engine.status())
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
