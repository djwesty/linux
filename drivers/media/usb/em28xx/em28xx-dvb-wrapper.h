#include <media/dvb_demux.h>
#include <media/dvb_net.h>
#include <media/dmxdev.h>

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