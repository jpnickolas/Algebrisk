fn main() -> windows::core::Result<()> {
  println!("Hello world!");

  unsafe {
    RegisterHotKey(
      windows::Win32::Foundation::HWND(0),
      1,
      windows::Win32::UI::Input::KeyboardAndMouse::MOD_WIN
        | windows::Win32::UI::Input::KeyboardAndMouse::MOD_NOREPEAT,
      0xC0, /* the `~ key*/
    );

    let mut msg = windows::Win32::UI::WindowsAndMessaging::MSG::default();
    while windows::Win32::UI::WindowsAndMessaging::GetMessageA(
      &mut msg,
      windows::Win32::Foundation::HWND(0),
      0,
      0,
    )
    .into()
    {
      if msg.message == windows::Win32::UI::WindowsAndMessaging::WM_HOTKEY {
        println!("TEST");
        break;
      }
    }
  }

  return Ok(());
}
