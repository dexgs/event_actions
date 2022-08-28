#include <linux/input.h>
#include <poll.h>
#include <sys/inotify.h>
#include <fcntl.h>
#include <dlfcn.h>

// ioctl grab request
unsigned long EVIOCGRAB_ = EVIOCGRAB;

// polling event types
short POLLIN_ = POLLIN;

// inotify
uint32_t IN_CREATE_ = IN_CREATE;
uint32_t IN_DELETE_ = IN_DELETE;

// fcntl
int F_SETFL_ = F_SETFL;
int O_NONBLOCK_ = O_NONBLOCK;

// dlfcn
int RTLD_NOW_ = RTLD_NOW;
