// #![no_std]
use kernel::{
    c_str,
    device::Device,
    prelude::*,
    Module,
    faux,
    pr_info
};

module! {
    type: SampleModule,
    name: "rust_faux_driver",
    author: "David Westgate",
    description: "Hauppauge TV Tuner",
    license: "GPL",
}

struct SampleModule {
    _reg: faux::Registration,
}

impl Module for SampleModule {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Initialising Rust Faux Device Sample\n");

        let reg = faux::Registration::new(c_str!("rust-faux-sample-device"))?;

        dev_info!(reg.as_ref(), "Hello from faux device!\n");

        Ok(Self { _reg: reg })
    }
}

#[no_mangle]
pub extern "C" fn rust_em28174() {
    pr_info!("em28174 routine\n");
}
