//! Process management syscalls
use crate::{
    config::{MAX_SYSCALL_NUM, PAGE_SIZE},
    task::{ change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, current_user_token, allocate_mem, deallocate_mem},
    timer::get_time_us,
    mm::{VirtAddr, PhysAddr, PageTable, VirtPageNum, translated_byte_buffer},
};

/// TimerVal
#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    /// Sec precision
    pub sec: usize,
    /// USec precision
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    pub status: TaskStatus,
    /// The numbers of syscall called by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    pub time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();

    unsafe {
        let ptr = translated_byte_buffer(
            current_user_token(),
            ts as *const u8,
            core::mem::size_of::<TimeVal>())[0].as_ptr() as *mut u8 as *mut TimeVal;

        let time_val = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };

        *ptr = time_val;
    }

    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
/// YOUR JOB: Finish sys_task_info to pass testcases

pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");

    let page_offset = VirtAddr::from(ti as usize).page_offset();

    let ppn = PhysAddr::from(
        PageTable::from_token(
            current_user_token()
        )
            .translate(
                VirtAddr::from(ti as usize)
                    .floor()
            )
            .unwrap()
            .ppn()
    );

    let kernel_task_info = (page_offset + usize::from(ppn)) as *mut TaskInfo;

    TaskInfo::reveal(kernel_task_info);

    0
}

/// sys_mmap
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap");

    if start % PAGE_SIZE != 0 {
        return -1;
    }
    if port & !0x7 != 0 {
        return -1;
    }
    if port & 0x7 == 0 {
        return -1;
    }
    if len == 0 {
        return 0;
    }

    allocate_mem(
        VirtPageNum::from(VirtAddr::from(start)),
        VirtPageNum::from(VirtAddr::from(start + len).ceil()),
        port,
    )
}

/// sys_munmap
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap");

    if start % PAGE_SIZE != 0 {
        return -1;
    }
    if len == 0 {
        return 0;
    }

    deallocate_mem(
        VirtPageNum::from(VirtAddr::from(start)),
        VirtPageNum::from(VirtAddr::from(start + len).ceil()),
    )
}

/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
