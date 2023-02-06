use std::process::exit;

fn main() {
    let window = match moonshine::create_window() {
        Ok(val) => val,
        Err(err) => {
            eprintln!("ERROR: {}", err);
            exit(1);
        }
    };

    let mut quit = false;
    while !quit {
        if let Some(_) = moonshine::poll_event() {
            println!("some event received");
        }
    }

    moonshine::close_window(window);
}
