use std::thread;

fn main() {
    let numbers = Vec::from_iter(0..=1000);

    let h = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.into_iter().sum::<usize>();
        sum / len
    });

    let average = h.join().unwrap();

    println!("average: {average}");
}
