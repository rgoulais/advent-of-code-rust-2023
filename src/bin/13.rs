use std::ops::BitXor;
advent_of_code::solution!(13);

struct Block {
    data: Vec<usize>,
    width: usize,
}

impl Block {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            width: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn push(&mut self, value: usize, strlen: usize) {
        self.data.push(value);
        if self.width == 0 {
            self.width = strlen;
        }
    }

    fn transpose_block(&self) -> Block {
        let mut transposed = Block::new();
        transposed.width = self.data.len();

        for _ in 0..self.width {
            transposed.data.push(0);
        }

        for i in 0..self.width {
            let pow = 2_u32.pow((self.width - 1 - i) as u32) as usize;
            for j in 0..self.data.len() {
                if self.data[j] & pow == pow {
                    transposed.data[i] += 2_u32.pow((transposed.width - j - 1) as u32) as usize;
                }
            }
        }

        transposed
    }
}

fn parse_file_content(input: &str) -> Vec<Block> {
    let mut blocks = Vec::new();
    let mut current_block = Block::new();

    for line in input.lines() {
        if line.is_empty() {
            if !current_block.is_empty() {
                blocks.push(current_block);
                current_block = Block::new();
            }
        } else {
            current_block.push(usize::from_str_radix(line, 2).unwrap(), line.len());
        }
    }

    if !current_block.is_empty() {
        blocks.push(current_block);
    }

    blocks
}

fn is_truncated_reverse_of_each_other(list1: &[usize], list2: &[usize]) -> bool {
    if list1.is_empty() || list2.is_empty() {
        return false;
    }
    let len = list1.len() - 1;
    let shortlen = list1.len().min(list2.len());

    for i in 0..shortlen {
        if list1[len - i] != list2[i] {
            return false;
        }
    }
    true
}

fn find_mirror_start_index(block: &[usize], original_index: usize) -> usize {
    for i in 1..block.len() {
        if i != original_index && is_truncated_reverse_of_each_other(&block[..i], &block[i..]) {
            return i;
        }
    }

    0
}

fn get_numeric_blocks(input: &str) -> (Vec<Block>, Vec<Block>) {
    let binary_string = input.replace('#', "1").replace('.', "0");
    let blocks = parse_file_content(binary_string.as_str());
    let v_blocks = blocks.iter().map(|b| b.transpose_block()).collect();
    (blocks, v_blocks)
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut result = 0;
    let (h_blocks, v_blocks) = get_numeric_blocks(input);
    for i in 0..h_blocks.len() {
        let ret = find_mirror_start_index(&v_blocks[i].data, 0);
        if ret > 0 {
            result += ret;
        } else {
            result += 100 * find_mirror_start_index(&h_blocks[i].data, 0);
        };
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (h_blocks, v_blocks) = get_numeric_blocks(input);
    let mut result = 0;
    for i in 0..h_blocks.len() {
        let ret = iterate_on_block(&v_blocks[i]);
        if ret > 0 {
            result += ret;
        } else {
            result += 100 * iterate_on_block(&h_blocks[i]);
        }
    }
    Some(result)
}

fn iterate_on_block(block: &Block) -> usize {
    let original_index_h = find_mirror_start_index(&block.data, 0);
    for i in 0..block.data.len() {
        for j in 0..block.width {
            let new_block = change_block_usize(&block.data, i, j);
            let ret = find_mirror_start_index(&new_block, original_index_h);
            if ret > 0 {
                return ret;
            }
        }
    }
    0
}

fn change_block_usize(block: &[usize], i: usize, j: usize) -> Vec<usize> {
    let mut new_block = block.to_vec();
    new_block[i] = new_block[i].bitxor(2_i32.pow(j as u32) as usize);
    new_block
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn parse_file_content_splits_blocks_correctly() {
        let input = "11100\n11100\n\n11100\n11100";
        let result = parse_file_content(input);
        assert_eq!(result[0].data, [28, 28]);
        assert_eq!(result[1].data, [28, 28]);
    }

    #[test]
    fn parse_file_content_handles_input_without_empty_lines() {
        let input = "11100\n11100";
        let result = parse_file_content(input);
        assert_eq!(result[0].data, [28, 28]);
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
        assert!(is_truncated_reverse_of_each_other(
            &[1, 2, 3, 4],
            &[4, 3, 2],
        ));
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
        let mut block = Block::new();
        block.push(128, 8);
        block.push(0, 8);
        let result = block.transpose_block();
        assert_eq!(
            result.data,
            [2, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn transpose_block_returns_single_character_strings_for_single_row_block() {
        let mut block = Block::new();
        block.push(7, 3);
        let result = block.transpose_block();
        assert_eq!(
            result.data,
            [1, 1, 1]
        );
    }

    #[test]
    fn transpose_block_returns_single_string_for_single_column_block() {
        let mut block = Block::new();
        block.push(1, 1);
        block.push(1, 1);
        block.push(1, 1);
        let result = block.transpose_block();
        assert_eq!(
            result.data,
            [7]
        );
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
        let mut block = Block::new();
        block.data =vec![3, 0];
        block.width = 2;
        let result = iterate_on_block(&block);
        assert_eq!(result, 0);
    }

    #[test]
    fn iterate_on_block_returns_correct_value_for_mirrored_block() {
        let mut block = Block::new();
        block.data =vec![2, 2, 2, 3];
        block.width = 2;
        let result = iterate_on_block(&block);
        assert_eq!(result, 2);
    }
}
