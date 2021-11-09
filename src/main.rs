use std::{thread::sleep, time::Duration};

use rusb::{Context, UsbContext};

fn main() {
    let ctx = Context::new().expect("Can't create libusb context");
    let mut dev = ctx
        .open_device_with_vid_pid(0x2b73, 0x0020)
        .expect("Can't open Pioneer");
    println!("Found DDJ-1000.");

    println!("Claiming interface 0...");
    dev.claim_interface(0).expect("Can't claim interface");
    println!("Setting iface 0 to alternate 1...");
    dev.set_alternate_setting(0, 1).expect("Setting failed");

    println!("Sleeping 10 seconds...");
    sleep(Duration::from_secs(10));
    println!("Releasing interface...");
    dev.release_interface(0).expect("Can't release interface");
}
