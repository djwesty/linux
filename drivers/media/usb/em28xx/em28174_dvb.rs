// SPDX-License-Identifier: GPL-2.0

//! Sub Module to support the Em28174 subset of em28xx-dvb.

use kernel::{prelude::*, Module};

module! {
    type: Em28174,
    name: "em28174_dvb",
    author: "David Westgate",
    description: "Support module for em28174 subset of em28xx_dvb device",
    license: "GPL",
}

struct Em28174 {
    // _reg: faux::Registration,
}

impl Module for Em28174 {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("CS533: Initializing module\n");
        // let reg = faux::Registration::new(c_stThis is an example Rust helper functionr!("em28174-dvb-sample-device"))?;
        // dev_info!(reg.as_ref(), "Hello from faux device!\n");
        // Ok(Self { _reg: reg })
        Ok(Self {})
    }
}
