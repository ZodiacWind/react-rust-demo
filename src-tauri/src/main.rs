#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Submenu, Menu, MenuItem, Manager};
//无参调用
#[tauri::command]
fn my_custom_command() {
  println!("I was invoked from JS!");
}
//有参调用
#[tauri::command]
fn my_custom_command_param(invoke_message: String) {
  println!("I was invoked from JS, with this message: {}", invoke_message);
}
//有返回值调用
#[tauri::command]
fn my_custom_command_return() -> String {
  "Hello from Rust!".into()
}
//有返回值Result调用
#[tauri::command]
fn my_custom_command_result() -> Result<String, String> {
  // If something fails
  Err("This failed!".into())
  // If it worked
  // Ok("This worked!".into())
}
//异步调用
#[tauri::command]
async fn my_custom_command_async() {
  // Call another async function and wait for it to finish
  // let result = some_async_function().await;
  println!("Result: {}", "123");
}
 //可以访问windows实例
#[tauri::command]
async fn my_custom_command_windows(window: tauri::Window) {
  println!("Window: {}", window.label());
}
//注册热键
#[tauri::command]
async fn my_custom_command_handle(app_handle: tauri::AppHandle) {
  let app_dir = app_handle.path_resolver().app_dir();
  print!("{:#?}",app_dir);
  use tauri::GlobalShortcutManager;
 let _ =  app_handle.global_shortcut_manager().register("CTRL + U", move || { println!("热键。。。。")});
}
//全局状态
struct MyState(String);
#[tauri::command]
fn my_custom_command_state(state: tauri::State<MyState>) {
  println!("{}", state.0 == "some state value");
}
//向前端发出事件
#[tauri::command]
async fn my_custom_command_emit(window: tauri::Window) {
  let app = window.app_handle();
  //向前端发出事件
  app.emit_all("_error", Payload { message: "Tauri is awesome!".into() }).unwrap();
}

#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}
fn main() {

  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
let close = CustomMenuItem::new("close".to_string(), "Close");
let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
let menu = Menu::new()
  // .add_native_item(MenuItem::Copy)
  // .add_item(CustomMenuItem::new("hide", "Hide"))
  .add_submenu(submenu);

  tauri::Builder::default()
  .manage(MyState("some state value".into()))
  .menu(menu)
  .invoke_handler(tauri::generate_handler![
    my_custom_command,my_custom_command_param,
    my_custom_command_return,my_custom_command_result,
    my_custom_command_async,my_custom_command_windows,
    my_custom_command_handle,my_custom_command_state,
    my_custom_command_emit
    
    ])  .setup(|app| {
      // listen to the `event-name` (emitted on any window)
      let id = app.listen_global("event-name", |event| {
        println!("got event-name with payload {:?}", event.payload());
      });
      // unlisten to the event using the `id` returned on the `listen_global` function
      // an `once_global` API is also exposed on the `App` struct
      // app.unlisten(id);

      // emit the `event-name` event to all webview windows on the frontend
      //windows特殊事件,event为特殊事件名如tauri://resize
      
      Ok(())
    })


    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
