//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM, syscall::add_syscall_time, task::{exit_current_and_run_next, get_running_task_info, get_running_task_start_time, suspend_current_and_run_next, TaskStatus}, timer::{get_time_ms, get_time_us}
};

use super::{SYSCALL_EXIT, SYSCALL_TASK_INFO, SYSCALL_YIELD, SYSCALL_GET_TIME};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
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

impl TaskInfo {
    /// Create a new TaskInfo
    pub fn new() -> Self {
        TaskInfo {
            status: TaskStatus::UnInit,
            syscall_times: [0; MAX_SYSCALL_NUM],
            time: 0,
        }
    }
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);

    add_syscall_time(SYSCALL_EXIT);

    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");

    add_syscall_time(SYSCALL_YIELD);

    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");

    add_syscall_time(SYSCALL_GET_TIME);

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
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    // 取得当前任务的开始时间
    let start_time = get_running_task_start_time();

    add_syscall_time(SYSCALL_TASK_INFO);

    let task_info = get_running_task_info();
    let mut task_info = task_info.exclusive_access();
    let cur_time = get_time_ms();
    task_info.time = cur_time - start_time;
    unsafe {
        *ti = TaskInfo {
            status: task_info.status,
            syscall_times: task_info.syscall_times,
            time: task_info.time,
        };
    }
    0
}
