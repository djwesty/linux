### Debug info
```
New device HCW 955D @ 480 Mbps (2040:826d, interface 0, class 0)
[ 1792.782161] em28xx 1-6.2.2:1.0: DVB interface 0 found: bulk
[ 1792.833301] em28xx 1-6.2.2:1.0: chip ID is em28174
[ 1793.968268] em28xx 1-6.2.2:1.0: EEPROM ID = 26 00 01 00, EEPROM hash = 0x7d990f4e
[ 1793.968283] em28xx 1-6.2.2:1.0: EEPROM info:
[ 1793.968287] em28xx 1-6.2.2:1.0: 	microcode start address = 0x0004, boot configuration = 0x01
[ 1793.974677] em28xx 1-6.2.2:1.0: 	AC97 audio (5 sample rates)
[ 1793.974681] em28xx 1-6.2.2:1.0: 	500mA max power
[ 1793.974683] em28xx 1-6.2.2:1.0: 	Table at offset 0x27, strings=0x0a72, 0x187c, 0x086a
[ 1794.025302] em28xx 1-6.2.2:1.0: Identified as Hauppauge WinTV-dualHD 01595 ATSC/QAM (card=100)
```

### SETUP
```
export LIBCLANG_PATH=/home/djw2/Documents/llvm/lib/
CONFIG_FRAME_WARN=2048
CONFIG_MODVERSIONS=n
mokutil --sb-state
scripts/config --disable CONFIG_SYSTEM_TRUSTED_KEYS
scripts/config --disable CONFIG_SYSTEM_REVOCATION_KEYS
PROMPT_DIRTIM=1
```

[blog](https://medium.com/@alessandrozanni.dev/rust-in-kernel-development-1aea34e5c4b0)


### copy config
```
cp -v /boot/config-$(uname -r) .config
make -j$(nproc) LLVM=/home/djw2/Documents/llvm/bin/ olddefconfig
make -j$(nproc) LLVM=/home/djw2/Documents/llvm/bin/ rust.config
```


### Compile kernel and modules
```
make -j$(nproc) LLVM=/home/djw2/Documents/llvm/bin/ CLIPPY=1
make -j$(nproc) modules LLVM=/home/djw2/Documents/llvm/bin/  CLIPPY=1
```


### Install kernel and modules
```
sudo make modules_install -j$(nproc) LLVM=/home/djw2/Documents/llvm/bin/ CLIPPY=1
sudo make install -j$(nproc) LLVM=/home/djw2/Documents/llvm/bin/ CLIPPY=1
```


### Bootloader
```
sudo kernelstub -v
```


### Switch Video Driver
```
software-properties-gtk
```


### Compile my module
```
make -j$(nproc) modules -C ../../../.. M=$(pwd) LLVM=/home/djw2/Documents/llvm/bin/ CLIPPY=1  obj-m=em28xx-dvb.o
```


### Rust make stuff
```
make -j$(nproc) LLVM=/home/djw2/Documents/llvm/bin/ alldefconfig rust.config
make -j$(nproc) LLVM=/home/djw2/Documents/llvm/bin/ rustavailable
make -j$(nproc) LLVM=/home/djw2/Documents/llvm/bin/ rust-analyzer
```

### Rust Binding stuff
```
bindgen \
--allowlist-type em28xx_dvb \
--allowlist-file em28xx-dvb.c \
--opaque-type fe lock  nfeeds adapter demux dmxdev fe_hw fe_mem net pll_mutex dont_attach_fe1 lna_gpio i2c_client_demod i2c_client_tuner i2c_client_sec \
-- --detect_include_paths \
-o bindings.rs
```

```
bindgen \
--clang_args=I~/Documents/linux \
--allowlist-type em28xx_dvb \
--allowlist-file em28xx-dvb.c \
--opaque-type fe \
--opaque-type lock \
--opaque-type nfeeds \
--opaque-type adapter \
--opaque-type demux \
--opaque-type dmxdev \
--opaque-type fe_hw \
--opaque-type fe_mem \
--opaque-type net \
--opaque-type pll_mutex \
--opaque-type dont_attach_fe1 \
--opaque-type lna_gpio \
--opaque-type i2c_client_demod \
--opaque-type i2c_client_tuner \
--opaque-type i2c_client_sec \
-o bindings.rs
```


### Mod Info
```
modinfo em28xx_dvb
```

### Backup Original Module
```
sudo mv /lib/modules/$(uname -r)/kernel/drivers/media/usb/em28xx/em28xx-dvb.ko \
        /lib/modules/$(uname -r)/kernel/drivers/media/usb/em28xx/em28xx-dvb.ko.ogbak
```


### Unload current module
```
sudo modprobe -r em28xx_dvb
```


### Copy New
```
sudo cp /home/djw2/Documents/linux/drivers/media/usb/em28xx/em28xx-dvb.ko .
```


### Load 
```
sudo modprobe em28xx_dvb
```


### Update module dependencies
```
sudo depmod -a
```


### Verify
```
lsmod | grep em28xx-dvb
modinfo em28xx-dvb
```

### Logs
```
sudo dmesg --follow | grep em28xx_dvb
```

### Test
```
dvbv5-zap -c dvb_channel.conf -CUS -IZAP "KATU"
```

### Unload
```
sudo rmmod em28xx_dvb
```