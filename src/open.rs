use std::ffi::OsStr;
use std::io;
use std::process::{Command, ExitStatus};

#[cfg(target_os = "windows")]
pub fn that<T: AsRef<OsStr> + Sized>(program: T, path: T) -> io::Result<ExitStatus> {
    let mut cmd = Command::new("cmd");
    cmd.arg("/C").arg("start").arg("");
    if let Some(s) = path.as_ref().to_str() {
        cmd.arg(s.replace("&", "^&"));
    } else {
        cmd.arg(path.as_ref());
    }
    cmd.spawn()?.wait()
}

#[cfg(not(any(target_os = "windows")))]
pub fn that<T: AsRef<OsStr> + Sized>(program: T, path: T) -> io::Result<ExitStatus> {
    Command::new(program).arg(path.as_ref()).spawn()?.wait()
}
