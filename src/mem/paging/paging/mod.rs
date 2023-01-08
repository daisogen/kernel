mod structures;

use crate::mem::pmm;
use crate::mem::PAGE_SIZE;
use structures::*;

pub struct Paging {
    data: &'static mut [PML4E; EPP],
}

#[derive(Clone, Copy)]
pub struct Map {
    virt: u64,
    phys: u64,
    pub npages: usize,
    pub global: bool,
    pub ro: bool,
    pub nx: bool,
    pub pcd: bool,
}

fn get_indices(addr: u64) -> (usize, usize, usize, usize) {
    (
        ((addr >> (9 * 3 + 12)) & 0x1FF) as usize, // pml4i (PML4 index)
        ((addr >> (9 * 2 + 12)) & 0x1FF) as usize, // pdpi
        ((addr >> (9 * 1 + 12)) & 0x1FF) as usize, // pdi
        ((addr >> (9 * 0 + 12)) & 0x1FF) as usize, // pti
    )
}

macro_rules! alloc_paging_level {
    ($obj:expr, $class:ty) => {
        if !$obj.get_present() {
            let next = pmm::calloc(1);
            if next.is_err() {
                return Err(());
            }
            *$obj = <$class>::new();
            $obj.set_ptr(next.unwrap() >> 12);
        }
    };
}

impl Paging {
    pub fn new(cr3: u64) -> Paging {
        Paging {
            data: unsafe { &mut *(cr3 as *mut [PML4E; EPP]) },
        }
    }

    pub fn newmap(virt: u64, phys: u64) -> Map {
        Map {
            virt: virt,
            phys: phys,
            npages: 1,
            global: false,
            ro: false,
            nx: false,
            pcd: false,
        }
    }

    /*
    I've simplified this from Strife to make it more readable,
      but in exchange it's ~4x slower, since this only maps one page,
      and the indices must be recalculated and the fields reaccessed.
    I think the point of Daisogen is to keep things as simple as possible
      so it's easy to hack them, so I don't feel too bad about it.
    Mappings of big regions hardly ever happen after boot so it's not
      that big of a deal anyway.
    */
    pub fn map(&mut self, map: Map) -> Result<(), ()> {
        let npages = map.npages;
        let mut map = map; // Copy and modify it
        for _ in 0..npages {
            if self.map_one(map).is_err() {
                return Err(());
            }
            map.virt += PAGE_SIZE as u64;
            map.phys += PAGE_SIZE as u64;
        }

        Ok(())
    }

    fn map_one(&mut self, map: Map) -> Result<(), ()> {
        let (pml4i, pdpi, pdi, pti) = get_indices(map.virt);
        let pml4e: &mut PML4E = &mut self.data[pml4i];
        alloc_paging_level!(pml4e, PML4E);

        let pdpes = pml4e.get_ptr() << 12;
        let pdpes: &'static mut [PDPE; EPP] = unsafe { &mut *(pdpes as *mut [PDPE; EPP]) };
        let pdpe: &mut PDPE = &mut pdpes[pdpi];
        alloc_paging_level!(pdpe, PDPE);

        let pdes = pdpe.get_ptr() << 12;
        let pdes: &'static mut [PDE; EPP] = unsafe { &mut *(pdes as *mut [PDE; EPP]) };
        let pde: &mut PDE = &mut pdes[pdi];
        alloc_paging_level!(pde, PDE);

        let ptes = pde.get_ptr() << 12;
        let ptes: &'static mut [PTE; EPP] = unsafe { &mut *(ptes as *mut [PTE; EPP]) };
        let pte: &mut PTE = &mut ptes[pti];
        *pte = PTE::new();

        pte.set_global(map.global);
        pte.set_rw(!map.ro);
        pte.set_nx(map.nx);
        pte.set_pcd(map.pcd);
        pte.set_ptr(map.phys >> 12);

        Ok(())
    }

    // Get mapped
    pub fn get_ptr(&self, virt: u64) -> Option<u64> {
        let (pml4i, pdpi, pdi, pti) = get_indices(virt);
        let pml4e: &PML4E = &self.data[pml4i];
        if !pml4e.get_present() {
            return None;
        }

        let pdpes = pml4e.get_ptr() << 12;
        let pdpes: &'static [PDPE; EPP] = unsafe { &*(pdpes as *const [PDPE; EPP]) };
        let pdpe: &PDPE = &pdpes[pdpi];
        if !pdpe.get_present() {
            return None;
        }

        let pdes = pdpe.get_ptr() << 12;
        let pdes: &'static [PDE; EPP] = unsafe { &*(pdes as *const [PDE; EPP]) };
        let pde: &PDE = &pdes[pdi];
        if !pde.get_present() {
            return None;
        }

        let ptes = pde.get_ptr() << 12;
        let ptes: &'static [PTE; EPP] = unsafe { &*(ptes as *mut [PTE; EPP]) };
        let pte: &PTE = &ptes[pti];
        if !pte.get_present() {
            return None;
        }

        Some(pte.get_ptr() << 12)
    }

    pub fn load(&self) {
        crate::utils::regs::set_cr3(self.data as *const _ as u64);
    }
}
