use anyhow::anyhow;
use anyhow::Result;
use nix::sys::ptrace;
use nix::sys::signal::Signal;
use nix::sys::wait::waitpid;
use nix::sys::wait::WaitStatus;
use nix::unistd::Pid;
use rustyline;
use rustyline::error::ReadlineError;

enum DebuggerCommand {
    Continue,
    Exit,
}

pub struct Debugger {
    pid: Pid,
    io: rustyline::Editor<()>,
}

impl Debugger {
    pub fn new(pid: Pid) -> Result<Self> {
        Ok(Debugger {
            pid,
            io: rustyline::Editor::<()>::new()?,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        match waitpid(self.pid, None)? {
            WaitStatus::Stopped(pid, signal) if signal == Signal::SIGTRAP => {
                println!("Debugger attached to {}", pid)
            }
            event => return Err(anyhow!("Unexpected event: {:?}", event)),
        }

        loop {
            match self.get_next_command()? {
                DebuggerCommand::Continue => {
                    ptrace::cont(self.pid, None)?;
                    match waitpid(self.pid, None)? {
                        WaitStatus::Exited(_, _) => {
                            println!("Debuggee exited");
                            return Ok(());
                        }
                        event => println!("Received event: {:?}", event),
                    }
                }
                DebuggerCommand::Exit => return Ok(()),
            }
        }
    }

    fn get_next_command(&mut self) -> Result<DebuggerCommand> {
        loop {
            match self.io.readline("debugger> ") {
                Ok(line) => match Debugger::parse_command(&line) {
                    Ok(cmd) => {
                        self.io.add_history_entry(line);
                        return Ok(cmd);
                    }
                    Err(e) => println!("{}", e),
                },
                Err(ReadlineError::Interrupted) => return Ok(DebuggerCommand::Exit),
                Err(ReadlineError::Eof) => return Ok(DebuggerCommand::Exit),
                Err(e) => return Err(anyhow!(e)),
            }
        }
    }

    fn parse_command(line: &str) -> Result<DebuggerCommand> {
        if "continue".starts_with(line) {
            return Ok(DebuggerCommand::Continue);
        } else {
            return Err(anyhow!("Unknown command: {}", line));
        }
    }
}
