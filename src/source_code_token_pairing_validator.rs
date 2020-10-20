use crate::stack::Stack;

///
#[derive(Debug, PartialEq)]
pub enum TokenType {
    Braces(char),
    ReverseBraces(char),
    Brackets(char),
    ReverseBrackets(char),
    Parentheses(char),
    ReverseParentheses(char),
    Quotes(char),
    DoubleQuotes(char),
    Arrows(char),
    ReverseArrows(char),
    Charater(char),
    Space(char),
    Period(char),
    Comma(char),
    Unknown(char),
}

#[derive(Debug, PartialEq)]
enum NeedToPairingResult {
    Yes(TokenType),
    No,
}

///
#[derive(Debug, PartialEq, Clone)]
struct PairToken {
    token: char,
    line_number: usize,
    column_number: usize,
}

///
#[derive(Debug, PartialEq)]
pub enum CodeTokenValidateResult {
    Passed,
    Fail(usize, usize, String),
}

/// Use `Stack` to validate all paired tokens in any source code.
#[derive(Debug)]
pub struct CodeTokenPairingValidator {
    token_stack: Stack<PairToken>,
}

///
impl CodeTokenPairingValidator {
    ///
    pub fn new() -> Self {
        CodeTokenPairingValidator {
            token_stack: Stack::<PairToken>::new(),
        }
    }

    ///
    fn get_token_by_char(char_to_check: char) -> TokenType {
        match char_to_check {
            '{' => TokenType::Braces(char_to_check),
            '}' => TokenType::ReverseBraces(char_to_check),
            '[' => TokenType::Brackets(char_to_check),
            ']' => TokenType::ReverseBraces(char_to_check),
            '(' => TokenType::Parentheses(char_to_check),
            ')' => TokenType::ReverseParentheses(char_to_check),
            '<' => TokenType::Arrows(char_to_check),
            '>' => TokenType::ReverseArrows(char_to_check),
            '\'' => TokenType::Quotes(char_to_check),
            '"' => TokenType::DoubleQuotes(char_to_check),
            ' ' => TokenType::Space(char_to_check),
            '.' => TokenType::Period(char_to_check),
            ',' => TokenType::Comma(char_to_check),
            'a'..='z' | 'A'..='Z' => TokenType::Charater(char_to_check),
            _ => TokenType::Unknown(char_to_check),
        }
    }

    ///
    fn need_to_pairing(token: TokenType) -> NeedToPairingResult {
        match token {
            TokenType::Braces(_) => NeedToPairingResult::Yes(token),
            TokenType::ReverseBraces(_) => NeedToPairingResult::Yes(token),
            TokenType::Brackets(_) => NeedToPairingResult::Yes(token),
            TokenType::ReverseBrackets(_) => NeedToPairingResult::Yes(token),
            TokenType::Parentheses(_) => NeedToPairingResult::Yes(token),
            TokenType::ReverseParentheses(_) => NeedToPairingResult::Yes(token),
            TokenType::Arrows(_) => NeedToPairingResult::Yes(token),
            TokenType::ReverseArrows(_) => NeedToPairingResult::Yes(token),
            TokenType::Quotes(_) => NeedToPairingResult::Yes(token),
            TokenType::DoubleQuotes(_) => NeedToPairingResult::Yes(token),
            _ => NeedToPairingResult::No,
        }
    }

    /// This function should only be called for dealing with the `reverse` pairing case!!!
    /// For example, like `)`, `]`, `}`, `'` and `"`
    ///
    /// Plz do not use it for the `non-reverse` check!!!
    fn reverse_pairing_token_check(
        &mut self,
        token_type: TokenType,
        line_num: usize,
        col_num: usize,
    ) -> CodeTokenValidateResult {
        let mut failed = false;
        let current_checking_token: char;
        let mut missing_checking_token: char = ' ';
        match token_type {
            TokenType::ReverseBraces(current_token)
            | TokenType::ReverseBrackets(current_token)
            | TokenType::ReverseParentheses(current_token)
            | TokenType::Quotes(current_token)
            | TokenType::DoubleQuotes(current_token) => {
                current_checking_token = current_token;

                #[cfg(feature = "enable_debug_code_token_pairing")]
                {
                    println!(
                        "\nreverse_pairing_token_check -> token_stack:\n {:#?}",
                        &self.token_stack
                    );
                    println!("current_token: {}", current_token);
                }

                if self.token_stack.is_empty() {
                    failed = true;
                } else {
                    let pop_token = self.token_stack.pop().unwrap();

                    #[cfg(feature = "enable_debug_code_token_pairing")]
                    println!("pop_token: {:#?}", pop_token);

                    if current_token == ')' && pop_token.token != '(' {
                        failed = true;
                        missing_checking_token = '(';
                    } else if current_token == ']' && pop_token.token != '[' {
                        failed = true;
                        missing_checking_token = '[';
                    } else if current_token == '}' && pop_token.token != '{' {
                        failed = true;
                        missing_checking_token = '{';
                    } else if current_token == '"' && pop_token.token != '"' {
                        failed = true;
                        missing_checking_token = '"';
                    }
                }

                #[cfg(feature = "enable_debug_code_token_pairing")]
                println!("token_stack after pop:\n {:#?}", &self.token_stack);
            }
            _ => {
                unreachable!();
            }
        }

        if failed {
            CodeTokenValidateResult::Fail(
                line_num,
                col_num,
                format!(
                    "Missing the pairing token '{}' of '{}' at {}:{}",
                    missing_checking_token, current_checking_token, line_num, col_num
                ),
            )
        } else {
            CodeTokenValidateResult::Passed
        }
    }

    /// Whether all tokens in the source code are paired. Get the back the detail error when
    /// failed. The
    pub fn token_pairing_check<'ptc>(&mut self, source_code: &'ptc str) -> CodeTokenValidateResult {
        let source_code_to_check = source_code.trim();
        if source_code_to_check.is_empty() {
            return CodeTokenValidateResult::Passed;
        }

        println!("source_code_to_check:\n {}", source_code_to_check);

        // 1. Split into lines, then we can report line number when failed.
        let lines = source_code_to_check.split("\n").into_iter().enumerate();

        // 2. Walk through lines and columns. Plz keep that in mind, both `line_number` and
        //    `column_number` are start from `0`!!!
        for (line_number, line) in lines {
            for (column_number, temp_char) in line.chars().enumerate() {
                let token_type = Self::get_token_by_char(temp_char);
                let need_to_pairing_result = Self::need_to_pairing(token_type);

                // Continue next around if no need to do pairing
                if need_to_pairing_result == NeedToPairingResult::No {
                    continue;
                }

                // 3. Let's do a check
                if let NeedToPairingResult::Yes(st) = need_to_pairing_result {
                    match st {
                        TokenType::Braces(c)
                        | TokenType::Brackets(c)
                        | TokenType::Parentheses(c) => {
                            self.token_stack.push(PairToken {
                                token: c,
                                line_number: line_number + 1,
                                column_number: column_number + 1,
                            });
                        }

                        TokenType::ReverseBraces(_)
                        | TokenType::ReverseBrackets(_)
                        | TokenType::ReverseParentheses(_) => {
                            let temp_result = self.reverse_pairing_token_check(
                                st,
                                line_number + 1,
                                column_number + 1,
                            );
                            if temp_result != CodeTokenValidateResult::Passed {
                                #[cfg(feature = "enable_debug_code_token_pairing")]
                                println!("\ntoken_stack:\n {:#?}", &self.token_stack);

                                return temp_result;
                            }
                        }

                        TokenType::DoubleQuotes(_) => {
                            println!(">>>>> 4");
                        }
                        _ => {}
                    };
                }
            }
        }

        #[cfg(feature = "enable_debug_code_token_pairing")]
        println!("token_stack:\n {:#?}", &self.token_stack);

        // 4. If `token_stack` is not empty, that means we got missing pairing there.
        if !self.token_stack.is_empty() {
            let pop_token = self.token_stack.pop().unwrap();
            let missing_checking_token: char;

            if pop_token.token == '(' {
                missing_checking_token = ')';
            } else if pop_token.token == '[' {
                missing_checking_token = ']';
            } else if pop_token.token == '{' {
                missing_checking_token = '}';
            } else if pop_token.token == '"' {
                missing_checking_token = '"';
            } else {
                missing_checking_token = ' ';
            }

            CodeTokenValidateResult::Fail(
                pop_token.line_number,
                pop_token.column_number,
                format!(
                    "Missing the pairing token '{}' of '{}' at {}:{}",
                    missing_checking_token,
                    pop_token.token,
                    pop_token.line_number,
                    pop_token.column_number
                ),
            )
        } else {
            CodeTokenValidateResult::Passed
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn pure_token_pairing_should_pass() {
    // let mut validator = CodeTokenPairingValidator::new();
    // let source_code_sample = "{}()[]";
    // let result = validator.passed_token_pairing_check(source_code_sample);
    // println!("result {:#?}", result);
    // }

    #[test]
    fn pure_token_pairing_should_fail() {
        let mut validator = CodeTokenPairingValidator::new();
        let source_code_sample = "
        {}
        ()
        [";
        let result = validator.token_pairing_check(source_code_sample);
        println!("result {:#?}", result);
    }

    #[test]
    fn token_pairing_should_pass() {
        let mut validator = CodeTokenPairingValidator::new();
        let source_code_sample = "
     pub fn bytes_to_u32(v: &Vec<u8>) -> u32 {
     ((v[3] as u32) << 0x3 * 8)
     | ((v[2] as u32) << 0x2 * 8)
     | ((v[1] as u32) << 0x1 * 8)
     | ((v[0] as u32) << 0x0 * 8)
     }
     ";

        let result = validator.token_pairing_check(source_code_sample);
        println!("result {:#?}", result);
    }

    #[test]
    fn token_pairing_should_fail() {
        let mut validator = CodeTokenPairingValidator::new();
        let source_code_sample = "
     pub fn bytes_to_u32(v: &Vec<u8>) -> u32
     ((v[1] as u32) << 0x1 * 8)
     }
     ";

        let result = validator.token_pairing_check(source_code_sample);
        println!("result {:#?}", result);
    }
}
