//! Types related to task management

use super::TaskContext;
use crate::syscall::SYSCALL_TYPE_NUM;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    count: [SyscallCount; SYSCALL_TYPE_NUM],
}

impl TaskControlBlock {
    /// new for struct TaskControlBlock
    pub fn new() -> TaskControlBlock {
        return TaskControlBlock {
            task_cx: TaskContext::zero_init(),
            task_status: TaskStatus::UnInit,
            count: [SyscallCount {syscall: None, call_times: 0}; SYSCALL_TYPE_NUM],
        };
    }

    fn set_status(&mut self, status: TaskStatus) {
        self.task_status = status;
    }

    /// set status ready
    pub fn set_ready(&mut self) {
        self.set_status(TaskStatus::Ready);
    }

    /// record syscall times
    pub fn record_syscall(&mut self, syscall_id: usize) {
        for i in 0..SYSCALL_TYPE_NUM {
            if let Some(syscall) = self.count[i].syscall {
                if syscall == syscall_id {
                    self.count[i].call_times += 1;
                    return;
                }
            }
        }

        for i in 0..SYSCALL_TYPE_NUM {
            if self.count[i].syscall.is_none() {
                self.count[i].call_times = 1;
                self.count[i].syscall = Some(syscall_id);
                return;
            }
        }
    }

    /// get syscall times
    pub fn get_syscall_times(&mut self, syscall_id: usize) -> usize {
        for i in 0..SYSCALL_TYPE_NUM {
            if let Some(syscall) = self.count[i].syscall {
                if syscall == syscall_id {
                    return self.count[i].call_times;
                }
            }
        }

        return 0;
    }
}

#[derive(Copy, Clone)]
struct SyscallCount {
    syscall: Option<usize>,
    call_times: usize,
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
