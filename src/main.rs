#[macro_use]
extern crate stdweb;

fn main() {
    stdweb::initialize();

    let message = "Hello, 世界!";

    js! {
        console.log( @{message} );
    }

    stdweb::event_loop();
}