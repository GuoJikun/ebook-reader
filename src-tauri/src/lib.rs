use txt_novel_parser::Novel;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn read_txt(path: &str) -> Novel {
    let content_v8 = std::fs::read(path);
    if content_v8.is_err() {
        return Novel::default();
    }
    txt_novel_parser::parse_txt(&content_v8.unwrap())
}

mod shared;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build());

    builder = builder.plugin(tauri_plugin_opener::init());

    let app = builder
        .setup(|mut app| {
            let _ = shared::Novel::new(&mut app);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![read_txt])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, code, .. } => {
            if code.is_none() {
                api.prevent_exit();
            }
        },
        _ => {},
    });
}
