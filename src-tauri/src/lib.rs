use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_user, login])
        .manage(Mutex::new(User {
            name: String::from(""),
            password: String::from(""),
        }))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_user() -> Vec<User> {
    let user1 = User {
        name: String::from("Some"),
        password: String::from("User")
    };

    let user2 = User {
        name: String::from("Some Other"),
        password: String::from("User")
    };

    vec![user1, user2]
}

#[tauri::command]
fn login(name: String, password: String) -> bool {
    true
}

#[derive(serde::Serialize)]
struct User {
    name: String,
    password: String,
}
