use crate::tasks::scheduler;

pub fn run() -> ! {
    for i in &*crate::boot::b2k::MODULES.lock() {
        crate::print!("{} ", i.name);

        // Load it
        let begin = i.begin + crate::mem::HH; // Mapped as "kernel and modules"
        let size = (i.end - i.begin) as usize;
        let pid = crate::tasks::loader::load(begin, size).unwrap();

        // Run it
        scheduler::add(pid);
        scheduler::schedule_saving();
        crate::println!("[OK]");
    }

    // Bootstrapping finished
    crate::println!("Have fun!");
    scheduler::schedule();
}
