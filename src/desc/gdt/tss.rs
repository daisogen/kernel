const TSS_TYPE: u32 = 0x9;

bitfield! {
    pub struct RealTSSGDTEntry(u128);
    u32;

    get_lolimit, set_lolimit: 15, 0; // 104 (TSS_SIZE)
    get_lobase, set_lobase: 31, 16;
    get_midbase, set_midbase: 39, 32;
    get_type, set_type: 43, 40; // 0x9 for TSS (TSS_TYPE)
    get_mbz0, set_mbz0: 44;
    get_dpl, set_dpl: 46, 45;
    get_p, set_p: 47;
    get_hilimit, set_hilimit: 51, 48;
    get_avl, set_avl: 52; // Available (ignored)
    get_empty, set_empty: 54, 53;
    get_g, set_g: 55;
    get_hibase, set_hibase: 63, 56;
    get_rhibase, set_rhibase: 95, 64;
    get_mbz1, set_mbz1: 127, 96;
}

impl RealTSSGDTEntry {
    pub fn new() -> RealTSSGDTEntry {
        let mut ret = RealTSSGDTEntry(0);
        ret.set_lolimit(crate::desc::tss::TSS_SIZE as u32);
        // base pending
        ret.set_type(TSS_TYPE);
        ret.set_mbz0(false);
        ret.set_dpl(0);
        ret.set_p(true);
        ret.set_hilimit(0);
        ret.set_avl(false);
        ret.set_empty(0);
        ret.set_g(false);
        // base pending (again)
        ret.set_mbz1(0);

        ret
    }

    pub fn raw(&self) -> (u64, u64) {
        (self.0 as u64, (self.0 >> 64) as u64)
    }
}

pub struct TSSGDTEntry {
    pub ptr: u64,
}

impl TSSGDTEntry {
    pub fn real(&self) -> RealTSSGDTEntry {
        let mut ret = RealTSSGDTEntry::new();
        ret.set_lobase((self.ptr & 0xFFFF) as u32);
        ret.set_midbase(((self.ptr >> 16) & 0xFF) as u32);
        ret.set_hibase(((self.ptr >> 24) & 0xFF) as u32);
        ret.set_rhibase((self.ptr >> 32) as u32);
        ret
    }
}
