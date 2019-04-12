use std::thread;
use sha2::{Sha256,Digest};
use std::sync::{Arc,Mutex};
use std::fmt;
use std::ops::Deref;
use std::time::Duration;
use tiny_keccak::Keccak;
use itertools::Itertools;

struct my_string(String);

impl my_string {
    fn new(s: &str) -> my_string {
        my_string(s.to_string())
    }
}

impl fmt::Display for my_string {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


// impl fmt::LowerHex for my_string {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:x}", self.0) 
//     }
// }

// using sha2

// fn main() {
//     let paragraph = 
//     "Sed ut perspiciatis unde omnis iste natus error sit voluptatem.
//     accusantium doloremque laudantium, totam rem aperiam eaque ipsa.
//     quae ab illo inventore veritatis et quasi architecto beatae vitae.
//     dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit. 
//     aspernatur aut odit aut fugit, sed quia consequuntur magni dolores. 
//     eos qui ratione voluptatem sequi nesciunt. Neque porro quisquam est. 
//     qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit. 
//     sed quia non numquam eius modi tempora incidunt ut labore et dolore. 
//     magnam aliquam quaerat voluptatem. Ut enim ad minima veniam quis nostrum. 
//     exercitationem ullam corporis suscipit laboriosam, nisi ut aliquid ex.";

//     let split_sentences = paragraph.split_terminator('.');
//     let hash_value = Arc::new(Mutex::new(my_string::new("")));

//     for (i,sentence) in split_sentences.enumerate() {

//         let hash_value =  Arc::clone(&hash_value);

//         let handle = thread::spawn(move || {    
            
//             let mut hash  = hash_value.lock().unwrap();
//             let old_hash = &hash.0;
//             let hashing = Sha256::new()
//                 .chain(old_hash.to_string())
//                 .chain(sentence.to_string())
//                 .result();
//             println!("{:x}",hashing);    

//             hash.0 = format!("{:x}", hashing); 
//         });
//         handle.join().unwrap();

//     }
//     println!("Result: {}", hash_value.lock().unwrap());
// }


// using tiny-keccak sha-3

fn main() {
    let paragraph = 
    "Sed ut perspiciatis unde omnis iste natus error sit voluptatem.
    accusantium doloremque laudantium, totam rem aperiam eaque ipsa.
    quae ab illo inventore veritatis et quasi architecto beatae vitae.
    dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit. 
    aspernatur aut odit aut fugit, sed quia consequuntur magni dolores. 
    eos qui ratione voluptatem sequi nesciunt. Neque porro quisquam est. 
    qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit. 
    sed quia non numquam eius modi tempora incidunt ut labore et dolore. 
    magnam aliquam quaerat voluptatem. Ut enim ad minima veniam quis nostrum. 
    exercitationem ullam corporis suscipit laboriosam, nisi ut aliquid ex.";

    let split_sentences = paragraph.split_terminator('.');
    let hash_value = Arc::new(Mutex::new(my_string::new("")));

    for (i,sentence) in split_sentences.enumerate() {

        let hash_value =  Arc::clone(&hash_value);

        let handle = thread::spawn(move || {    
            
            let mut hash  = hash_value.lock().unwrap();
            let old_hash = &hash.0;

            let mut sha3 = Keccak::new_sha3_256();
            let data1: Vec<u8> = From::from(old_hash.to_string());
            let data2: Vec<u8> = From::from(sentence);

            sha3.update(&data1);
            sha3.update(&[b' ']);
            sha3.update(&data2);

            let mut res: [u8; 32] = [0; 32];
            sha3.finalize(&mut res);  

            println!("{:x}", res.iter().format(""));  

            hash.0 = format!("{:x}", res.iter().format("")); 
        });
        handle.join().unwrap();

    }
    println!("Result: {}", hash_value.lock().unwrap());
}
