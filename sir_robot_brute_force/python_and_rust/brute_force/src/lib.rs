use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use itertools::iproduct;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

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

///Does a dictionary_attack against the passed in target
#[pyfunction]
fn dictionary_attack(path_to_dic: &str, target: &str) -> Option<String> {
    let mut result = None;

    let word_list = get_dict_list(path_to_dic)
        .expect(format!("Error opening dictionary at path '{}'", path_to_dic).as_str());

    //First pass through word list
    for word in word_list.iter() {
        if target.eq(word) {
            result = Some(word.to_string());
            break;
        }
    }

    if result == None {
        //Second pass through
        let word_list_clone = word_list.clone();

        for (word_1, word_2) in iproduct!(word_list, word_list_clone) {
            let mut phrase = word_1;
            phrase.push_str(word_2.as_str());

            if target.eq(phrase.as_str()) {
                result = Some(phrase);
                break;
            }
        }
    }

    return result;
}

#[pymodule]
fn brute_force(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(dictionary_attack))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{dictionary_attack, get_dict_list};

    #[test]
    fn test_get_dict_list() {
        let file = "passwords.txt";
        let result = get_dict_list(file).expect(&format!("verify that the {} file exists", file));

        assert_ne!(result.len(), 0);
    }
}
