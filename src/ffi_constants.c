#include <glob.h>
#include <linux/input.h>
#include <poll.h>
#include <sys/inotify.h>
#include <fcntl.h>
#include <dlfcn.h>

// ioctl grab request
unsigned long EVIOCGRAB_ = EVIOCGRAB;

// event types
/*
unsigned short EV_SYN_ = EV_SYN;
unsigned short EV_KEY_ = EV_KEY;
unsigned short EV_REL_ = EV_REL;
unsigned short EV_ABS_ = EV_ABS;
unsigned short EV_MSC_ = EV_ABS;
unsigned short EV_SW_ = EV_SW;
unsigned short EV_LED_ = EV_LED;
unsigned short EV_SND_ = EV_SND;
unsigned short EV_REP_ = EV_REP;
unsigned short EV_FF_ = EV_FF;
unsigned short EV_PWR_ = EV_PWR;
unsigned short EV_FF_STATUS_ = EV_FF_STATUS;
unsigned short EV_MAX_ = EV_MAX;
unsigned short EV_CNT_ = EV_CNT;
*/

// polling event types
short POLLIN_ = POLLIN;

// glob constants
int GLOB_NOMATCH_ = GLOB_NOMATCH;

// glob flags
int GLOB_NOSORT_ = GLOB_NOSORT;
int GLOB_TILDE_ = GLOB_TILDE;

// inotify
uint32_t IN_CREATE_ = IN_CREATE;
uint32_t IN_DELETE_ = IN_DELETE;

// fcntl
int F_SETFL_ = F_SETFL;
int O_NONBLOCK_ = O_NONBLOCK;

// dlfcn
int RTLD_NOW_ = RTLD_NOW;
