# SPDX-License-Identifier: GPL-2.0
em28xx-y +=	em28xx-core.o em28xx-i2c.o em28xx-cards.o em28xx-camera.o

em28xx-v4l-objs := em28xx-video.o em28xx-vbi.o
em28xx-alsa-objs := em28xx-audio.o
em28xx-rc-objs := em28xx-input.o



obj-$(CONFIG_VIDEO_EM28XX) += em28xx.o
obj-$(CONFIG_VIDEO_EM28XX_V4L2) += em28xx-v4l.o
obj-$(CONFIG_VIDEO_EM28XX_ALSA) += em28xx-alsa.o
obj-$(CONFIG_VIDEO_EM28XX_DVB) += em28xx-dvb.o
obj-$(CONFIG_VIDEO_EM28XX_RC) += em28xx-rc.o

ifdef CONFIG_RUST
	# Approach 1: Include rust lib file to support existing (renamed) dvb  module
	em28xx-dvb-objs += em28174_dvb_lib.o em28xx-dvb-2.o

	# Approach 2: Build new specific sub-module
	obj-$(CONFIG_VIDEO_EM28XX_DVB) += em28174_dvb.o
endif

ccflags-y += -I $(srctree)/drivers/media/tuners
ccflags-y += -I $(srctree)/drivers/media/dvb-frontends