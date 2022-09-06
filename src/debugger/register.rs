use anyhow::Result;
use nix::sys::ptrace;
use nix::unistd::Pid;
use strum::EnumString;

#[allow(non_camel_case_types)]
#[derive(EnumString)]
pub enum Register {
    rax,
    rbx,
    rcx,
    rdx,
    rdi,
    rsi,
    rbp,
    rsp,
    r8,
    r9,
    r10,
    r11,
    r12,
    r13,
    r14,
    r15,
    rip,
    rflags,
    cs,
    orig_rax,
    fs_base,
    gs_base,
    fs,
    gs,
    ss,
    ds,
    es,
}

pub trait RegisterReadWrite {
    fn read_register(pid: Pid, register: Register) -> Result<u64> {
        let registers = ptrace::getregs(pid)?;
        Ok(match register {
            Register::rax => registers.rax,
            Register::rbx => registers.rbx,
            Register::rcx => registers.rcx,
            Register::rdx => registers.rdx,
            Register::rdi => registers.rdi,
            Register::rsi => registers.rsi,
            Register::rbp => registers.rbp,
            Register::rsp => registers.rsp,
            Register::r8 => registers.r8,
            Register::r9 => registers.r9,
            Register::r10 => registers.r10,
            Register::r11 => registers.r11,
            Register::r12 => registers.r12,
            Register::r13 => registers.r13,
            Register::r14 => registers.r14,
            Register::r15 => registers.r15,
            Register::rip => registers.rip,
            Register::rflags => registers.eflags,
            Register::cs => registers.cs,
            Register::orig_rax => registers.orig_rax,
            Register::fs_base => registers.fs_base,
            Register::gs_base => registers.gs_base,
            Register::fs => registers.fs,
            Register::gs => registers.gs,
            Register::ss => registers.ss,
            Register::ds => registers.ds,
            Register::es => registers.es,
        })
    }

    fn write_register(pid: Pid, register: Register, value: u64) -> Result<()> {
        let mut registers = ptrace::getregs(pid)?;
        match register {
            Register::rax => registers.rax = value,
            Register::rbx => registers.rbx = value,
            Register::rcx => registers.rcx = value,
            Register::rdx => registers.rdx = value,
            Register::rdi => registers.rdi = value,
            Register::rsi => registers.rsi = value,
            Register::rbp => registers.rbp = value,
            Register::rsp => registers.rsp = value,
            Register::r8 => registers.r8 = value,
            Register::r9 => registers.r9 = value,
            Register::r10 => registers.r10 = value,
            Register::r11 => registers.r11 = value,
            Register::r12 => registers.r12 = value,
            Register::r13 => registers.r13 = value,
            Register::r14 => registers.r14 = value,
            Register::r15 => registers.r15 = value,
            Register::rip => registers.rip = value,
            Register::rflags => registers.eflags = value,
            Register::cs => registers.cs = value,
            Register::orig_rax => registers.orig_rax = value,
            Register::fs_base => registers.fs_base = value,
            Register::gs_base => registers.gs_base = value,
            Register::fs => registers.fs = value,
            Register::gs => registers.gs = value,
            Register::ss => registers.ss = value,
            Register::ds => registers.ds = value,
            Register::es => registers.es = value,
        }

        ptrace::setregs(pid, registers)?;
        Ok(())
    }
}
