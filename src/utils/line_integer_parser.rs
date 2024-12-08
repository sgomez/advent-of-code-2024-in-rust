use nom::character::complete::{digit1, multispace0, multispace1};
use nom::combinator::map_res;
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::IResult;

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

#[cfg(test)]
mod tests {
    use crate::utils::line_integer_parser::{parse_line_of_integers, parse_two_integers};

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
}
