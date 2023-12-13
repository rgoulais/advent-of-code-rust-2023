use std::ops::BitXor;
advent_of_code::solution!(13);

fn convert_string_to_binary(input: &str) -> usize {
    usize::from_str_radix(input, 2).unwrap()
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

fn is_truncated_reverse_of_each_other(list1: &[usize], list2: &[usize]) -> bool {
    if list1.len() == 0 || list2.len() == 0 {
        return false;
    }
    let mut list1_reversed = list1.to_vec();
    list1_reversed.reverse();
    if list1_reversed.len() <= list2.len() {
        let truncated_long_list = &list2[0..list1_reversed.len()];
        truncated_long_list == list1_reversed
    } else {
        let truncated_long_list = &list1_reversed[0..list2.len()];
        truncated_long_list == list2
    }
}

fn find_mirror_start_index(block: &[usize], original_index: usize) -> usize {
    for i in 1..block.len() {
        if i != original_index && is_truncated_reverse_of_each_other(&block[..i], &block[i..]) {
            return i;
        }
    }

    0
}

fn transpose_block(block: &Vec<String>) -> Vec<String> {
    let mut transposed = vec![String::new(); block[0].len()];

    for row in block {
        for (i, ch) in row.chars().enumerate() {
            transposed[i].push(ch);
        }
    }

    transposed
}

fn convert_string_blocks_to_int(block: &Vec<String>) -> Vec<usize> {
    return block.iter().map(|line| convert_string_to_binary(line)).collect();
}

fn get_numeric_blocks(input: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let binary_string = input.replace("#", "1").replace(".", "0");
    let blocks = parse_file_content(binary_string.as_str());
    let blocks_size = blocks.len();
    let mut h_blocks = Vec::with_capacity(blocks_size);
    let mut v_blocks = Vec::with_capacity(blocks_size);
    for block in blocks {
        h_blocks.push(convert_string_blocks_to_int(&block));
        v_blocks.push(convert_string_blocks_to_int(&transpose_block(&block)));
    }
    (h_blocks, v_blocks)
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut result = 0;
    let (h_blocks, v_blocks) = get_numeric_blocks(input);
    for i in 0..h_blocks.len() {
        result += 100 * find_mirror_start_index(&h_blocks[i], 0);
        result += find_mirror_start_index(&v_blocks[i], 0);
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (h_blocks, v_blocks) = get_numeric_blocks(input);
    let mut result = 0;
    for i in 0..h_blocks.len() {
        let h_len = v_blocks[i].len();
        let v_len = h_blocks[i].len();
        let ret = iterate_on_block(&h_blocks[i], h_len);
        if ret > 0 {
            result += 100 * ret;
        } else {
            result += iterate_on_block(&v_blocks[i], v_len);
        }
    }
    Some(result)
}


fn iterate_on_block(block: &Vec<usize>, row_len: usize) -> usize {
    let height = block.len();
    let original_index_h = find_mirror_start_index(&block, 0);
    for i in 0..height {
        for j in 0..row_len {
            let new_block = change_block_usize(block, i, j);
            let ret = find_mirror_start_index(&new_block, original_index_h);
            if ret > 0 {
                return ret;
            }
        }
    }
    return 0;
}

fn change_block_usize(block: &Vec<usize>, i: usize, j: usize) -> Vec<usize> {
    let mut new_block = block.clone();
    new_block[i] = new_block[i].bitxor(2_i32.pow(j as u32) as usize);
    new_block
}

#[cfg(test)]
mod tests {
    use super::*;


    mod tests {
        use super::*;

        #[test]
        fn parse_file_content_splits_blocks_correctly() {
            let input = "11100\n11100\n\n11100\n11100";
            let result = parse_file_content(input);
            assert_eq!(result, vec![vec!["11100", "11100"], vec!["11100", "11100"]]);
        }

        #[test]
        fn parse_file_content_handles_empty_input() {
            let input = "";
            let result = parse_file_content(input);
            assert_eq!(result, Vec::<Vec<String>>::new());
        }

        #[test]
        fn parse_file_content_handles_input_without_empty_lines() {
            let input = "11100\n11100";
            let result = parse_file_content(input);
            assert_eq!(result, vec![vec!["11100", "11100"]]);
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
        let result = convert_string_to_binary("11100");
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
        assert!(is_truncated_reverse_of_each_other(&[1, 2, 3], &[3, 2, 1]));
    }

    #[test]
    fn is_truncated_reverse_of_each_other_returns_true_for_truncated_reversed_lists() {
        assert!(is_truncated_reverse_of_each_other(&[1, 2, 3, 4], &[4, 3, 2]));
    }

    #[test]
    fn is_truncated_reverse_of_each_other_returns_false_for_non_reversed_lists() {
        assert!(!is_truncated_reverse_of_each_other(&[1, 2, 3], &[1, 2, 3]));
    }

    #[test]
    fn is_truncated_reverse_of_each_other_returns_false_for_two_values() {
        assert!(!is_truncated_reverse_of_each_other(&[3], &[0]));
    }

    #[test]
    fn transpose_block_returns_correct_transposition_for_non_empty_block() {
        let block = vec!["123".to_string(), "456".to_string(), "789".to_string()];
        let result = transpose_block(&block);
        assert_eq!(result, vec!["147".to_string(), "258".to_string(), "369".to_string()]);
    }

    #[test]
    fn transpose_block_returns_single_character_strings_for_single_row_block() {
        let block = vec!["123".to_string()];
        let result = transpose_block(&block);
        assert_eq!(result, vec!["1".to_string(), "2".to_string(), "3".to_string()]);
    }

    #[test]
    fn transpose_block_returns_single_string_for_single_column_block() {
        let block = vec!["1".to_string(), "2".to_string(), "3".to_string()];
        let result = transpose_block(&block);
        assert_eq!(result, vec!["123".to_string()]);
    }

    #[test]
    fn find_mirror_start_index_returns_correct_index_for_mirrored_block() {
        let block = vec![3, 2, 2, 3];
        let result = find_mirror_start_index(&block, 0);
        assert_eq!(result, 2);
    }

    #[test]
    fn find_mirror_start_index_returns_zero_for_non_mirrored_block() {
        let block = vec![3, 0];
        let result = find_mirror_start_index(&block, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn find_mirror_start_index_returns_correct_index_for_partially_mirrored_block_haut() {
        let block = vec![2, 3, 2, 2, 3];
        let result = find_mirror_start_index(&block, 0);
        assert_eq!(result, 3);
    }

    #[test]
    fn find_mirror_start_index_returns_correct_index_for_partially_mirrored_block_bas() {
        let block = vec![3, 2, 2, 3, 4];
        let result = find_mirror_start_index(&block, 0);
        assert_eq!(result, 2);
    }

    #[test]
    fn change_block_char_changes_hash_to_dot() {
        let block = vec![2, 3];
        let result = change_block_usize(&block, 0, 1);
        assert_eq!(result, vec![0, 3]);
    }

    #[test]
    fn change_block_usize_changes_dot_to_hash() {
        let block = vec![2, 3];
        let result = change_block_usize(&block, 0, 0);
        assert_eq!(result, vec![3, 3]);
    }

    #[test]
    fn change_block_usize_does_not_change_other_lines() {
        let block = vec![2, 3];
        let result = change_block_usize(&block, 0, 0);
        assert_eq!(result[1], block[1]);
    }

    #[test]
    fn iterate_on_block_returns_zero_for_non_mirrored_block() {
        let block = vec![3, 0];
        let result = iterate_on_block(&block, 2);
        assert_eq!(result, 0);
    }

    #[test]
    fn iterate_on_block_returns_correct_value_for_mirrored_block() {
        let block = vec![2, 2, 2, 3];
        let result = iterate_on_block(&block, 2);
        assert_eq!(result, 2);
    }
}