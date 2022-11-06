use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // do nothing but looping if error occurs
    loop {}
}
