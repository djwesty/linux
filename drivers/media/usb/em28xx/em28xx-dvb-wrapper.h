// #include "em28xx.h"

// #include <linux/kernel.h>
// #include <linux/slab.h>
// #include <linux/usb.h>

// #include <media/v4l2-common.h>
#include <media/dvb_demux.h>
#include <media/dvb_net.h>
#include <media/dmxdev.h>
// #include <media/tuner.h>
// #include "tuner-simple.h"
// #include <linux/gpio.h>

// #include "lgdt330x.h"
// #include "lgdt3305.h"
// #include "lgdt3306a.h"
// #include "zl10353.h"
// #include "s5h1409.h"
// #include "mt2060.h"
// #include "mt352.h"
// #include "mt352_priv.h" /* FIXME */
// #include "tda1002x.h"
// #include "drx39xyj/drx39xxj.h"
// #include "tda18271.h"
// #include "s921.h"
// #include "drxd.h"
// #include "cxd2820r.h"
// #include "tda18271c2dd.h"
// #include "drxk.h"
// #include "tda10071.h"
// #include "tda18212.h"
// #include "a8293.h"
// #include "qt1010.h"
// #include "mb86a20s.h"
// #include "m88ds3103.h"
// #include "ts2020.h"
// #include "si2168.h"
// #include "si2157.h"
// #include "tc90522.h"
// #include "qm1d1c0042.h"
// #include "mxl692.h"

struct em28xx_dvb {
	struct dvb_frontend        *fe[2];

	/* feed count management */
	struct mutex               lock;
	int                        nfeeds;

	/* general boilerplate stuff */
	struct dvb_adapter         adapter;
	struct dvb_demux           demux;
	struct dmxdev              dmxdev;
	struct dmx_frontend        fe_hw;
	struct dmx_frontend        fe_mem;
	struct dvb_net             net;

	/* Due to DRX-K - probably need changes */
	int (*gate_ctrl)(struct dvb_frontend *fe, int gate);
	struct semaphore      pll_mutex;
	bool			dont_attach_fe1;
	int			lna_gpio;
	struct i2c_client	*i2c_client_demod;
	struct i2c_client	*i2c_client_tuner;
	struct i2c_client	*i2c_client_sec;
};