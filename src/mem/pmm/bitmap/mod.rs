// A somewhat slow but freestanding implementation of a variable-sized bitmap

pub struct Bitmap {
    pub ptr: *mut u8,
    pub sz: usize,
}

impl Bitmap {
    pub fn get(&self, n: usize) -> Result<bool, ()> {
        if n >= self.sz {
            return Err(());
        }

        let byte: u8 = unsafe { *(self.ptr.add(n / 8)) };
        let off = n % 8;
        Ok(byte & (1 << off) != 0)
    }

    pub fn set(&self, n: usize, v: bool) -> Result<(), ()> {
        if n >= self.sz {
            return Err(());
        }

        let byte: &mut u8 = unsafe { &mut *(self.ptr.add(n / 8)) };
        let off = n % 8;
        if v {
            *byte |= 1 << off;
        } else {
            *byte &= !((1 << off) as u8);
        }

        Ok(())
    }
}
