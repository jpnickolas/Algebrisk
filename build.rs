/*
extern crate winres;

fn main() {
  let mut res = winres::WindowsResource::new();
  // res.set_toolkit_path("C:\\Program Files (x86)\\Windows Kits\\10\\bin\\10.0.17763.0");
  // res.set_windres_path("C:\\msys64\\mingw64\\bin\\windres.exe");
  // res.set_ar_path("C:\\msys64\\mingw64\\bin\\ar.exe");
  res.set_icon("app.ico");
  res.compile().unwrap();
}*/
use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
  let out_dir = env::var("OUT_DIR").ok().expect("can't find out_dir");

  Command::new("windres")
    .args(&["algebrisk.rc", "-o"])
    .arg(&format!("{}/algebrisk.rc.o", out_dir))
    .status()
    .unwrap();
  Command::new("ar")
    .args(&["crus", "libalgebrisk_rc.a", "algebrisk.rc.o"])
    .current_dir(&Path::new(&out_dir))
    .status()
    .unwrap();

  println!("cargo:rustc-link-search=native={}", out_dir);
  println!("cargo:rustc-link-lib=static:+whole-archive=algebrisk_rc");
}
