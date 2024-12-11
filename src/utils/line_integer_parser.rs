use nom::{
    branch::alt,
    character::complete::{char, digit1, multispace0, multispace1, one_of},
    combinator::{map, map_res, value},
    multi::{many1, separated_list0},
    sequence::tuple,
    IResult,
};

fn str_to_i32(input: &str) -> Result<i32, std::num::ParseIntError> {
    input.parse::<i32>()
}

pub fn parse_two_integers(input: &str) -> IResult<&str, (i32, i32)> {
    let (input, _) = multispace0(input)?;
    let (input, (left, _, right)) = tuple((
        map_res(digit1, str_to_i32),
        multispace0,
        map_res(digit1, str_to_i32),
    ))(input)?;

    Ok((input, (left, right)))
}

pub fn parse_line_of_integers(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list0(multispace1, map_res(digit1, str_to_i32))(input)
}

pub fn parse_levels(input: &str) -> IResult<&str, Vec<Option<i32>>> {
    let parse_digit = map(one_of("0123456789"), |c: char| {
        Some(c.to_digit(10).unwrap() as i32)
    });
    let parse_dot = value(None, char('.'));
    let parse_element = alt((parse_digit, parse_dot));

    many1(parse_element)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_integers_parser() {
        // Arrange
        let input = "10    100";
        // Act
        let (_, (left, right)) = parse_two_integers(&input).unwrap();
        // Assert
        assert_eq!(left, 10);
        assert_eq!(right, 100);
    }

    #[test]
    fn test_parse_line_of_integers() {
        // Arrange
        let input = "1 2 3 4 5";
        // Act
        let (_, result) = parse_line_of_integers(&input).unwrap();
        // Arrange
        assert_eq!(result, vec![1, 2, 3, 4, 5])
    }

    #[test]
    fn test_parse_levels() {
        // Arrange
        let input = "1.345";
        // Act
        let (_, result) = parse_levels(&input).unwrap();
        // Arrange
        assert_eq!(result, vec![Some(1), None, Some(3), Some(4), Some(5)])
    }
}
