use std::io;
fn main() {
    println!("Give me five words!");
    let mut index: i8 = 0;
    
    let mut answers =Vec::new();
    
    while index < 5 {
        
        println!("word {}:", index + 1);
        
        let mut word = String::new();

        io::stdin().read_line(&mut word).expect("Something went wrong");

        answers.push(word);

        index = index + 1
    }
    println!("Nice one. Your words are:");
    
    for word in answers {
        println!("{word}");
    }
}
