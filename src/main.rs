// #![windows_subsystem = "windows"]

extern crate sciter;
extern crate windows;

use windows::{
  Win32::Foundation::HWND, Win32::UI::Input::KeyboardAndMouse::*, Win32::UI::WindowsAndMessaging::*,
};

struct Handler {}

impl Handler {
  fn quit(&self) {
    unsafe { PostQuitMessage(0) };
  }
}

impl sciter::EventHandler for Handler {
  sciter::dispatch_script_call! {
    fn quit();
  }
}

fn open_calc() {
  let assets = include_bytes!("../target/assets.rc");

  // Enable debug mode for all windows, so that we can inspect them via Inspector.
  sciter::set_options(sciter::RuntimeOptions::DebugMode(true)).unwrap();

  let mut frame = sciter::window::Builder::tool()
    .with_size((800, 200))
    .with_pos((300, 300))
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
  // frame.run_app();
  // let mut window = frame.get_hwnd();

  unsafe {
    println!("Waiting for WIN+`");
    let mut msg = MSG::default();
    while GetMessageA(&mut msg, windows::Win32::Foundation::HWND(0), 0, 0).into() {
      if msg.message == WM_HOTKEY {
        frame.expand(false);
        // SetFocus(frame.get_hwnd());
      }
      TranslateMessage(&mut msg);
      DispatchMessageW(&mut msg);
    }
  }
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
  }

  open_calc();

  return Ok(());
}
