use nix::sched::{clone, CloneFlags};
use nix::sys::wait::waitpid;
use nix::sys::signal::Signal;

//since child_pid dont have 
const STACK_SIZE: usize = 1024 * 1024;

fn child_fn() -> isize {
    println!("inside namespace, PID = {}", std::process::id());
    0
}

fn main(){
    let mut stack = vec![0u8; STACK_SIZE];

    let flags = CloneFlags::CLONE_NEWPID
        | CloneFlags::CLONE_NEWUTS
        | CloneFlags::CLONE_NEWNS;

    let child_pid = unsafe {
        clone(
            Box::new(child_fn),
            &mut stack,
            flags,
            Some(Signal::SIGCHLD as i32),
        ).expect("Clone() Failed")
    };

    println!("parent: child is running as PID {}", child_pid);
    waitpid(child_pid, None).expect("waitpid failed");
    println!("parent: child exited");
}