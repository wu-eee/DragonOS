use crate::arch::interrupt::TrapFrame;
use crate::arch::syscall::nr::SYS_EXIT_GROUP;
use crate::process::ProcessManager;
use crate::syscall::table::{FormattedSyscallParam, Syscall};
use alloc::vec::Vec;
use system_error::SystemError;

pub struct SysExitGroup;

impl SysExitGroup {
    fn exit_code(args: &[usize]) -> usize {
        args[0]
    }
}

impl Syscall for SysExitGroup {
    fn num_args(&self) -> usize {
        1
    }

    fn handle(&self, args: &[usize], _frame: &mut TrapFrame) -> Result<usize, SystemError> {
        let exit_code = Self::exit_code(args);
        ProcessManager::exit((exit_code & 0xff) << 8);
    }

    fn entry_format(&self, args: &[usize]) -> Vec<FormattedSyscallParam> {
        vec![FormattedSyscallParam::new(
            "exit_code",
            format!("{:#x}", Self::exit_code(args)),
        )]
    }
}

syscall_table_macros::declare_syscall!(SYS_EXIT_GROUP, SysExitGroup);
