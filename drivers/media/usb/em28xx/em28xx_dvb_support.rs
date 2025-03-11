// SPDX-License-Identifier: GPL-2.0

//! This is a Rust implementation Em28174.

// #![no_std]
// #[allow(unused_imports)]
// use kernel::{c_str, device::Device, faux, pr_info, prelude::*, Module};
use kernel::{pr_info, prelude::*, print};
// use kernel::*;
// module! {
//     type: Em28174,
//     name: "rust_minimal",
//     author: "David Westgate",
//     description: "Module for em28174 support",
//     license: "GPL",
// }

// struct Em28174 {}

// impl kernel::Module for Em28174 {
//     fn init(_module: &'static ThisModule) -> Result<Self> {
//         pr_info!("David Westgate Em28174 Module (init)\n");
//         Ok(Em28174 {  })
//     }
// }

/// My function
#[no_mangle]
#[expect(clippy::missing_safety_doc)]
pub extern "C" fn hello_from_rust() -> u8 {
    // bindings::printk("6" "Message: %s\n", "test");
    // format_string: &[u8; format_strings::LENGTH],
    // module_name: &[u8],
    // args: fmt::Arguments<'_>,
    unsafe {
        print::call_printk(b"CS533: Hel", b"em28xx-dvb-driver", format_args!(""));
    }
    // pr_info!("CS533: Hello From Em28174 ");
    0
}
