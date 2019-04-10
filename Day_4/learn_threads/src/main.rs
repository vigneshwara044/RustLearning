use std::thread;
use std::time::Duration;

// This runs properly one after the other. 

// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     handle.join().unwrap();

//     for i in 1..10 {
//         println!("hi number {} from the main thread!", i);
//         thread::sleep(Duration::from_millis(1));
//     }
// }


// This causes error as the closure tries to borrow v but compiler doesn't know when the thread will run.

// fn main() {
//     let v = vec![1, 2, 3];

//     let handle = thread::spawn(|| {
//         println!("Here's a vector: {:?}", v);
//     });

//     handle.join().unwrap();
// }


// This causes an error as v might be dropped even before the child starts. 

// fn main() {
//     let v = vec![1, 2, 3];

//     let handle = thread::spawn(|| {
//         println!("Here's a vector: {:?}", v);
//     });

//     drop(v); // oh no!

//     handle.join().unwrap();
// }

// This works.
fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}