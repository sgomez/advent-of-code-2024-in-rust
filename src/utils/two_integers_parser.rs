use nom::character::complete::{digit1, multispace0};
use nom::combinator::map_res;
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

#[cfg(test)]
mod tests {
    use crate::utils::two_integers_parser::parse_two_integers;

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
}
