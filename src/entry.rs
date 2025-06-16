use crate::{libs::libc::exit, main};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    main();

    exit(0);

    // unreachable
    loop {}
}