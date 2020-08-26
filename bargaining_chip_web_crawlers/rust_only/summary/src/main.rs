use std::fs::File;
use std::io::prelude::*;
use std::time::SystemTime;

fn start_buffer(num: i32, buf: usize) -> usize {
    let mut result = num - buf as i32;

    while result < 0 {
        result += 1;
    }

    result as usize
}

fn end_buffer(num: usize, limit: usize, buf: usize) -> usize {
    let mut result = num + buf;
    while result > limit {
        result -= 1;
    }

    result
}

fn get_summary(contents: String, target: String, buffer: usize) -> Vec<String> {
    let sentences: Vec<String> = contents.split("\n").map(|x| x.trim().to_string()).collect();

    let mut result = Vec::new();
    for (index, sentence) in sentences.clone().iter().enumerate() {
        if sentence.contains(target.as_str()) {
            println!("{}", sentence);

            let start = start_buffer(index as i32, buffer);
            let end = end_buffer(index, sentences.len(), buffer);

            result.push(sentences[start..=end].join("\n"))
        }
    }

    result
}

fn main() -> std::io::Result<()> {
    let now = SystemTime::now();

    let mut file = File::open("peter_pan.txt")?;
    let mut contents = String::new();
    let buffer = 2;

    let target = String::from("Peter");

    file.read_to_string(&mut contents)?;

    let result = get_summary(contents, target, buffer);

    for summary in result.iter() {
        println!("{}\n\n", summary)
    }

    let elasped_time = now.elapsed().expect("Couldn't unwrap timer");
    println!(
        "\n\nTimer elasped: {:?} in nano seconds",
        elasped_time.as_nanos()
    );

    Ok(())
}
