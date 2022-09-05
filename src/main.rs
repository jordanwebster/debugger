use nix::sys::ptrace;
use nix::unistd::execv;
use nix::unistd::fork;
use nix::unistd::ForkResult;
use std::ffi::CStr;
use std::ffi::CString;

mod debugger;
use debugger::Debugger;

fn main() {
    let program = std::env::args()
        .nth(1)
        .expect("Need to provide the path to a binary to debug");

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            let mut debugger = match Debugger::new(child) {
                Ok(debugger) => debugger,
                Err(e) => {
                    println!("Failed to create debugger: {}", e);
                    return;
                }
            };

            match debugger.run() {
                Err(e) => println!("Encounted unexpected error: {}", e),
                _ => (),
            }
        }
        Ok(ForkResult::Child) => {
            let debuggee = CString::new(program.as_str()).unwrap();
            ptrace::traceme().unwrap();
            execv::<&CStr>(debuggee.as_c_str(), &[]).unwrap();
        }
        Err(_) => println!("Failed to execute {}", program),
    }
}
