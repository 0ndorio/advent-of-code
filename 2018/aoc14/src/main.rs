use std::error::Error;

type Result<ContentT> = std::result::Result<ContentT, Box<dyn Error>>;

fn main() -> Result<()> {
    let final_score = run_part1(286_051);
    println!("Final score for method one: {}", final_score);

    let num_recipes = run_part2("286051");
    println!("Final score for method two: {}", num_recipes);

    Ok(())
}

fn run_part1(num_recipes: usize) -> String {
    let mut first_elf_index = 0;
    let mut second_elf_index = 1;

    let mut scores = vec![3, 7];

    for _ in 0..(num_recipes + 10) {
        update_scores(&mut scores, &mut first_elf_index, &mut second_elf_index);
    }

    calc_part1_score(&scores, num_recipes)
}

fn run_part2(score_entry: &str) -> usize {
    let mut first_elf_index = 0;
    let mut second_elf_index = 1;

    let mut scores = vec![3, 7];
    let score_entry = score_entry
        .chars()
        .flat_map(|symbol| symbol.to_digit(10))
        .map(|digit| digit as usize)
        .collect::<Vec<_>>();

    while scores.len() < score_entry.len() {
        update_scores(&mut scores, &mut first_elf_index, &mut second_elf_index);
    }

    loop {
        update_scores(&mut scores, &mut first_elf_index, &mut second_elf_index);
        let board_entry_discovered = scores.ends_with(&score_entry);
        if board_entry_discovered {
            break scores.len() - score_entry.len();
        }
    }
}

fn update_scores(scores: &mut Vec<usize>, first_elf: &mut usize, second_elf: &mut usize) {
    let first_elf_score = scores[*first_elf];
    let second_elf_score = scores[*second_elf];

    let score = first_elf_score + second_elf_score;
    if score < 10 {
        scores.push(score);
    } else {
        scores.push(score / 10);
        scores.push(score % 10);
    }

    *first_elf += first_elf_score + 1;
    *first_elf %= scores.len();

    *second_elf += second_elf_score + 1;
    *second_elf %= scores.len();
}

fn calc_part1_score(recipe_scores: &[usize], last_recipe: usize) -> String {
    let from = last_recipe;
    let to = last_recipe + 10;

    recipe_scores[from..to]
        .iter()
        .map(ToString::to_string)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_part1() {
        let mut expected = HashMap::new();
        expected.insert(5, "0124515891");
        expected.insert(9, "5158916779");
        expected.insert(18, "9251071085");
        expected.insert(2018, "5941429882");

        expected.iter().for_each(|(amount, expected)| {
            let final_score = run_part1(*amount);
            assert_eq!(expected, &final_score)
        });
    }

    #[test]
    fn test_part2() {
        let mut expected = HashMap::new();
        expected.insert("01245", 5);
        expected.insert("51589", 9);
        expected.insert("92510", 18);
        expected.insert("59414", 2018);

        expected.iter().for_each(|(score_entry, expected)| {
            let final_score = run_part2(score_entry);
            assert_eq!(*expected, final_score)
        });
    }
}
