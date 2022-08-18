#![windows_subsystem = "windows"]

extern crate dark_light;
extern crate directories;
extern crate kalk;
extern crate msw_hotkey;
extern crate sciter;
extern crate windows;

use msw_hotkey::Hotkey;
use windows::{
  Win32::Foundation::HWND, Win32::UI::Input::KeyboardAndMouse::*, Win32::UI::WindowsAndMessaging::*,
};

static mut LAST_KEYBOARD_SHORTCUT_ID: i32 = 0;

struct Handler {}

impl Handler {
  fn quit(&self) {
    unsafe { PostQuitMessage(0) };
  }

  fn eval(&self, expr: String) -> sciter::Value {
    let mut ctx = kalk::parser::Context::new();
    let res = kalk::parser::eval(&mut ctx, &expr);
    if res.is_err() {
      return sciter::Value::null();
    }

    let res2 = res.unwrap();
    if res2.is_none() {
      return sciter::Value::null();
    }
    return sciter::Value::from(res2.unwrap().to_js_string());
  }

  fn get_system_theme(&self) -> sciter::Value {
    let mode = dark_light::detect();

    match mode {
      dark_light::Mode::Dark => {
        return sciter::Value::from("dark");
      }
      dark_light::Mode::Light => {
        return sciter::Value::from("light");
      }
    }
  }

  fn register_shortcut(&self, shortcut: String) -> sciter::Value {
    let result = shortcut.parse::<Hotkey>();
    if result.is_err() {
      return sciter::Value::from(result.unwrap_err().to_string());
    }
    let hotkey = result.unwrap();

    unsafe {
      let keyboard_shortcut_id = LAST_KEYBOARD_SHORTCUT_ID + 1;
      let result = RegisterHotKey(
        HWND(0),
        keyboard_shortcut_id,
        HOT_KEY_MODIFIERS(hotkey.modifier() as u32) | MOD_NOREPEAT,
        VkKeyScanA(windows::Win32::Foundation::CHAR(hotkey.key())) as u32,
      );
      if !result.as_bool() {
        return sciter::Value::from("Could not register hotkey. Please try again.");
      }

      if LAST_KEYBOARD_SHORTCUT_ID > 0 {
        UnregisterHotKey(HWND(0), LAST_KEYBOARD_SHORTCUT_ID);
      }
      LAST_KEYBOARD_SHORTCUT_ID = keyboard_shortcut_id;
    }
    return sciter::Value::from(true);
  }
}

impl sciter::EventHandler for Handler {
  sciter::dispatch_script_call! {
    fn quit();
    fn eval(str);
    fn get_system_theme();
    fn register_shortcut(str);
  }
}

fn open_calc() {
  let assets = include_bytes!("../target/assets.rc");

  // Enable debug mode for all windows, so that we can inspect them via Inspector.
  // sciter::set_options(sciter::RuntimeOptions::DebugMode(true)).unwrap();
  sciter::set_options(sciter::RuntimeOptions::ScriptFeatures(
    sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_FILE_IO as u8
      | sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SYSINFO as u8,
  ))
  .unwrap();

  let mut frame = sciter::window::Builder::tool()
    .with_size((800, 350))
    .create();

  frame
    .set_options(sciter::window::Options::TransparentWindow(true))
    .unwrap();

  frame
    .archive_handler(assets)
    .expect("Unable to load archive");
  frame.event_handler(Handler {});

  frame.load_file("this://app/main.htm");
  frame.collapse(true);

  unsafe {
    let mut msg = MSG::default();
    while GetMessageA(&mut msg, HWND(0), 0, 0).into() {
      if msg.message == WM_HOTKEY {
        frame.expand(false);
      }
      if msg.message == WM_DWMCOLORIZATIONCOLORCHANGED {
        frame
          .get_host()
          .call_function("applyTheme", &sciter::make_args!())
          .unwrap_or_default();
      }
      TranslateMessage(&mut msg);
      DispatchMessageW(&mut msg);
    }
  }
}

fn init_working_dir() {
  let project = directories::ProjectDirs::from("", "", "Algebrisk").unwrap();
  let working_dir = project.config_dir();
  if !working_dir.is_dir() {
    std::fs::create_dir_all(working_dir).unwrap();
  }
  std::env::set_current_dir(working_dir).unwrap();
}

fn is_already_running() -> bool {
  // Just look for the settings window of another running algebrisk exe. There
  // is a brief window where two executables can be launched at the same time
  // before the settings window is created, but having two launched isn't that
  // big of a deal.
  unsafe {
    let existing_app_settings_window = FindWindowA(
      windows::core::PCSTR(std::ptr::null()),
      windows::s!("Algebrisk: Settings"),
    );
    if existing_app_settings_window.0 != 0 {
      ShowWindow(existing_app_settings_window, SW_SHOW);
      SetForegroundWindow(existing_app_settings_window);
      return true;
    }
    return false;
  }
}

fn main() -> windows::core::Result<()> {
  if is_already_running() {
    return Ok(());
  }
  init_working_dir();
  open_calc();
  return Ok(());
}
