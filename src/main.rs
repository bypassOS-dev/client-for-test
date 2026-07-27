use std::thread;
use std::time::Duration;

fn main() {
    let cool = vec!["Rust".to_string(), "is".to_string(), "cool".to_string()];
    let thread1 = thread::spawn(move || {
        let mut sum = 0;
        for i in 1..=10000 {
            sum += i;
            println!("{i}");
            thread::sleep(Duration::from_millis(1));
        }
        println!("{}(1)",sum);
        println!("{:?}",cool);
    });
    let thread2 = thread::spawn(|| {
        let mut sum = 0;
        for i in 1..=10000 {
            sum += i;
            println!("{i}");
            thread::sleep(Duration::from_millis(1));
        }
        println!("{}(2)",sum);
    });
    let thread3 = thread::spawn(|| {
        let mut sum = 0;
        for i in 1..=10000 {
            sum += i;
            println!("{i}");
            thread::sleep(Duration::from_millis(1));
        }
        println!("{}(3)",sum);
    });
    thread1.join().unwrap();
    thread2.join().unwrap();
    thread3.join().unwrap();
}