use std::{
    error::Error,
    io::{self, Read},
};

type Result<ContentT> = std::result::Result<ContentT, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let checksum = calc_checksum(&input)?;
    println!("Boxes checksum: {}", checksum);

    let common_id = find_common_box_id(&input)?;
    println!("Common box id: {}", common_id);

    Ok(())
}

fn calc_checksum(input: &str) -> Result<u32> {
    let ids: Vec<String> = input.split('\n').map(String::from).collect();
    let mut num_doubles = 0;
    let mut num_triples = 0;

    for id in ids {
        let mut contains_double = false;
        let mut contains_triple = false;

        for char in id.chars() {
            let num = id.chars().filter(|current| current == &char).count();
            match num {
                2 => contains_double = true,
                3 => contains_triple = true,
                _ => (),
            };
        }

        if contains_double {
            num_doubles += 1;
        }

        if contains_triple {
            num_triples += 1;
        }
    }

    Ok(num_doubles * num_triples)
}

fn find_common_box_id(input: &str) -> Result<String> {
    let ids: Vec<&str> = input.split('\n').collect();
    let num_ids = ids.len();

    for word_index in 0..num_ids {
        for candidate_word_index in word_index + 1..num_ids {
            let word = ids[word_index];
            let candidate_word = ids[candidate_word_index];

            let common_id = find_id_with_single_letter_difference(word, candidate_word);
            if common_id.is_ok() {
                return common_id;
            }
        }
    }

    Err("Couldn't find any common ids.".into())
}

fn find_id_with_single_letter_difference(word: &str, candidate_word: &str) -> Result<String> {
    let zipped_chars = word.chars().zip(candidate_word.chars());
    let mut found_wrong = false;

    for (first_char, second_char) in zipped_chars.clone() {
        if first_char != second_char {
            if found_wrong {
                return Err("Difference bigger than one letter".into());
            }

            found_wrong = true;
        }
    }

    let common_id: String = zipped_chars
        .filter(|(first_char, second_char)| first_char == second_char)
        .map(|(first_char, _)| first_char)
        .collect();

    Ok(common_id)
}
