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
