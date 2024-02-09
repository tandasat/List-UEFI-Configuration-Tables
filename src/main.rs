#![doc = include_str!("../README.md")]
#![no_main]
#![no_std]

use uefi::prelude::*;
use uefi_services::println;

#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    let _ = uefi_services::init(&mut system_table).unwrap();

    for table in system_table.config_table() {
        println!("GUID:{}, Address:{:?}", table.guid, table.address);
    }

    Status::SUCCESS
}
