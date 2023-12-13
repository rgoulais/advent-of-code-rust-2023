advent_of_code::solution!(13);

fn convert_string_to_binary(input: &str) -> u32 {
    let binary_string = input.replace("#", "1").replace(".", "0");
    u32::from_str_radix(&binary_string, 2).unwrap()
}

fn parse_file_content(input: &str) -> Vec<Vec<String>> {
    let mut blocks = Vec::new();
    let mut current_block = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            if !current_block.is_empty() {
                blocks.push(current_block);
                current_block = Vec::new();
            }
        } else {
            current_block.push(line.to_string());
        }
    }

    if !current_block.is_empty() {
        blocks.push(current_block);
    }

    blocks
}

fn is_truncated_reverse_of_each_other(list1: Vec<u32>, list2: Vec<u32>) -> bool {
    if list1.len() == 0 || list2.len() == 0 {
        return false;
    }
    let mut list1_reversed = list1;
    list1_reversed.reverse();
    let (short_list, long_list) = if list1_reversed.len() <= list2.len() {
        (list1_reversed, list2)
    } else {
        (list2, list1_reversed)
    };

    let truncated_long_list = &long_list[0..short_list.len()];
    truncated_long_list == short_list
}

fn find_mirror_start_index(block: Vec<String>, original_index: usize) -> usize {
    let binary_values: Vec<u32> = block.iter().map(|line| convert_string_to_binary(line)).collect();

    for i in 1..binary_values.len() {
        if i != original_index && is_truncated_reverse_of_each_other(binary_values[..i].to_vec(), binary_values[i..].to_vec()) {
            return i;
        }
    }

    0
}

fn transpose_block(block: Vec<String>) -> Vec<String> {
    let mut transposed = vec![String::new(); block[0].len()];

    for row in block {
        for (i, ch) in row.chars().enumerate() {
            transposed[i].push(ch);
        }
    }

    transposed
}

fn find_mirror_start_index_in_columns(block: Vec<String>) -> usize {
    let transposed_block = transpose_block(block);
    find_mirror_start_index(transposed_block, 0)
}

pub fn part_one(input: &str) -> Option<usize> {
    let blocks = parse_file_content(input);
    let mut result = 0;

    for block in blocks {
        result += 100 * find_mirror_start_index(block.clone(), 0);
        result += find_mirror_start_index_in_columns(block);
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let blocks = parse_file_content(input);
    let mut result = 0;

    for block in blocks {
        let ret = iterate_on_block(&block);
        result += ret;
    }
    Some(result)
}


fn iterate_on_block(block: &Vec<String>) -> usize {
    let height = block.len();
    let width = block[0].len();
    let transposed_block = transpose_block(block.clone());
    let original_index_h = find_mirror_start_index(block.clone(), 0);
    let original_index_v = find_mirror_start_index(transposed_block, 0);
    for i in 0..height {
        for j in 0..width {
            let new_block = change_block_char(&block, i, j);
            let new_transposed_block = transpose_block(new_block.clone());
            let ret = find_mirror_start_index(new_block, original_index_h);
            let ret2 = find_mirror_start_index(new_transposed_block, original_index_v);
            if ret > 0 {
                return 100 * ret;
            }
            if ret2 > 0 {
                return ret2;
            }
        }
    }
    return 0;
}

fn change_block_char(block: &Vec<String>, i: usize, j: usize) -> Vec<String> {
    let mut new_block = block.clone();
    let mut new_line = new_block[i].clone();
    let ch = new_line.chars().nth(j).unwrap();
    let new_ch = if ch == '#' { '.' } else { '#' };
    new_line.replace_range(j..j + 1, &new_ch.to_string());
    new_block[i] = new_line;
    new_block
}

#[cfg(test)]
mod tests {
    use super::*;


    mod tests {
        use super::*;

        #[test]
        fn parse_file_content_splits_blocks_correctly() {
            let input = "###..\n###..\n\n###..\n###..";
            let result = parse_file_content(input);
            assert_eq!(result, vec![vec!["###..", "###.."], vec!["###..", "###.."]]);
        }

        #[test]
        fn parse_file_content_handles_empty_input() {
            let input = "";
            let result = parse_file_content(input);
            assert_eq!(result, Vec::<Vec<String>>::new());
        }

        #[test]
        fn parse_file_content_handles_input_without_empty_lines() {
            let input = "###..\n###..";
            let result = parse_file_content(input);
            assert_eq!(result, vec![vec!["###..", "###.."]]);
        }

        #[test]
        fn parse_file_content_handles_input_with_only_empty_lines() {
            let input = "\n\n\n";
            let result = parse_file_content(input);
            assert_eq!(result, Vec::<Vec<String>>::new());
        }
    }

    #[test]
    fn test_convert_string_to_binary() {
        let result = convert_string_to_binary("###..");
        assert_eq!(result, 28);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }

    #[test]
    fn is_truncated_reverse_of_each_other_returns_true_for_reversed_lists() {
        assert!(is_truncated_reverse_of_each_other(vec![1, 2, 3], vec![3, 2, 1]));
    }

    #[test]
    fn is_truncated_reverse_of_each_other_returns_true_for_truncated_reversed_lists() {
        assert!(is_truncated_reverse_of_each_other(vec![1, 2, 3, 4], vec![4, 3, 2]));
    }

    #[test]
    fn is_truncated_reverse_of_each_other_returns_false_for_non_reversed_lists() {
        assert!(!is_truncated_reverse_of_each_other(vec![1, 2, 3], vec![1, 2, 3]));
    }

    #[test]
    fn transpose_block_returns_correct_transposition_for_non_empty_block() {
        let block = vec!["123".to_string(), "456".to_string(), "789".to_string()];
        let result = transpose_block(block);
        assert_eq!(result, vec!["147".to_string(), "258".to_string(), "369".to_string()]);
    }

    #[test]
    fn transpose_block_returns_single_character_strings_for_single_row_block() {
        let block = vec!["123".to_string()];
        let result = transpose_block(block);
        assert_eq!(result, vec!["1".to_string(), "2".to_string(), "3".to_string()]);
    }

    #[test]
    fn transpose_block_returns_single_string_for_single_column_block() {
        let block = vec!["1".to_string(), "2".to_string(), "3".to_string()];
        let result = transpose_block(block);
        assert_eq!(result, vec!["123".to_string()]);
    }

    #[test]
    fn find_mirror_start_index_returns_correct_index_for_mirrored_block() {
        let block = vec!["##".to_string(), "#.".to_string(), "#.".to_string(), "##".to_string()];
        let result = find_mirror_start_index(block, 0);
        assert_eq!(result, 2);
    }

    #[test]
    fn find_mirror_start_index_returns_zero_for_non_mirrored_block() {
        let block = vec!["##".to_string(), "..".to_string()];
        let result = find_mirror_start_index(block, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn find_mirror_start_index_returns_correct_index_for_partially_mirrored_block_haut() {
        let block = vec!["#.".to_string(), "##".to_string(), "#.".to_string(), "#.".to_string(), "##".to_string()];
        let result = find_mirror_start_index(block, 0);
        assert_eq!(result, 3);
    }

    #[test]
    fn find_mirror_start_index_returns_correct_index_for_partially_mirrored_block_bas() {
        let block = vec!["##".to_string(), "#.".to_string(), "#.".to_string(), "##".to_string(), "##".to_string()];
        let result = find_mirror_start_index(block, 0);
        assert_eq!(result, 2);
    }

    #[test]
    fn change_block_char_changes_hash_to_dot() {
        let block = vec!["#.".to_string(), "##".to_string()];
        let result = change_block_char(&block, 0, 0);
        assert_eq!(result, vec!["..".to_string(), "##".to_string()]);
    }

    #[test]
    fn change_block_char_changes_dot_to_hash() {
        let block = vec!["#.".to_string(), "##".to_string()];
        let result = change_block_char(&block, 0, 1);
        assert_eq!(result, vec!["##".to_string(), "##".to_string()]);
    }

    #[test]
    fn change_block_char_does_not_change_other_lines() {
        let block = vec!["#.".to_string(), "##".to_string()];
        let result = change_block_char(&block, 0, 0);
        assert_eq!(result[1], block[1]);
    }

    #[test]
    fn change_block_char_does_not_change_other_characters_in_same_line() {
        let block = vec!["#.".to_string(), "##".to_string()];
        let result = change_block_char(&block, 0, 0);
        assert_eq!(result[0].chars().nth(1), block[0].chars().nth(1));
    }

    #[test]
    fn iterate_on_block_returns_zero_for_non_mirrored_block() {
        let block = vec!["##".to_string(), "..".to_string()];
        let result = iterate_on_block(&block);
        assert_eq!(result, 0);
    }

    #[test]
    fn iterate_on_block_returns_correct_value_for_mirrored_block() {
        let block = vec!["#.".to_string(), "#.".to_string(), "#.".to_string(), "##".to_string()];
        let result = iterate_on_block(&block);
        assert_eq!(result, 200);
    }
}