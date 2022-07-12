use std::collections::HashMap;
use std::io::stdin;

fn morsedecoder(morseinput: String) {
 
    // CLEAN MORSE INPUTS FROM ".-/-.../-.-." TO [".-", "-...", "-.-."]
    let cleanmorseinput: Vec<_> = morseinput
        .split('/')
        .collect();
    
    // CHECKPOINT 1: println!("{:?}", cleanmorseinput);
    // BUILDING MORSE CODE LIBRARY 
    let morsecode = vec![".-","-...","-.-.","-..",".",
    "..-.","--.","....","..",".---", "-.-",".-..","--","-.","---",
    ".--.","--.-",".-.","...","-", "..-","...-",".--","-..-","-.--",
    "--..", "..--.."];

    let alphabet = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
    'W', 'X', 'Y', 'Z','?']; 
    
        // MASS INSERT AND MATCH INTO HASHMAP
        let mut morsecodemap: HashMap<&str, Option<&char>> = HashMap::new();
            let mut x = 0;
            for eachcode in &morsecode{
                // for loop prints out all Z
                // use vector.get method
                morsecodemap.insert(eachcode, alphabet.get(x));
                x += 1;
                }
    
    // CHECKPOINT 2: println!("{:?}", morsecodemap);
    // DECLARE VEC FOR DECODED OUTPUT
    let mut output: Vec<_> = Vec::new();

        // MATCH INPUTS TO MORSE CODE LIBRARY
        for eachcode in cleanmorseinput{
            match morsecodemap.get(eachcode){
                Some(eachletter) => output.push(eachletter.unwrap()),
                None => (),
            }
              
        }

    println!("{:?}", output);
     
}


fn main() {
   
    // TITLE
    println!("-- rust project june 2022 --");
    println!("-- rui xin oh --");
    println!("");

    // SCRIPT
    println!("Chapter Ten: Lost Sheep");
    println!("");

    println!("Splintered apart, Hawkins lies quiet in the chaos and mourning of all.");
    println!("Though valorous and united, the gang goes astray with each death it faces.");
    println!("");
    
    println!("INT. CREEL HOUSE - DAY");
    println!("");
    println!("Lamp light in the attic flickers intermittently. Camera tracks forward with foreboding music.");
    println!("");

    println!("...-/");
    println!("..../.-/.../");
    println!("--/./");
    println!("");

    // TAKE IN 3 USER INPUTS
    let mut attempts = 1;
    while attempts !=4{

        println!("ENTER MORSE CODE [{}/3]", attempts);
       
        let mut userinput = String::new();
        stdin().read_line(&mut userinput)
    	    .ok()
            .expect("Failed to read line");
        println!("");

        println!("Translating in progress {}", userinput);
        morsedecoder(userinput);
        println!("");

        attempts += 1;
    }

println!("Lamp jerks violently to the side and shatters. Ravenous flames emerge, relentlessly engulfing dry attic wood.");
println!("Camera zooms out to a Creel House crumbling into a fissure between two worlds.");
println!("");

println!("-- to be continued --");
println!("-- special thanks: Kevin & Vincent --");
    // TEST
     // let x = "-.-./.-/-.../..--..".to_string();
     // println!("You've entered {}", x);
     // println!("Decoding...");
     // morsedecoder(x);

}