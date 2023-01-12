use crate::tasks;
use crate::tasks::task::Task;
use crate::{print, println};

pub fn run() -> ! {
    for i in &*crate::boot::b2k::MODULES.lock() {
        print!("{} ", i.name);

        // Load it
        let begin = i.begin + crate::mem::HH; // Mapped as "kernel and modules"
        let size = (i.end - i.begin) as usize;
        let pid = crate::tasks::loader::load(begin, size).unwrap();
        let task: &Task = tasks::get_task(pid);

        task.dispatch_saving();

        println!("[OK]");
    }
    println!("Bootstrapping finished");
    println!("Launching init");

    panic!("init returned");
}
