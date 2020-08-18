use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::SystemTime;

use itertools::iproduct;

fn get_dict_list(path_to_dict: &str) -> std::io::Result<Vec<String>> {
    // Open File
    let file = File::open(path_to_dict)?;

    // Read contents as string
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    // create list of words
    let mut word_list = Vec::new();
    for word in contents.split("\n") {
        word_list.push(word.to_string());
    }

    Ok(word_list)
}

fn main() -> std::io::Result<()> {
    //let target = "123456seven"; // Can find
    let target = "123456Seven"; // Cannot find

    let mut count = 0;

    let now = SystemTime::now();

    let word_list = get_dict_list("passwords.txt")?;
    let total = (word_list.len() * word_list.len()) + word_list.len();

    // First pass through word list
    for word in word_list.iter() {
        count += 1;
        println!("Checking {} of {}", count, total);
        if target.eq(word) {
            println!("(count -- {}) Fount word: {}", count, word);
            break;
        }
    }

    let word_list_2 = word_list.clone();

    for (word_1, word_2) in iproduct!(word_list, word_list_2) {
        count += 1;
        println!("Checking {} of {}", count, total);

        let mut phrase = word_1;
        phrase.push_str(word_2.as_str());

        if target.eq(phrase.as_str()) {
            println!("(count -- {}) Fount word: {}", count, phrase);
            break;
        }
    }

    let elasped_time = now.elapsed().expect("Could't unwrap timer");
    println!("\n\nTime Elasped: {:?}", elasped_time);

    Ok(())
}
