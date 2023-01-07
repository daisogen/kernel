use crate::mem::PAGE_SIZE;

// Entries per page. All are 64 bits.
pub const EPP: usize = PAGE_SIZE / (64 / 8);

// AMD64 Architecture Programmer's Manual, Volume 2: System Programming,
//   5.3 Long-Mode Page Translation

bitfield! {
    pub struct PML4E(u64);
    u64;

    pub get_present, set_present: 0;
    pub get_rw, set_rw: 1;
    pub get_user, set_user: 2;
    pub get_pwt, set_pwt: 3;
    pub get_pcd, set_pcd: 4;
    pub get_a, set_a: 5; // Accessed
    pub get_ign0, set_ign0: 6;
    pub get_mbz, set_mbz: 8, 7;
    pub get_avl0, set_avl0: 11, 9;
    pub get_ptr, set_ptr: 51, 12;
    pub get_avl1, set_avl1: 62, 52;
    pub get_nx, set_nx: 63;
}

impl PML4E {
    pub fn new() -> PML4E {
        // Sane defaults
        let mut ret = PML4E(0);
        ret.set_present(true);
        ret.set_rw(true);
        ret.set_user(false);
        ret.set_pwt(false);
        ret.set_pcd(false);
        ret.set_a(false);
        ret.set_ign0(false);
        ret.set_mbz(0);
        ret.set_avl0(0);
        // ptr pending
        ret.set_avl1(0);
        ret.set_nx(false);
        ret
    }
}

bitfield! {
    pub struct PDPE(u64);
    u64;

    pub get_present, set_present: 0;
    pub get_rw, set_rw: 1;
    pub get_user, set_user: 2;
    pub get_pwt, set_pwt: 3;
    pub get_pcd, set_pcd: 4;
    pub get_a, set_a: 5; // Available
    pub get_ign0, set_ign0: 6;
    pub get_mbz, set_mbz: 7;
    pub get_ign1, set_ign1: 8;
    pub get_avl0, set_avl0: 11, 9;
    pub get_ptr, set_ptr: 51, 12;
    pub get_avl1, set_avl1: 62, 52;
    pub get_nx, set_nx: 63;
}

impl PDPE {
    pub fn new() -> PDPE {
        let mut ret = PDPE(0);
        ret.set_present(true);
        ret.set_rw(true);
        ret.set_user(false);
        ret.set_pwt(false);
        ret.set_pcd(false);
        ret.set_a(false);
        ret.set_ign0(false);
        ret.set_mbz(false);
        ret.set_ign1(false);
        ret.set_avl0(0);
        // ptr pending
        ret.set_avl1(0);
        ret.set_nx(false);
        ret
    }
}

pub type PDE = PDPE; // Same structure

bitfield! {
    pub struct PTE(u64);
    u64;

    pub get_present, set_present: 0;
    pub get_rw, set_rw: 1;
    pub get_user, set_user: 2;
    pub get_pwt, set_pwt: 3;
    pub get_pcd, set_pcd: 4;
    pub get_a, set_a: 5;
    pub get_dirty, set_dirty: 6;
    pub get_pat, set_pat: 7;
    pub get_global, set_global: 8;
    pub get_avl0, set_avl0: 11, 9;
    pub get_ptr, set_ptr: 51, 12;
    pub get_avl1, set_avl1: 58, 52;
    pub get_avl2, set_avl2: 62, 59; // Available if CR3.PKE=0
    pub get_nx, set_nx: 63;
}

impl PTE {
    pub fn new() -> PTE {
        let mut ret = PTE(0);
        ret.set_present(true);
        ret.set_rw(true);
        ret.set_user(false);
        ret.set_pwt(false);
        ret.set_pcd(false);
        ret.set_a(false);
        ret.set_dirty(false);
        ret.set_pat(false);
        ret.set_global(false);
        ret.set_avl0(0);
        // ptr pending
        ret.set_avl1(0);
        ret.set_avl2(0);
        ret.set_nx(false);
        ret
    }
}
