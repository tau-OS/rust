// Generated by gir (https://github.com/gtk-rs/gir @ 68044d02c2a3)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 5245759fdac8)
// DO NOT EDIT

#[cfg(not(docsrs))]
use std::process;

#[cfg(docsrs)]
fn main() {} // prevent linking libraries to avoid documentation failure

#[cfg(not(docsrs))]
fn main() {
    if let Err(s) = system_deps::Config::new().probe() {
        println!("cargo:warning={s}");
        process::exit(1);
    }
}
