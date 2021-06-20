#![feature(asm)]

use std::ffi::CString;
use std::fs;
use std::io;
use std::mem;
use std::path::Path;

// linux amd64 syscall ID
const SYS_STAT: usize = 4;

// linux amd64 types only
#[derive(Debug, Default)]
#[repr(C)]
pub struct Stat {
    st_dev: u64,   // ID of device containing file
    st_ino: u64,   // Inode number
    st_nlink: u64, // Number of hard links
    st_mode: u32,  // File type and mode
    st_uid: u32,   // User ID of owner
    st_gid: u32,   // Group ID of owner
    pad0: u32,
    st_rdev: u64,      // Device ID (if special file)
    st_size: i64,      // Total size, in bytes
    st_blksize: i64,   // Block size for filesystem I/O
    st_blocks: i64,    // Number of 512B blocks allocated
    st_atim: Timespec, // Time of last access
    st_mtim: Timespec, // Time of last modification
    st_ctim: Timespec, // Time of last status changei
    pad1: i64,
    pad2: i64,
    pad3: i64,
}

// linux amd64 types only
#[derive(Debug, Default)]
#[repr(C)]
pub struct Timespec {
    tv_sec: u64,  // Seconds
    tv_nsec: u64, // Nanoseconds
}

#[inline(always)]
unsafe fn syscall2(n: usize, a1: usize, a2: usize) -> usize {
    let ret: usize;
    asm!(
        "syscall",
        in("rax") n,
        in("rdi") a1,
        in("rsi") a2,
        out("rcx") _,
        out("r11") _,
        lateout("rax") ret,
    );
    return ret;
}

fn stat(path: &str) -> Result<Stat, io::Error> {
    let buf = Stat::default();
    let cpath = CString::new(path);
    return match unsafe { syscall2(SYS_STAT, cpath?.as_ptr() as usize, mem::transmute(&buf)) }
        as i32
    {
        s if s < 0 => Err(io::Error::from_raw_os_error(s.abs())),
        _ => Ok(buf),
    };
}

fn traverse_dirs(dir: &Path, all: bool) -> Result<i64, io::Error> {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("crustydu: cannot read directory '{}': {}", dir.display(), e);
            println!("{}\t{}", 0, dir.display());
            return Err(e);
        }
    };

    let mut size: i64 = 0;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            size += match traverse_dirs(&path, all) {
                Ok(s) => s,
                Err(_e) => 0,
            };
        } else {
            // replace with stat syscall
            let fsize = match stat(path.to_str().expect("missing path")) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("crustydu: cannot stat file '{}': {}", path.display(), e);
                    continue;
                }
            }
            .st_size;
            if all {
                println!("{}\t{}", fsize, path.display());
            }
            size += fsize;
        }
    }

    println!("{}\t{}", size, dir.display());
    return Ok(0);
}

fn main() {
    // traverse_dirs(Path::new("/etc"), true);
    let s = match stat("/etc/shadow") {
        Ok(s) => println!("{:?}", s),
        Err(e) => eprintln!("{}", e),
    };
}
