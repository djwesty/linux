// SPDX-License-Identifier: GPL-2.0

//! This is an example Rust helper function to support the Em28174 subset of em28xx-dvb.

use kernel::pr_info;
const __LOG_PREFIX: &[u8] = b"em28xx  ";

/// A quick test of calling rust code from a Kernel C module
#[no_mangle]
pub extern "C" fn em28174_helper() -> u8 {
    pr_info!("CS533: Hello From EM28174 helper lib");
    0
}
