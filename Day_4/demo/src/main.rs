use std::thread;
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
    
    
    let mut kids = vec![];

    let split_sentences = paragraph.split_terminator('.');

    for (i,sentence) in split_sentences.enumerate() {
        println!("For sentence no.{} and sentence {}", i+1, sentence);

        kids.push(thread::spawn(move || {
            let uppercase_sentence = sentence.to_uppercase();
            let length = sentence.len();
            println!("{}", uppercase_sentence);

            length
        }));
    }

    let mut sum = 0;

    for kid in kids {
        let x = kid.join().unwrap();
        sum = sum + x;
    }

    println!("The no of characters is {}", sum);
    
}
