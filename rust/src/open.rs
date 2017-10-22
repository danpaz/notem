use std::io;
use std::process::{Command, ExitStatus};
use std::ffi::OsStr;

#[cfg(target_os = "windows")]
pub fn that<T:AsRef<OsStr>+Sized>(program: T, path: T) -> io::Result<ExitStatus> {
    let mut cmd = Command::new("cmd");
    cmd.arg("/C").arg("start").arg("");
    if let Some(s) = path.as_ref().to_str() {
        cmd.arg(s.replace("&", "^&"));
    } else {
        cmd.arg(path.as_ref());
    }
    try!(cmd.spawn()).wait()
}

#[cfg(not(any(target_os = "windows")))]
pub fn that<T:AsRef<OsStr>+Sized>(program: T, path: T) -> io::Result<ExitStatus> {
    try!(Command::new(program).arg(path.as_ref()).spawn()).wait()
}
