use crate::parser::Token;

pub struct ForthCodegen;

impl ForthCodegen {
    pub fn gen(tokens: Vec<Token>) -> String {
        let mut ret = String::new();
        for tok in tokens {
            match tok {
                Token::Str(data) => {
                    ret.push_str("s\" ");
                    ret.push_str(&data);
                    ret.push_str("\" ");
                },
                Token::Print => ret.push_str("print"),
                Token::Comment(_) => (),
                _ => todo!(),
            }
        }

        ret
    }
}
