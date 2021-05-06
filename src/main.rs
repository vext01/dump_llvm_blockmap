use byteorder::{NativeEndian, ReadBytesExt};
use memmap2;
use object::{Object, ObjectSection};
use std::{env, fs, io::Cursor};

const BLOCK_MAP_SEC: &str = ".llvm_bb_addr_map";

pub fn dump(path: &Path) {
    let file = fs::File::open(path).unwrap();
    let mmap = unsafe { memmap2::Mmap::map(&file).unwrap() };
    let object = object::File::parse(&*mmap).unwrap();
    let sec = object.section_by_name(BLOCK_MAP_SEC).unwrap();
    let sec_size = sec.size();
    let mut crsr = Cursor::new(sec.data().unwrap());

    println!("{}-byte {} section:", sec_size, BLOCK_MAP_SEC);

    // Keep reading records until we fall outside of the section's bounds.
    while crsr.position() < sec_size {
        let f_off = crsr.read_u64::<NativeEndian>().unwrap();
        println!("  function offset: 0x{:x}", f_off);
        let n_blks = leb128::read::unsigned(&mut crsr).unwrap();
        println!("  num blocks: {}", n_blks);
        for bb in 0..n_blks {
            println!("    bb{}:", bb);

            let b_off = leb128::read::unsigned(&mut crsr).unwrap();
            println!("      offset: {}", b_off);

            let b_sz = leb128::read::unsigned(&mut crsr).unwrap();
            println!("      size: {}", b_sz);

            let b_meta = crsr.read_u8().unwrap();
            println!("      meta: {}\n", b_meta);
        }
    }
}

use std::path::Path;
fn main() {
    let path = env::args().skip(1).next().unwrap();
    dump(&Path::new(&path));
}
