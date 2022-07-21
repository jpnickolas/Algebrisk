use windows::{
  Win32::Foundation::HWND, Win32::UI::Input::KeyboardAndMouse::*, Win32::UI::WindowsAndMessaging::*,
};

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
        break;
      }
    }
  }

  return Ok(());
}
