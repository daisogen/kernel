bitfield! {
    pub struct RealCodeGDTEntry(u64);
    u32;

    get_lolimit, set_lolimit: 15, 0; // Ignored
    get_lobase, set_lobase: 31, 16; // Ignored
    get_midbase, set_midbase: 39, 32; // Ignored
    get_ign0, set_ign0: 41, 40; // Readable and Accessed attributes (ignored)
    get_conform, set_conform: 42; // Conforming bit
    get_mbo0, set_mbo0: 43;
    get_mbo1, set_mbo1: 44;
    get_dpl, set_dpl: 46, 45; // DPL
    get_p, set_p: 47; // Present (MBO)
    get_hilimit, set_hilimit: 51, 48; // Ignored
    get_avl, set_avl: 52; // Available (ignored)
    get_long, set_long: 53; // MBO in this case
    get_d, set_d: 54; // D bit, MBZ if long=1
    get_g, set_g: 55; // Granularity, ignored
    get_hibase, set_hibase: 63, 56; // Ignored
}

impl RealCodeGDTEntry {
    pub fn new() -> RealCodeGDTEntry {
        let mut ret = RealCodeGDTEntry(0);
        ret.set_lolimit(0);
        ret.set_lobase(0);
        ret.set_midbase(0);
        ret.set_ign0(0);
        // Conforming bit pending
        ret.set_mbo0(true);
        ret.set_mbo1(true);
        // DPL pending
        ret.set_p(true);
        ret.set_hilimit(0);
        // AVL pending
        ret.set_long(true);
        ret.set_d(false);
        ret.set_g(false);
        ret.set_hibase(0);
        ret
    }

    pub fn raw(&self) -> u64 {
        self.0
    }
}

// Actually usable
pub struct CodeGDTEntry {
    pub conforming: bool,
    pub dpl: u32,
    pub avl: bool,
}

impl CodeGDTEntry {
    pub fn real(&self) -> RealCodeGDTEntry {
        let mut ret = RealCodeGDTEntry::new();
        ret.set_conform(self.conforming);
        ret.set_dpl(self.dpl);
        ret.set_avl(self.avl);
        ret
    }
}
