//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, TASK_MANAGER},
    timer::{get_time_us, get_time_ms},
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
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(_ti: *mut TaskInfo, manager: &TASK_MANAGER) -> isize {
    trace!("kernel: sys_task_info");
    TaskInfo::reveal(_ti, manager);
    0
}

/// NOTE: TaskInfo revealing
impl TaskInfo {
    fn reveal(task_info: *mut TaskInfo, manager: &TASK_MANAGER) {
        let mut inner = manager.inner.exclusive_access();
        let current_task = inner.current_task;
        let current_task_controller = &mut inner.tasks[current_task];

        let current_time = get_time_ms();

        unsafe {
            (*task_info).time = current_time - current_task_controller.start_time;

            (*task_info).syscall_times[64]  = current_task_controller.syscall_count[0];
            (*task_info).syscall_times[93]  = current_task_controller.syscall_count[1];
            (*task_info).syscall_times[124] = current_task_controller.syscall_count[2];
            (*task_info).syscall_times[169] = current_task_controller.syscall_count[3];
            (*task_info).syscall_times[410] = current_task_controller.syscall_count[4];

            (*task_info).status = TaskStatus::Running;
        }

        drop(inner);
    }
}
