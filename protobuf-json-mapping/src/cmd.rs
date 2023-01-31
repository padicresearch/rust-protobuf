use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::alpha1;
use nom::character::complete::alphanumeric0;
use nom::character::complete::char;
use nom::character::complete::one_of;
use nom::combinator::map_parser;
use nom::sequence::delimited;
use nom::IResult;

#[derive(Clone, Debug, thiserror::Error)]
pub enum CommandError {
    #[error("FailedToParse")]
    FailedToParse,
    #[error("FailedToParse {0}")]
    FailedToParseNom(String),
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Command<'a> {
    pub op: &'a str,
    pub data: &'a str,
}

pub type CommandHandler = dyn 'static + Send + Sync + Fn(&Command) -> Result<Vec<u8>, CommandError>;

pub(crate) fn parse_cmd_str(input: &str) -> Result<Command, CommandError> {
    match _parse_cmd_str(input) {
        Ok((remaining, cmd)) => {
            if !remaining.is_empty() {
                return Err(CommandError::FailedToParse);
            }
            Ok(cmd)
        }
        Err(e) => return Err(CommandError::FailedToParseNom(format!("{}", e))),
    }
}

fn _parse_cmd_str(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$")(input)?;
    let (input, op) = map_parser(take_until("("), alpha1)(input)?;
    let build_str = alt((
        delimited(one_of("\"\'"), alphanumeric0, one_of("\"\'")),
        alphanumeric0,
    ));
    let (input, data) = delimited(char('('), build_str, char(')'))(input)?;
    Ok((input, Command { op, data }))
}

#[cfg(test)]
mod tests {
    use crate::cmd::parse_cmd_str;

    #[test]
    fn test_parser_alpha() {
        let parsed_0 = parse_cmd_str("$hex(sdsahkjdhaskjhd)").unwrap();
        let parsed_1 = parse_cmd_str("$hex('sdsahkjdhaskjhd')").unwrap();
        let parsed_2 = parse_cmd_str(r#"$hex("sdsahkjdhaskjhd")"#).unwrap();
        assert_eq!(parsed_0, parsed_1);
        assert_eq!(parsed_1, parsed_2);
    }

    #[test]
    fn test_parser_numeric() {
        let parsed_0 = parse_cmd_str("$hex(1232323128129893218312)").unwrap();
        let parsed_1 = parse_cmd_str("$hex('1232323128129893218312')").unwrap();
        let parsed_2 = parse_cmd_str(r#"$hex("1232323128129893218312")"#).unwrap();
        assert_eq!(parsed_0, parsed_1);
        assert_eq!(parsed_1, parsed_2);
    }

    #[test]
    fn test_parser_alphanumeric() {
        let parsed_0 = parse_cmd_str("$hex(123232312uioeroqewiruowei8129893218312837738473w8hdsjhiaydUHJHSKHDHJShGydsayDBSJKHJ)").unwrap();
        let parsed_1 = parse_cmd_str("$hex('123232312uioeroqewiruowei8129893218312837738473w8hdsjhiaydUHJHSKHDHJShGydsayDBSJKHJ')").unwrap();
        let parsed_2 = parse_cmd_str(r#"$hex("123232312uioeroqewiruowei8129893218312837738473w8hdsjhiaydUHJHSKHDHJShGydsayDBSJKHJ")"#).unwrap();
        assert_eq!(parsed_0, parsed_1);
        assert_eq!(parsed_1, parsed_2);
    }
}
