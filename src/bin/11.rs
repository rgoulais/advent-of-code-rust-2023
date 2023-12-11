advent_of_code::solution!(11);

fn part_all(input_str: &str, galaxy_age: usize) -> Option<usize> {
    let mut effective_row = 0;
    let mut last_row = 0;
    let mut coordinates_to_value: Vec<(usize, usize)> = vec![];

    for (i, row) in input_str.lines().enumerate() {
        for (j, item) in row.chars().enumerate() {
            if item == '#' {
                    let ecart_row = i - last_row;
                    last_row = i;
                    if ecart_row > 1 {
                        effective_row += 1 + (galaxy_age * (ecart_row - 1));
                    } else {
                        effective_row += ecart_row;
                    }
                coordinates_to_value.push((j, effective_row));
            }
        }
    }
    coordinates_to_value.sort_by_key(|&(first, _)| first);

    let (mut effective_col, mut last_col) = (0usize, 0usize);
    let (mut coords, mut total) = (vec![], 0usize);

    for (new_col, row) in coordinates_to_value.iter() {
        let ecart_col = new_col - last_col;
        last_col = *new_col;
        if ecart_col > 1 {
            effective_col += 1 + (galaxy_age * (ecart_col - 1));
        } else {
            effective_col += ecart_col;
        }
        let new_coord = (*row, effective_col);
        for &(r, c) in coords.iter() {
            total += new_coord.0.abs_diff(r) + new_coord.1.abs_diff(c)
        }
        coords.push(new_coord);
    }
    Some(total)
}

pub fn part_one(input: &str) -> Option<usize> {
    return part_all(input, 2);
}

pub fn part_two(input: &str) -> Option<usize> {
    return part_all(input, 1000000);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let result = part_all(&advent_of_code::template::read_file("examples", DAY), 2);
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_two() {
        let result = part_all(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(1030));
    }

    #[test]
    fn test_three() {
        let result = part_all(&advent_of_code::template::read_file("examples", DAY), 100);
        assert_eq!(result, Some(8410));
    }
}
