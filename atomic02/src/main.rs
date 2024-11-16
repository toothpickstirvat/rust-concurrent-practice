use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn process_item(i: usize) {
    sleep(Duration::from_secs(1));
}

fn main() {
    let num_done = AtomicUsize::new(0);

    thread::scope(|s| {
        s.spawn(|| {
            for i in 0..100 {
                process_item(i);
                num_done.store(i + 1, Relaxed);
            }
        });

        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }
            println!("Working...{n}/100 done");
            sleep(Duration::from_secs(1));
        }
    });

    println!("Done!");
}
