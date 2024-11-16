use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn f() {
    sleep(Duration::from_secs(1));
    let id = thread::current().id();
    println!("Hello from thread: {id:?}");
}

fn main() {
    let h1 = thread::spawn(f);
    let h2 = thread::spawn(f);

    let id = thread::current().id();
    println!("Hello from the main thread: {id:?}");

    h1.join().unwrap();
    h2.join().unwrap();
}
