// SPDX-License-Identifier: GPL-2.0

//! This is an example Rust helper function to support the Em28174 subset of em28xx-dvb.

use kernel::pr_info;
const __LOG_PREFIX: &[u8] = b"em28xx  ";
#[repr(C)]
struct dvb_frontend;
#[repr(C)]
struct mutex;
#[repr(C)]
struct dvb_adapter;
#[repr(C)]
struct dvb_demux;
#[repr(C)]
struct dmxdev;
#[repr(C)]
struct dmx_frontend;
#[repr(C)]
struct dvb_net;
#[repr(C)]
struct semaphore;
#[repr(C)]
struct i2c_client;

#[repr(C)]
pub struct em28xx_dvb {
    fe: [*mut dvb_frontend; 2], // Array of pointers to dvb_frontend

    /* Feed count management */
    lock: mutex,
    nfeeds: i32,

    /* General boilerplate stuff */
    adapter: dvb_adapter,
    demux: dvb_demux,
    dmxdev: dmxdev,
    fe_hw: dmx_frontend,
    fe_mem: dmx_frontend,
    net: dvb_net,

    /* Due to DRX-K - probably need changes */
    gate_ctrl: Option<extern "C" fn() -> i32>, // Function pointer

    pll_mutex: *mut semaphore, // Semaphore as opaque pointer
    dont_attach_fe1: bool,
    lna_gpio: i32,

    i2c_client_demod: *mut i2c_client,
    i2c_client_tuner: *mut i2c_client,
    i2c_client_sec: *mut i2c_client,
}

/// A quick test of calling rust code from a Kernel C module
#[no_mangle]
pub extern "C" fn em28174_helper(dev: *mut em28xx_dvb) -> u8 {
    pr_info!("CS533: nope");
    unsafe {
        // pr_info!("CS533: feeds {}", (*dev).nfeeds);
        // pr_info!("CS533: lna_gpio {}", (*dev).lna_gpio);
    }
    0
}
