#![no_std]



use core::panic::PanicInfo;


#[repr(C, packed)]
pub struct WifiPacket {
   pub data: [u8; 1500],
   pub length: u32,
   pub source_mac: [u8; 6], 
   pub dest_mac: [u8; 6],
}

#[no_mangle]
pub extern "C" fn rust_validate_packet(packet: *mut WifiPacket) -> i8 {
   let p = unsafe { &*packet };

   if p.length > 1500 {
   return -1;
  }
  return 0;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
