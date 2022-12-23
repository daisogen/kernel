bitfield! {
    pub struct RealDataGDTEntry(u64);
    u32;

    get_lolimit, set_lolimit: 15, 0; // Ignored
    get_lobase, set_lobase: 31, 16; // Should probably be zero (ignored sometimes)
    get_midbase, set_midbase: 39, 32; // Probably zero
    get_a, set_a: 40; // Ignored
    get_w, set_w: 41;
    get_e, set_e: 42; // Ignored
    get_mbz0, set_mbz0: 43;
    get_mbo0, set_mbo0: 44;
    get_dpl, set_dpl: 46, 45; // Ignored
    get_p, set_p: 47; // Present (MBO)
    get_hilimit, set_hilimit: 51, 48; // Ignored
    get_avl, set_avl: 52; // Available (ignored)
    get_wtf, set_wtf: 53; // Ignored
    get_db, set_db: 54; // Ignored
    get_g, set_g: 55; // Ignored
    get_hibase, set_hibase: 63, 56; // Probably zero
}

impl RealDataGDTEntry {
    pub fn new() -> RealDataGDTEntry {
        let mut ret = RealDataGDTEntry(0);
        ret.set_lolimit(0);
        ret.set_lobase(0);
        ret.set_midbase(0);
        ret.set_a(false);

        /*
        Intel says only writable segments can be loaded in SS.
        AMD does not mention that anywhere. It says the field is ignored.
        Setting W=1 works. I've been stuck plenty of hours because of this.
        I hate computers.
        */
        ret.set_w(true);

        ret.set_e(false);
        ret.set_mbz0(false);
        ret.set_mbo0(true);
        ret.set_dpl(0);
        ret.set_p(true);
        ret.set_hilimit(0);
        ret.set_avl(false);
        ret.set_wtf(false);
        ret.set_db(false);
        ret.set_g(false);
        ret.set_hibase(0);
        ret
    }

    pub fn raw(&self) -> u64 {
        self.0
    }
}

pub struct DataGDTEntry {}
impl DataGDTEntry {
    pub fn real(&self) -> RealDataGDTEntry {
        RealDataGDTEntry::new()
    }
}
