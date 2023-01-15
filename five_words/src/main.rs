use std::io;
fn main() {
    println!("Give me five words!");    
    let mut answers = Vec::new();
    
    for index in 0..5 {
        
        println!("word {}:", index + 1);
        
        let mut word = String::new();

        io::stdin().read_line(&mut word).expect("Something went wrong");

        answers.push(word);
    }
    println!("Nice one. Your words are:");
    
    for (i,word) in answers.iter().enumerate() {
        println!("Word {}: {word}", i + 1);
    }
}
