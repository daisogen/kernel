mod default_isr;

use core::arch::global_asm;

const IDT_ENTRIES: usize = 256; // Fixed by arch

bitfield! {
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct RealGate(u128);
    u32;

    get_loaddr, set_loaddr: 15, 0;
    get_seg, set_seg: 31, 16;
    get_ist, set_ist: 34, 32;
    get_res0, set_res0: 39, 35;
    get_type, set_type: 43, 40;
    get_mbz, set_mbz: 44;
    get_dpl, set_dpl: 46, 45;
    get_p, set_p: 47;
    get_midaddr, set_midaddr: 63, 48;
    get_hiaddr, set_hiaddr: 95, 64;
    get_res1, set_res1: 127, 96;
}

impl RealGate {
    pub fn new() -> RealGate {
        let mut ret = RealGate(0);
        // Addr is pending
        // Segment selector pending
        // IST pending
        ret.set_res0(0);
        ret.set_type(0xE); // I mean I ain't using traps
        ret.set_mbz(false);
        // DPL is pending
        ret.set_p(true);
        // Addr is pending (again)
        ret.set_res1(0);
        ret
    }

    pub fn set_wholeaddr(&mut self, addr: u64) {
        self.set_loaddr((addr & 0xFFFF) as u32);
        self.set_midaddr(((addr >> 16) & 0xFFFF) as u32);
        self.set_hiaddr(((addr >> 32) & 0xFFFFFFFF) as u32);
    }

    pub fn raw(&self) -> u128 {
        self.0
    }
}

#[repr(transparent)]
struct IDTstruct {
    vec: [[u64; 2]; IDT_ENTRIES],
}

#[repr(C, packed)]
struct IDTRstruct {
    limit: u16,
    addr: *const IDTstruct,
}

static mut IDT: IDTstruct = IDTstruct {
    vec: [[0; 2]; IDT_ENTRIES], // u128 is no good
};

static mut IDTR: IDTRstruct = IDTRstruct {
    limit: (IDT_ENTRIES as u16 * 16) - 1,
    addr: unsafe { &IDT as *const IDTstruct },
};

struct Gate {
    addr: u64,
    seg: u32,
    ist: u32,
    dpl: u32,
}

impl Gate {
    pub fn real(&self) -> RealGate {
        let mut ret: RealGate = RealGate::new();
        ret.set_wholeaddr(self.addr);
        ret.set_seg(self.seg);
        ret.set_ist(self.ist);
        ret.set_dpl(self.dpl);
        ret
    }
}

global_asm!(include_str!("lidt.s"));
global_asm!(include_str!("isrs.s"));

extern "C" {
    fn switchIDT(idtr: *const IDTRstruct) -> u64;
    static ISRS: [u64; IDT_ENTRIES];
}

pub fn init() {
    // Put ISRs
    for i in 0..256 {
        let addr = unsafe { ISRS[i] };
        let gate = Gate {
            addr: addr,
            seg: 0x08, // Kernel code (TODO)
            ist: 0,
            dpl: 0,
        }
        .real()
        .raw();

        let msbb = (gate >> 64) as u64;
        let lsbb = (gate & ((1 << 64) - 1)) as u64;

        unsafe {
            IDT.vec[i][0] = lsbb;
            IDT.vec[i][1] = msbb;
        }
    }

    unsafe {
        switchIDT(&IDTR);
    }
}
