# SPDX-License-Identifier: GPL-2.0
em28xx-y +=	em28xx-core.o em28xx-i2c.o em28xx-cards.o em28xx-camera.o

em28xx-v4l-objs := em28xx-video.o em28xx-vbi.o
em28xx-alsa-objs := em28xx-audio.o
em28xx-rc-objs := em28xx-input.o



obj-$(CONFIG_VIDEO_EM28XX) += em28xx.o
obj-$(CONFIG_VIDEO_EM28XX_V4L2) += em28xx-v4l.o
obj-$(CONFIG_VIDEO_EM28XX_ALSA) += em28xx-alsa.o
# em28xx-dvb-objs += lib.o em28xx-dvb
obj-$(CONFIG_VIDEO_EM28XX_DVB) += em28xx-dvb.o
obj-$(CONFIG_VIDEO_EM28XX_RC) += em28xx-rc.o # delete?
# obj-m += em2174.o

# Option 1: Add Rust object file to em28xx-dvb-objs
# em28xx-dvb-objs := em28xx-dvb.o lib.o
# $(info em28xx-dvb: $(em28xx-dvb-objs))
# rust-lib := lib.o
em28xx-dvb-objs += em28xx_dvb_support.o em28xx-dvb-driver.o

# em28xx-dvb-objs += lib.o
# $(info em28xx-dvb: $(obj-m))


# Option 2: Build this as an independent module to be used by em28xx-dvb
# obj-m += lib.o


ccflags-y += -I $(srctree)/drivers/media/tuners
ccflags-y += -I $(srctree)/drivers/media/dvb-frontends
# $(info em28xx-dvb: $(em28xx-dvb-objs))
