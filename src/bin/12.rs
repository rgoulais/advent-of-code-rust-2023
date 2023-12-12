advent_of_code::solution!(12);

use cached::proc_macro::cached;

pub fn split_line(input: &str) -> (String, Vec<usize>) {
    let mut lines = input.split(' ');
    let first = lines.next().unwrap();

    let mut result = String::new();
    let mut prev_char = '\0';

    for char in first.chars() {
        if char != '.' || prev_char != '.' {
            result.push(char);
        }
        prev_char = char;
    }

    let second = lines.next().unwrap();
    let numbers: Result<Vec<usize>, _> = second.split(',')
        .map(|s| s.parse::<usize>())
        .collect();
    match numbers {
        Ok(nums) => (result, nums),
        Err(_e) => (result, vec![]),
    }
}


#[cached]
pub fn count_match(pattern: String, list: Vec<usize>) -> usize {
    let pattern = pattern.trim_start_matches('.');
    if list.len() == 0 {
        if pattern.contains('#') {
            return 0;
        }
        return 1;
    } else if pattern.len() == 0 {
        return 0;
    }

    if list.iter().sum::<usize>() + (list.len() - 1) > pattern.len() {
        return 0;
    }
    let first = list[0];
    let rest = &list[1..];
    if pattern.starts_with('#') {
        let chars = pattern.chars()
            .take_while(|&c| c == '#' || c == '?')
            .count();
        if chars < first {
            return 0;
        }
        let mut nouveau_texte: String = pattern.chars().skip(first).collect();
        if nouveau_texte.len() == 0 {
            if rest.len() == 0 {
                return 1;
            } else {
                return 0;
            }
        } else {
            if nouveau_texte.starts_with('#') {
                return 0;
            }
        }
        nouveau_texte = nouveau_texte.chars().skip(1).collect();
        return count_match(nouveau_texte, rest.to_vec());
    } else {
        let mut text1 = String::from(pattern);
        text1.replace_range(0..1, "#");
        let mut text2 = String::from(pattern);
        text2.replace_range(0..1, ".");
        let c1 = count_match(text1, list.clone());
        let c2 = count_match(text2, list);
        return c1 + c2;
    }
}


fn multicount_match(line: &str, count: i32) -> usize {
    let (pattern, list) = split_line(line);
    let mut repeated_list = Vec::new();
    for _ in 0..count {
        for item in &list {
            repeated_list.push(*item);
        }
    }
    let mut repeated_pattern = pattern.clone();
    for _ in 1..count {
        repeated_pattern += "?";
        repeated_pattern += pattern.as_str();
    }
    count_match(repeated_pattern, repeated_list)
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut result = 0;
    let lines = input.split('\n');
    for line in lines {
        let line_clone = line.to_string();
        let (pattern, list) = split_line(&line_clone);
        result += count_match(pattern, list);
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut result = 0;
    let lines = input.split('\n');
    for line in lines {
        let line_clone = line.to_string();
        result += multicount_match(&line_clone, 5)
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_count_match1() {
        let result = count_match("#.###".to_string(), [1, 3].to_vec());
        assert_eq!(result, 1);
    }

    #[test]
    fn test_count_match2() {
        let result = count_match("???".to_string(), [1, 2].to_vec());
        assert_eq!(result, 0);
    }

    #[test]
    fn test_split_line3() {
        let result = count_match(".???#?..???#??.?.".to_string(), vec![1, 4]);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_multicount_match0() {
        let result = count_match("???.###????.###".to_string(), vec![1, 1, 3, 1, 1, 3]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_multicount_match1() {
        let result = multicount_match("???.### 1,1,3", 2);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_multicount_match2() {
        let result = multicount_match("????.#...#... 4,1,1", 5);
        assert_eq!(result, 16);
    }

    #[test]
    fn test_multicount_match3() {
        let result = multicount_match(".??..??...?##. 1,1,3", 5);
        assert_eq!(result, 16384);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }
}
