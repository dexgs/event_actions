use std::collections::HashSet;


mod events;
mod glob;
mod language;

use glob::*;

fn main() {
    let glob = glob("~/*");
    println!("{:?}", glob);
    /*
    let mut event_codes = HashSet::new();
    event_codes.insert("KEY_EDITOR");

    let event_values = event_codes::lookup(&event_codes).unwrap();

    let a = *event_values.get("KEY_EDITOR").unwrap();

    println!("{a}");
    */

    /*
    let mut readers = [
        EventReader::new("/dev/input/by-id/usb-0079_USB_Gamepad-event-joystick").unwrap(),
    ];
    */
    /*
    let mut poller = EventReaderPoller::new(&mut readers);

    loop {
        for reader in poller.poll().into_iter() {
            match reader.next() {
                Ok(input_event) => println!("{:?}", input_event),
                Err(e) => eprintln!("{}", e)
            }
        }
    }
    */
}
