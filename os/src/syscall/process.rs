//! Process management syscalls



use crate::{
    config::{MAX_SYSCALL_NUM, PAGE_SIZE}, mm::{translated_byte_buffer, MapPermission, VirtAddr}, task::{
        change_program_brk, current_task_memory_set, current_user_token, exit_current_and_run_next, get_current_task_info, get_current_task_start_time, suspend_current_and_run_next, TaskStatus
    }, timer::{get_time_ms, get_time_us}
};

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
            status: TaskStatus::Ready,
            syscall_times: [0; MAX_SYSCALL_NUM],
            time: 0,
        }
    }
    
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
    let time = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };

    // println!("kernel: sys_get_time: {:?}", time);
    let time: *mut u8 = &time as *const _ as *mut u8;

    let buffers = translated_byte_buffer(current_user_token(), ts as *mut u8, core::mem::size_of::<TimeVal>());
    for buffer in buffers {
        for i in 0..buffer.len() {
            buffer[i] = unsafe { *time.add(i) };
        }
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    let task_info = get_current_task_info();
    let mut task_info = task_info.exclusive_access();

    let cur_time = get_time_ms();
    let Some(start_time) = get_current_task_start_time() else {
        return -1;
    };

    task_info.time = cur_time - start_time;

    let task_info = &*task_info;

    let task_info: *mut u8 = task_info as *const _ as *mut u8;
    let buffers = translated_byte_buffer(current_user_token(), _ti as *mut u8, core::mem::size_of::<TaskInfo>());
    for buffer in buffers {
        for i in 0..buffer.len() {
            buffer[i] = unsafe { *task_info.add(i) };
        }
    }
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap");
    if (port & 0x7 == 0) || (port & !0x7 != 0) || (start & (PAGE_SIZE - 1) != 0){
        return -1;
    }
    let memset = current_task_memory_set();

    let mut map_permmision = MapPermission::U;
    if port & 0x1 != 0 {
        map_permmision.insert(MapPermission::R);
    }
    if port & 0x2 != 0 {
        map_permmision.insert(MapPermission::W);
    }
    if port & 0x4 != 0 {
        map_permmision.insert(MapPermission::X);
    }

    memset.try_insert_framed_area(VirtAddr::from(start), VirtAddr::from(start + len), map_permmision)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap");
    if start & (PAGE_SIZE - 1) != 0 {
        return -1;
    }
    let memset = current_task_memory_set();
    memset.try_remove_area(VirtAddr::from(start), VirtAddr::from(start + len))
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
