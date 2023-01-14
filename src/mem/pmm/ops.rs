use super::bitmap::Bitmap;
use super::Frame;
use crate::mem::PAGE_SIZE;

pub fn alloc(npages: usize) -> Result<u64, ()> {
    let state = super::STATE.lock();

    let mut ptr = state.regions.0;
    for _ in 0..state.nregions {
        let frame: &Frame = unsafe { &**ptr };

        // Working with a region
        let got = alloc_in_region(frame, npages);
        if got.is_ok() {
            // Got it
            return Ok(frame.first + (got.unwrap() * PAGE_SIZE) as u64);
        }

        ptr = unsafe { ptr.add(1) };
    }

    Err(()) // OOM :(
}

fn alloc_in_region(frame: &Frame, want: usize) -> Result<usize, ()> {
    // Are there "want" consecutive free pages in "frame"?
    // Let's start checking the bitmap
    let pages = frame.pages;
    let mut have = 0;

    let ptr = frame as *const Frame as u64;
    let ptr = ptr + core::mem::size_of::<Frame>() as u64;
    let ptr = ptr as *mut u8;
    let bm: Bitmap = Bitmap {
        ptr: ptr,
        sz: pages,
    };

    // Let's start looking
    let mut cur = 0;
    let mut ret = 0;
    while cur < pages {
        if !bm.get(cur).unwrap() {
            have += 1; // Nice
        } else {
            ret = cur + 1; // Let's try again
            have = 0;
        }

        if have == want {
            // Nice! Set the bits
            for i in 0..want {
                bm.set(ret + i, true).unwrap();
            }
            return Ok(ret);
        }

        cur += 1;
    }

    // Tough luck
    Err(())
}

pub fn calloc(npages: usize) -> Result<u64, ()> {
    let ret = alloc(npages);
    if ret.is_err() {
        return Err(());
    }

    let ret = ret.unwrap();
    unsafe {
        compiler_builtins::mem::memset(ret as *mut u8, 0, PAGE_SIZE * npages);
    }
    Ok(ret)
}

pub fn free(_ptr: u64, _npages: usize) {
    // https://github.com/the-strife-project/kernel/blob/c6be30bff6c9748da499ad92e2a577058665da57/src/mem/PMM/ops.cpp#L67
    todo!("I'm lazy rn");
}
