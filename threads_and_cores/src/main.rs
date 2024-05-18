use libc;
use std::io;

fn current_cpu() -> Result<usize, io::Error> {
    let ret = unsafe { libc::sched_getcpu() };

    if ret < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(ret as usize)
    }
}

fn main() {
    use std::thread;

    let other_thread = thread::spawn(|| {
        println!(
            "I'm thread {:?} on cpu {:?}",
            thread::current().id(),
            current_cpu()
        );
    });

    println!(
        "I'm thread {:?} on cpu {:?}",
        thread::current().id(),
        current_cpu()
    );
    other_thread.join().unwrap();
}
