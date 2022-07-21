#![windows_subsystem = "windows"]

extern crate sciter;
extern crate windows;

use windows::{
  Win32::Foundation::HWND, Win32::UI::Input::KeyboardAndMouse::*, Win32::UI::WindowsAndMessaging::*,
};

fn open_calc() {
  let assets = include_bytes!("../target/assets.rc");

  // Enable debug mode for all windows, so that we can inspect them via Inspector.
  sciter::set_options(sciter::RuntimeOptions::DebugMode(true)).unwrap();

  let mut frame = sciter::window::Builder::main_window()
    .with_size((800, 80))
    .with_pos((300, 300))
    // .alpha()
    .glassy()
    .create();

  frame
    .set_options(sciter::window::Options::TransparentWindow(true))
    .unwrap();
  frame
    .archive_handler(assets)
    .expect("Unable to load archive");

  frame.load_file("this://app/main.htm");
  frame.expand(false);
  frame.run_app();
  // let mut window = frame.get_hwnd();
}

fn main() -> windows::core::Result<()> {
  println!("Hello world!");

  unsafe {
    RegisterHotKey(
      HWND(0),
      1,
      MOD_WIN | MOD_NOREPEAT,
      0xC0, /* the `~ key*/
    );

    let mut msg = MSG::default();
    while GetMessageA(&mut msg, windows::Win32::Foundation::HWND(0), 0, 0).into() {
      if msg.message == WM_HOTKEY {
        println!("TEST");
        open_calc();
        break;
      }
    }
  }

  return Ok(());
}
