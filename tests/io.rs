use ansiconst::{*, io::*};
use std::{thread, time};

#[test]
fn test_io_threadsafe() {
    const RED: Ansi = ansi!(Red);
    const BLUE: Ansi = ansi!(Blue);

    // Ensure another thread CANNOT set ansiout().ansi() while this thread holds lock
    {
        let mut ansiout = Some(io::ansiout());
        ansiout.as_mut().unwrap().set_ansi(RED);
        assert_eq!(ansiout.as_ref().unwrap().ansi(), RED);

        thread::spawn(|| {
            io::ansiout().set_ansi(BLUE);
        });

        thread::sleep(time::Duration::from_millis(500));
        assert_eq!(ansiout.as_ref().unwrap().ansi(), RED);
    }

    // Ensure another thread CAN set ansiout().ansi() after dropped by this thread
    {
        io::ansiout().set_ansi(RED);
        assert_eq!(io::ansiout().ansi(), RED);

        thread::spawn(|| {
            io::ansiout().set_ansi(BLUE);
        });

        thread::sleep(time::Duration::from_millis(500));
        assert_eq!(io::ansiout().ansi(), BLUE);
    }
}
