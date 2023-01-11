use crate::{print, println};

pub fn run() -> ! {
    for i in &*crate::boot::b2k::MODULES.lock() {
        print!("{} ", i.name);

        // Load it
        let begin = i.begin + crate::mem::HH; // Mapped as "kernel and modules"
        let size = (i.end - i.begin) as usize;
        let _pid = crate::tasks::loader::load(begin, size).unwrap();

        println!("[OK]");
    }
    println!("Bootstrapping finished");
    println!("Launching init");

    panic!("init returned");
}
