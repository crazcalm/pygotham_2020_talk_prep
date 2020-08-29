use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

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

///Extracts lines surrounding target string in contents to create a summary.
#[pyfunction]
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

///Extracts lines surrounding target string in contents to create a summary.
#[pyfunction]
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

///Extracts lines surrounding target string in contents to create a summary.
#[pyfunction]
fn get_summary_references(contents: &str, target: &str, buffer: usize) -> Vec<String> {
    let sentences: Vec<&str> = contents.split("\n").map(|x| x.trim()).collect();

    let mut result = Vec::new();
    let total_num_of_sentence = sentences.len();
    for (index, sentence) in sentences.iter().enumerate() {
        if sentence.contains(target) {
            let start = start_buffer(index as i32, buffer);
            let end = end_buffer(index, total_num_of_sentence, buffer);

            result.push(sentences[start..=end].join("\n"));
        }
    }

    result
}

///Extracts lines surrounding target string in contents to create a summary.
#[pyfunction]
fn get_summary(contents: String, target: String, buffer: usize) -> Vec<String> {
    let sentences: Vec<String> = contents.split("\n").map(|x| x.trim().to_string()).collect();

    let mut result = Vec::new();
    let total_num_of_sentence = sentences.len();
    for (index, sentence) in sentences.iter().enumerate() {
        if sentence.contains(target.as_str()) {
            let start = start_buffer(index as i32, buffer);
            let end = end_buffer(index, total_num_of_sentence, buffer);

            result.push(sentences[start..=end].join("\n"));
        }
    }

    result
}

#[pymodule]
fn summary(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(get_summary))?;
    m.add_wrapped(wrap_pyfunction!(get_summary_references))?;
    m.add_wrapped(wrap_pyfunction!(get_summary_iterator))?;
    m.add_wrapped(wrap_pyfunction!(get_summary_iterator_parallel))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
