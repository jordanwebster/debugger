mod register;

use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Result;
use nix::sys::ptrace;
use nix::sys::signal::Signal;
use nix::sys::wait::waitpid;
use nix::sys::wait::WaitStatus;
use nix::unistd::Pid;
use rustyline;
use rustyline::error::ReadlineError;

use register::Register;
use register::RegisterReadWrite;

enum DebuggerCommand {
    Continue,
    Exit,
    ReadRegister(Register),
    WriteRegister(Register, u64),
}

pub struct Debugger {
    pid: Pid,
    io: rustyline::Editor<()>,
}

impl RegisterReadWrite for Debugger {}

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
                DebuggerCommand::ReadRegister(register) => {
                    match Self::read_register(self.pid, register) {
                        Ok(value) => println!("{}", value),
                        Err(error) => println!("Error: {}", error),
                    }
                }
                DebuggerCommand::WriteRegister(register, value) => {
                    match Self::write_register(self.pid, register, value) {
                        Ok(_) => println!("Register updated"),
                        Err(error) => println!("Error: {}", error),
                    }
                }
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
        } else if line.starts_with("register") {
            let usage_error = Err(anyhow!(
                "register read <register> OR register write <register> <value>)"
            ));
            let mut args = line.split(' ');
            let sub_cmd = args.nth(1);
            if sub_cmd == Some("read") {
                if let Some(reg_str) = args.nth(0) {
                    match Register::from_str(reg_str) {
                        Ok(register) => Ok(DebuggerCommand::ReadRegister(register)),
                        Err(_) => Err(anyhow!("Unknown register: {}", reg_str)),
                    }
                } else {
                    return usage_error;
                }
            } else if sub_cmd == Some("write") {
                let reg_arg = args.nth(0);
                let value_arg = args.nth(0);
                if let (Some(reg_str), Some(value_str)) = (reg_arg, value_arg) {
                    let value: u64 = match value_str.parse() {
                        Ok(value) => value,
                        Err(_) => {
                            return Err(anyhow!("{} is not a valid register value", value_str))
                        }
                    };

                    match Register::from_str(reg_str) {
                        Ok(register) => Ok(DebuggerCommand::WriteRegister(register, value)),
                        Err(_) => Err(anyhow!("Unknown register: {}", reg_str)),
                    }
                } else {
                    return usage_error;
                }
            } else {
                return usage_error;
            }
        } else {
            return Err(anyhow!("Unknown command: {}", line));
        }
    }
}
