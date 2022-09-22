// threads2.rs
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a hint.
// Building on the last exercise, we want all of the threads to complete their work but this time
// the spawned threads need to be in charge of updating a shared value: JobStatus.jobs_completed

use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // Wrap the data inside the Arc with a Mutex to be able to lock it in a theard
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        // Grap a reference to the main thred status
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            // Lock the value at the reference, if the lock goes "boom" we panic, so there should
            // really be another way then unwrap
            let mut s = status_shared.lock().unwrap();
            // Increment the mutable reference with 1
            *s = JobStatus { jobs_completed: s.jobs_completed + 1 };
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice anything
        // interesting in the output? Do you have to 'join' on all the handles?
        println!("jobs completed {}", status.lock().unwrap().jobs_completed);
    }
}
