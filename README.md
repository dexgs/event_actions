This software lets you run arbitrary logic which responds to inputs from
*specific* devices. You can supply your own program which overrides the default
functionality of any input device.

# Usage

Create a shared object file which defines the following functions:

(A Makefile is provided in the `actions/` directory which compiles an object
 from the source files in `actions/src/`)

## `device_update`
```c
int device_update(bool is_created, const char * name, size_t index);
```

 - `is_created`: Whether or not the device has been created or removed.
 - `name`: The name of the device.
 - `index`: The index used to refrence the device. This is used to
    distinguish incoming input events from different devices. When input events
    occur for the device, this index will be used to identify the device in
    calls to the `handle_input` function.

The `device_update` function will be called first for every device in
`/dev/input/by-id/` and additionally for each time a device is added/removed
while the program is running.

The value returned by `device_update` determines how to handle the device:

 - `0`: input events from this device will be ignored
 - `1`: input events will be read from this device
 - `2`: input events will be read from this device **and** the device will be
   "grabbed," i.e. inputs to the device will not be read by other programs.

If you don't want to use the `int` values, you can use this enum definition:
```c
enum device_action {
  IGNORE = 0,
  READ = 1,
  GRAB = 2
};
```


## `handle_input`
```c
void handle_input(struct input_event * event, size_t index);
```

 - `event`: The data for the input event.
 - `index`: The index of the device from which the input was read. This index
 
The `input_event` struct is defined as follows:

```c
struct input_event {
  struct timeval time;
  unsigned short type;
  unsigned short code;
  unsigned int value;
};
```

Further documentation on the input subsystem is available
[here](https://www.kernel.org/doc/html/v4.15/input/input.html?highlight=input_event#event-interface).

Here is a simple example program which counts key presses for an input device:

```c
#include <linux/input.h>
#include <stdio.h>
#include <stdbool.h>
#include <string.h>
#include <stdio.h>


enum device_handle {
    IGNORE = 0,
    READ = 1,
    GRAB = 2
};

enum device_handle device_update(bool is_created, const char * name, size_t index) {
    if (strcmp(name, "DEVICE NAME HERE") == 0) {
        return READ;
    } else {
        return IGNORE;
    }
}

void handle_input(struct input_event * event, size_t index) {
    if (event->type == EV_KEY && event->value == 1) {

        static int counter = 0;
        counter++;
        printf("counter: %i\n", counter);
    }
}
```

To use this program, copy it into a file located at `actions/src/action.c`
(create the `actions/src/` sub-directory if it does not already exist) and
replace `DEVICE NAME HERE` with the name of a device connected to your system.

You can find device names by reading the contents of the `/dev/input/by-id/`
directory. This program only supports the modern user-space input subsystem.
However, some devices will also create a legacy device which supplies events
using an older input system. If you see multiple entries with similar names,
the one with `event` in its name should be used.

For example: between `usb-mouse` and `usb-event-mouse`, the latter should be
used.

You can make sure you have the right device and investigate the proper input
codes using software like [evtest-qt](https://github.com/Grumbel/evtest-qt).


## Running
The program expects a single argument: the path to the shared object file to
load.

If you created the shared object using the provided Makefile in the `actions/`
directory, you can build the shared object and run the program using the
provided `run.sh` script.

The program will only ever call functions from the shared object from a single
thread, so there is no need to consider concurrency in your implementation by
default.
