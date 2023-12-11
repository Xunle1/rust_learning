use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {

    // ---------- use thread ----------
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(100));
    //     }
    // });
    //
    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(100));
    // }
    //
    // handle.join().unwrap();

    // ---------- use move ----------
    // let v = vec![1, 2, 3];
    // let handle = thread::spawn(move || {
    //     println!("vector: {:?}", v);
    // });
    //
    // handle.join().unwrap();

    // ----------
    let (tx, rx) = mpsc::channel();


}
