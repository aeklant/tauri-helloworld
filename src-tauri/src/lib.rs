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
fn get_user(user_state: tauri::State<Mutex<User>>) -> User {
    let user = &*user_state.lock().unwrap();

    user.clone()
}

#[tauri::command]
fn login(user_state: tauri::State<Mutex<User>>, user: User) {
    // Given a user state and some login credentials
    // Update the state to contain said credentials
    *user_state.lock().unwrap() = user;
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct User {
    name: String,
    password: String,
}
