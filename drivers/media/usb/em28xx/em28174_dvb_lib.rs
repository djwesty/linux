// SPDX-License-Identifier: GPL-2.0

//! This is an example Rust helper function to support the Em28174 subset of em28xx-dvb.

use kernel::pr_info;
const __LOG_PREFIX: &[u8] = b"rust_kernel\0";
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



/// Hello
#[no_mangle]
pub extern "C" fn hello_from_rust() -> u8 {
    pr_info!("CS533: Hello From Rust!");
    0
}

const PRIMARY_TS: i32 = 0;

/// Get address from em28xx->ts
#[no_mangle]
pub extern "C" fn get_em28174_addr_from_ts(ts: i32) -> u8 {
    unsafe {
        pr_info!("CS533: get_em28174_addr: ts {}", ts);
        if ts == PRIMARY_TS {
            0x59
        } else {
            0x0e
        }
    }
}

#[repr(C)]
pub struct em28xx {
    ts: i32,
}

/// Get address from em28xx->ts
#[no_mangle]
pub extern "C" fn get_em28174_addr(dev: *mut em28xx) -> u8 {
    unsafe {
        let ts: i32 = (*dev).ts;
        pr_info!("CS533: get_em28174_addr: dev->ts {}", ts);
        if ts == PRIMARY_TS {
            0x59
        } else {
            0x0e
        }
    }
}
