use std::fs::File;
use std::io::prelude::*;
use std::time::SystemTime;

use rayon::prelude::*;

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

fn get_summary_iterator_parallel(contents: &str, target: &str, buffer: usize) -> Vec<String> {
    let sentences: Vec<&str> = contents.split("\n").map(|x| x.trim()).collect();

    let total_num_of_sentences = sentences.len();

    let result: Vec<String> = sentences
        .par_iter()
        .enumerate()
        .map(|(index, sentence)| (index, sentence))
        .filter(|(_, sentence)| sentence.contains(target))
        .map(|(index, _)| {
            let start = start_buffer(index as i32, buffer);
            let end = end_buffer(index, total_num_of_sentences, buffer);

            sentences[start..=end].join("\n")
        })
        .collect();

    result
}

fn get_summary_iterator(contents: &str, target: &str, buffer: usize) -> Vec<String> {
    let sentences: Vec<&str> = contents.split("\n").map(|x| x.trim()).collect();

    let total_num_of_sentences = sentences.len();

    let result: Vec<String> = sentences
        .iter()
        .enumerate()
        .map(|(index, sentence)| (index, sentence))
        .filter(|(_, sentence)| sentence.contains(target))
        .map(|(index, _)| {
            let start = start_buffer(index as i32, buffer);
            let end = end_buffer(index, total_num_of_sentences, buffer);

            sentences[start..=end].join("\n")
        })
        .collect();

    result
}

fn get_summary_faster<'a>(contents: &'a str, target: &str, buffer: usize) -> Vec<String> {
    let sentences: Vec<&str> = contents.split("\n").map(|x| x.trim()).collect();

    let mut result = Vec::new();
    let total_num_of_sentences = sentences.len();
    for (index, sentence) in sentences.iter().enumerate() {
        if sentence.contains(target) {
            let start = start_buffer(index as i32, buffer);
            let end = end_buffer(index, total_num_of_sentences, buffer);

            result.push(sentences[start..=end].join("\n"));
        }
    }

    result
}

fn get_summary(contents: String, target: String, buffer: usize) -> Vec<String> {
    let sentences: Vec<String> = contents.split("\n").map(|x| x.trim().to_string()).collect();

    let mut result = Vec::new();
    let total_num_of_sentences = sentences.len();
    for (index, sentence) in sentences.iter().enumerate() {
        if sentence.contains(target.as_str()) {
            println!("{}", sentence);

            let start = start_buffer(index as i32, buffer);
            let end = end_buffer(index, total_num_of_sentences, buffer);

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

    let result = get_summary_iterator_parallel(&contents, &target, buffer);

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
