use std::{sync::Mutex, thread::{JoinHandle}};

static mut LOG_THREAD: Option<JoinHandle<fn(u64)>> = None;

static mut LOG_THREAD_REQUESTS: Mutex<Vec<Box<dyn Fn()>>> = Mutex::new(Vec::new());

/// Launch a thread that will handle logging operations.
/// Allow to log messages asynchronously, which can be useful for performance in applications with high logging demands.
/// 
/// Recomended when using log server: https://github.com/eVisualUser/log-server
/// # Parameters
/// 
/// -'idle_iteration_cooldown_in_milli' - The cooldown time in milliseconds when the thread is idle (not processing requests).
/// 
/// -'working_iteration_cooldown_in_milli' - The cooldown time in milliseconds when the thread is processing requests.
pub fn start_log_thread(idle_iteration_cooldown_in_milli: u64, working_iteration_cooldown_in_milli: u64) {
    unsafe {
        LOG_THREAD = Some(std::thread::spawn(move || {
            let mut was_working = false;
            loop {
                if !was_working {
                    // If the thread was not working, we wait for the cooldown before checking for requests
                    std::thread::sleep(std::time::Duration::from_millis(idle_iteration_cooldown_in_milli));
                } else {
                    // If the thread was working, we wait for the cooldown before checking for requests
                    std::thread::sleep(std::time::Duration::from_millis(working_iteration_cooldown_in_milli));
                }

                let mut requests = LOG_THREAD_REQUESTS.try_lock();
                while requests.is_err() {
                    std::thread::sleep(std::time::Duration::from_millis(10));
                    requests = LOG_THREAD_REQUESTS.try_lock();
                }

                if let Ok(mut reqs) = requests {
                    if !reqs.is_empty() {
                        let req = reqs.remove(0);
                        req();
                        was_working = true;
                    } else {
                        was_working = false;
                    }
                }
            }
        }));
    }
}

/// Launch a task in the logging thread if it is running.
/// If the logging thread is not running, the task will be executed immediately.
pub fn launch_task(task: Box<dyn Fn()>, iteration_cooldown_in_milli: u64)
{
    unsafe {
        if LOG_THREAD.is_some() {
            let mut requests = LOG_THREAD_REQUESTS.try_lock();
            while requests.is_err() {
                std::thread::sleep(std::time::Duration::from_millis(iteration_cooldown_in_milli));
                requests = LOG_THREAD_REQUESTS.try_lock();
            }

            if let Ok(mut reqs) = requests {
                reqs.push(task);
            }
        } else {
            // If the log thread is not running, execute the task immediately
            task();
        }
    }
}

pub fn log_thread_enabled() -> bool {
    unsafe { LOG_THREAD.is_some() }
}

/// Flush the log thread, ensuring that all pending tasks are processed.
pub fn flush_log_thread(iteration_cooldown_in_milli: u64) {
    unsafe {
        if LOG_THREAD.is_some() {
            let mut remaining_tasks = true;
            while remaining_tasks {
                let mut requests = LOG_THREAD_REQUESTS.try_lock();
                while requests.is_err() {
                    std::thread::sleep(std::time::Duration::from_millis(iteration_cooldown_in_milli));
                    requests = LOG_THREAD_REQUESTS.try_lock();
                }

                if let Ok(reqs) = requests {
                    if reqs.is_empty() {
                        remaining_tasks = false;
                    } else {
                        remaining_tasks = true;
                        std::thread::sleep(std::time::Duration::from_millis(iteration_cooldown_in_milli));
                    }
                }
            }
        }
    }
}