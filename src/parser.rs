use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::none_of,
    sequence::delimited,
    combinator::map,
    multi::many1,
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
    Comment(String),
    Str(String),
    Int(i32),
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Equal,
    LargerThan,
    Dup,
    Swap,
    Cjump,
    Print,
    Newline,
}

impl Token {
    fn string(s: &str) -> IResult<&str, Token> {
        map(
            delimited(tag("~"), many1(none_of("~")), tag("~")),
            |res: Vec<char>| Self::Str(res.into_iter().collect())
        )(s)
    }
    fn comment(s: &str) -> IResult<&str, Token> {
        map(
            delimited(tag("("), many1(none_of(")")), tag(")")),
            |res: Vec<char>| Self::Comment(res.into_iter().collect())
        )(s)
    }
    fn any_op(s: &str) -> IResult<&str, Token> {
        macro_rules! op {
            ($name: literal, $tok: expr) => { map(tag($name), |_| $tok) }
        }
        alt((
            op!(".+", Self::Add),
            op!(".-", Self::Sub),
            op!(".*", Self::Mul),
            op!("./", Self::Div),
            op!(".mod", Self::Mod),
            op!(".=?", Self::Equal),
            op!(".>?", Self::LargerThan),
            op!(".dup", Self::Dup),
            op!(".swap", Self::Swap),
            op!(".cjump", Self::Cjump),
            op!(".print", Self::Print),
            op!(".newline", Self::Newline),

        ))(s)
    }
    fn parse(s: &str) -> IResult<&str, Self> {
        nom::sequence::terminated(
            alt((
                Self::comment,
                Self::string,
                Self::any_op,
                map( nom::character::complete::i32, |val| Self::Int(val) ),
            )),
            nom::character::complete::multispace0,
        )(s)
    }
}

pub fn parse(s: &str) -> Vec<Token> {
    nom::multi::many0(Token::parse)(s.trim()).unwrap().1
}

pub fn parse_ignore_comments(s: &str) -> Vec<Token> {
    parse(s).into_iter().filter(|tok| !matches!(tok, Token::Comment(_))).collect()
}

#[cfg(test)]
mod tests {
    use crate::parser::parse_ignore_comments;
    use crate::parser::Token;

    static hello: &'static str = "(this is a comment, you can tell because it is enclosed in parens.
 the program below pushes hello world to stack, prints top of stack,
 and prints a line break)
~Hello, world!~ (this is an inline comment) .print .newline ";

    #[test]
    fn hello_world_parse() {
        assert_eq!(
            parse_ignore_comments(hello),
            vec![ Token::Str("Hello, world!".to_string()), Token::Print, Token::Newline ],
        );
    }


    static loop_: &'static str = "(a \"simple\" loop that counts to 10)
1                           (start at 1)
  .dup .print .newline      (print the current number)  
  1 .+                      (add 1) 
  .dup  11 .swap .>?       (is 11 > n?) 
-10 .cjump                  (then jump 10 words back -- count carefully) 
~end of loop here~ .print .newline";

    #[test]
    fn loop_parse() {
        assert_eq!(
            parse_ignore_comments(loop_),
            vec![ Token::Int(1), Token::Dup, Token::Print, Token::Newline,
                Token::Int(1), Token::Add,
                Token::Dup, Token::Int(11), Token::Swap, Token::LargerThan,
                Token::Int(-10), Token::Cjump,
                Token::Str("end of loop here".to_string()), Token::Print, Token::Newline,
            ],
        );
    }
}

