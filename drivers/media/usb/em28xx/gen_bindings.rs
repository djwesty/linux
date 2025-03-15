use bindgen;

fn main() {
    let bindingsWrapper = bindgen::Builder::default()
        .header("em28xx-dvb-wrapper.h")
        .allowlist_type("struct")
        .allowlist_file("em28xx-dvb-wrapper.h")

        .clang_arg("-I/home/djw2/Documents/llvm/include")
        .allowlist_recursively(false)
        
        .clang_arg("-I/home/djw2/Documents/linux/include") // Kernel headers (first)
        // .clang_arg("-I/home/djw2/Documents/linux/include/uapi/linux/") // userspace

        .clang_arg("-I/home/djw2/Documents/linux/arch/x86/include") // Architecture-specific
        // .clang_arg("-I/home/djw2/Documents/linux/arch/x86/include/uapi") // userspace

        // .clang_arg("-I/home/djw2/Documents/linux/tools/virtio") // Tools headers (last)
        .clang_arg("-I/home/djw2/Documents/linux/tools/include") // Tools headers (last)
        
        .opaque_type("fe")
        .opaque_type("lock")
        .opaque_type("nfeeds")
        .opaque_type("adapter")
        .opaque_type("demux")
        .opaque_type("dmxdev")
        .opaque_type("fe_hw")
        .opaque_type("fe_mem")
        .opaque_type("net")
        .opaque_type("pll_mutex")
        .opaque_type("dont_attach_fe1")
        .opaque_type("lna_gpio")
        .opaque_type("i2c_client_demod")
        .opaque_type("i2c_client_tuner")
        .opaque_type("i2c_client_sec")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("bindings.rs")
        .expect("Couldn't write bindings!");

    // let bindingsWrapper = bindgen::Builder::default()
    //     .header("em28xx.h")
    //     .allowlist_item("PRIMARY_TS")
    //     .allowlist_recursively(false)


    //     .clang_arg("-I/home/djw2/Documents/linux/include") // Kernel headers (first)
    //     // .clang_arg("-I/home/djw2/Documents/linux/include/uapi/linux/") // userspace

    //     .clang_arg("-I/home/djw2/Documents/linux/arch/x86/include") // Architecture-specific
    //     // .clang_arg("-I/home/djw2/Documents/linux/arch/x86/include/uapi") // userspace

    //     // .clang_arg("-I/home/djw2/Documents/linux/tools/virtio") // Tools headers (last)
    //     .clang_arg("-I/home/djw2/Documents/linux/tools/include") // Tools headers (last)

    //     .blocklist_file("linux/swab.h")
    //     .blocklist_file("swab.h")
    //     .blocklist_file("linux/bitfield.h")
    //     .blocklist_file("bitfield.h")
    //     .generate()
    //     .expect("Unable to generate bindings")
    //     .write_to_file("bindings.rs")
    //     .expect("Couldn't write bindings!");
}
