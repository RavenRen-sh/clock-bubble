#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn exit_app(app_handle: tauri::AppHandle) {
    app_handle.exit(0);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![exit_app])
        .setup(|app| {
            // macOS 下隐藏 Dock 栏图标，使其作为纯桌面挂件运行
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            let _ = app;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
