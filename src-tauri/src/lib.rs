// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn is_valid_email(email: &str) -> bool {
    if !email.ends_with("@ciencias.unam.mx") {
        return false;  
    }
    return true; 
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![is_valid_email])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
