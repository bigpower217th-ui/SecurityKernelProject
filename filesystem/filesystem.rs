/* The file system for the operating system kernel was
* created with Rust for security reasons.
* I've included the communication system kernel as tar.gz
* and zip files in the releases section If you have an i386
* 32-bit system, run it with qemu I've also included the 
* object files If you have any problems, write to the issues
* section. linking is difficult because I'm using ARM32 Android.
* 
*/

#![no_std]
#![no_main]

use core::ptr;

use::core::panic::PanicInfo;

extern "C" {
   fn ata_write_250gb(lba: u64, count: u16, buffer:  *const u16) -> i32;
   fn ata_status_wait(mask: u8, timeout: i32) -> u32;
}

const SECTOR_SIZE: usize = 512;
const MAX_FILES: usize = 256;
const DISK_SIZE_SECTORS: u64 = 488397168;
const BITMAP_START_LBA: u64 = 1;
const INODE_TABLE_START_LBA: u64 = 20;
const DATA_START_LBA: u64 = 2000;

#[repr(C, packed)]
pub struct Inode {
pub name: [u8; 64],
pub size_bytes: u64,
pub start_block: u64,
pub is_directory: bool,
pub permissions: u16,
pub created: u32,
pub is_active: bool,
}

pub struct SuperBlock {
pub magic_number: u32,
pub total_inodes: u32,
pub free_blocks: u64,
}

pub struct FileSystem {
pub root_inodes: [Inode; MAX_FILES],
pub free_map: [u8; 4096],
}

impl FileSystem {
 pub const fn new() -> Self {

    unsafe { core::mem::zeroed() }
  }

pub fn format_disk(&self) -> i32 {
   let sb = SuperBlock {
       magic_number: 0x535A4653,
       total_inodes: MAX_FILES as u32,
       free_blocks: DISK_SIZE_SECTORS - DATA_START_LBA,
    };

    unsafe {

       ata_write_250gb(1, 1, &sb as *const _ as *const u16)
    }
}

pub fn create_entry(&mut self, name: &str, is_dir: bool) -> i32 {
   let mut slot_index = -1;

  for i in 0..MAX_FILES {
      if !self.root_inodes[i].is_active {
         slot_index = i as i32;
        break;
     }
}

if slot_index == -1 { return -2; }

let idx = slot_index as usize;
let bytes = name.as_bytes();
let len = if bytes.len() > 64 { 64 } else { bytes.len() };

for i in 0..len { self.root_inodes[idx].name[i]= bytes[i]; }
self.root_inodes[idx].is_active = true;
self.root_inodes[idx].is_directory = is_dir;
self.root_inodes[idx].start_block = self.find_free_block();

self.sync_inode_table(idx as u64);

   0
 }

fn find_free_block(&self) -> u64 {

   DATA_START_LBA + 1
 }

fn sync_inode_table(&self, index: u64) {
   unsafe {

      let lba = INODE_TABLE_START_LBA + index;
     ata_write_250gb(lba, 1, &self.root_inodes[index as usize] as *const _ as *const u16);
   }
}

pub fn write_to_file(&self, inode_index: usize, buffer: *const u16, count: u16) -> i32 {
    if !self.root_inodes[inode_index].is_active { return -1; }

    let start_lba = self.root_inodes[inode_index].start_block;
    unsafe {
       ata_write_250gb(start_lba, count, buffer)
    }
  }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
     loop{}
    }
