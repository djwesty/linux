// use bindgen;

// fn main() {
//     let bindings = bindgen::Builder::default()
//         .header("em28xx-dvb-wrapper.h")
//         .allowlist_type("em28xx_dvb")
//         .allowlist_file("em28xx-dvb-wrapper.h")
//         // .header("em28xx-dvb.c")
//         .clang_arg("-I/home/djw2/Documents/llvm/include")
//         .allowlist_recursively(false)
        
//         .clang_arg("-I/home/djw2/Documents/linux/include") // Kernel headers (first)
//         // .clang_arg("-I/home/djw2/Documents/linux/include/uapi/linux/") // userspace

//         .clang_arg("-I/home/djw2/Documents/linux/arch/x86/include")  // Architecture-specific
//         // .clang_arg("-I/home/djw2/Documents/linux/arch/x86/include/uapi") // userspace

//         // .clang_arg("-I/home/djw2/Documents/linux/tools/virtio") // Tools headers (last)
//         .clang_arg("-I/home/djw2/Documents/linux/tools/include") // Tools headers (last)
        
//         // .blocklist_file("tools/include/linux/types.h")
//         // .blocklist_file("media/dvb_demux.h")
//         // .blocklist_file("media/dvb_net.h")
//         // .blocklist_file("media/dmxdev.h")

//         // .allowlist_file("em28xx-dvb.c")
//         // .allowlist_recursively(false)
//         // .blocklist_file(".*swab\\.h")
//         // .blocklist_file(".*compiler\\.h")
//         // .blocklist_file("linux/swab.h")
//         // .blocklist_file("uapi/linux/swab.h")
//         // .blocklist_file("/home/djw2/Documents/linux/include/uapi/linux/swab.h")
//         // .blocklist_file("linux/bitfield.h")
//         // .blocklist_file(".*bitfield\\.h")
//         // .blocklist_file(".*bitops\\.h")
//         // .blocklist_file(".*barrier\\.h")


//         // .blocklist_file(".*types\\.h") // Sometimes needed for kernel bindings
//         // .blocklist_file(".*kernel\\.h")
//         // .blocklist_file(".*uapi.*")




//         .opaque_type("fe")
//         .opaque_type("lock")
//         .opaque_type("nfeeds")
//         .opaque_type("adapter")
//         .opaque_type("demux")
//         .opaque_type("dmxdev")
//         .opaque_type("fe_hw")
//         .opaque_type("fe_mem")
//         .opaque_type("net")
//         .opaque_type("pll_mutex")
//         .opaque_type("dont_attach_fe1")
//         .opaque_type("lna_gpio")
//         .opaque_type("i2c_client_demod")
//         .opaque_type("i2c_client_tuner")
//         .opaque_type("i2c_client_sec")
//         .generate()
//         .expect("Unable to generate bindings");

//     bindings
//         .write_to_file("bindings.rs")
//         .expect("Couldn't write bindings!");
// }
