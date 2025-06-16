use core::panic::PanicInfo;

use crate::libs::libc::exit;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    exit(1);

    // unreachable
    loop {}
}