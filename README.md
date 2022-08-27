This software lets you run arbitrary logic which responds to inputs from
*specific* devices.


# Usage

Create a shared object file which defines the following functions:

## `device_update`
```c
int device_update(bool is_created, const char * name, size_t index);
```

 - `is_created`: Whether or not the device has been created or removed.
 - `name`: The name of the device.
 - `index`: The index used to refrence the device. This is used to
    distinguish incoming input events from different devices.

The `device_update` function will be called first for every device in
`/dev/input/by-id/` and additionally for each time a device is added/removed
while the program is running.

The value returned by `device_update` determines how to handle the device:

 - `0`: input events from this device will be ignored
 - `1`: input events will be read from this device
 - `2`: input events will be read from this device **and** the device will be
   "grabbed," i.e. inputs to the device will not be read by other programs.


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

## Running
The program expects a single argument: the path to the shared object file to
load.

The program will only ever call functions from the shared object from a single
thread.