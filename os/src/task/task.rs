//! Types related to task management

use alloc::vec::Vec;
use super::TaskContext;

/// The task control block (TCB) of a task.
#[derive(Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// record all syscall
    pub syscall_counts: Vec<SyscallRecord>,
    /// the time when this task was first run
    pub start_time_ms: Option<usize>,
}

impl TaskControlBlock {

    /// increment syscall count by 1
    pub fn add_syscall_count(&mut self, syscall_id: usize) {
        if let Some(record) = self.syscall_counts.iter_mut().find(|record| record.syscall_id == syscall_id) {
            record.count += 1;
            return;
        }
        self.syscall_counts.push(SyscallRecord{ syscall_id, count: 1 });
    }

    // 当前分时多任务系统是将用户程序统一链接到整个内核二进制文件中的，暂时不用真的重置状态
    /// clear all syscall records
    pub fn reset_syscall_counts(&mut self) {
        self.syscall_counts.clear();
    }

    /// set task start time if it's empty
    pub fn set_start_time(&mut self, start_time: usize) {
        if self.start_time_ms.is_none() {
            self.start_time_ms = Some(start_time);
        }
    }

    /// get current syscall count
    pub fn get_syscall_count(&self, syscall_id: usize) -> u32 {
        self.syscall_counts.iter().find(|record| record.syscall_id == syscall_id)
            .map_or(0, |record| record.count)
    }
}

impl Default for TaskControlBlock {
    fn default() -> Self {
        TaskControlBlock {
            task_cx: TaskContext::zero_init(),
            task_status: TaskStatus::UnInit,
            syscall_counts: Vec::new(),
            start_time_ms: None,
        }
    }
}

#[derive(Copy, Clone)]
pub struct SyscallRecord {
    pub syscall_id: usize,
    pub count: u32,
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
