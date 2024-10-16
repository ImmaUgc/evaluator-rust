#[derive(PartialEq, Clone, Debug)]
pub enum TokenKind {
    Number(i64),
    Plus,
    Minus,
    Slash,
    Asterisk,
    LParen,
    RParen,
    Whitespace,
    Eof,
    Unknown
}

#[derive(Clone, Debug)]
pub struct TextSpan {
    start: usize,
    end: usize,
    literal: String
}

impl TextSpan {
    pub fn new(start: usize, end: usize, literal: String) -> Self {
        Self {
            start,
            end,
            literal
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub span: TextSpan,
    pub kind: TokenKind
}

impl Token {
    pub fn new(span: TextSpan, kind: TokenKind) -> Self {
        Self {
            span,
            kind
        }
    }
}

pub struct Lexer<'a> {
    input: &'a str,
    current_pos: usize
}

impl <'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            current_pos: 0
        }
    }

    fn current_char(&mut self) -> Option<char> {
        self.input.chars().nth(self.current_pos)
    }

    fn peek_char(&mut self) -> Option<char> {
        self.input.chars().nth(self.current_pos + 1)
    }

    fn consume_character(&mut self) -> Option<char> {
        if self.current_pos > self.input.len() {
            return None;
        }
        let current = self.current_char();
        self.current_pos += 1;
        current
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.current_pos == self.input.len() {
            self.consume_character();
            return Some(Token::new(
                TextSpan::new(0, 0, '\0'.to_string()),
                TokenKind::Eof
            ));
        }
        
        let current_char = self.current_char();
        current_char.map(|c| {
            let start = self.current_pos;
            let mut kind = TokenKind::Unknown;

            if Self::is_number_start(&c) {
                let number = self.consume_number();
                kind = TokenKind::Number(number);
            } else if c.is_whitespace() {
                self.consume_character();
                kind = TokenKind::Whitespace;
            } else {
                kind = self.consume_poctuation();
            }

            let end = self.current_pos;
            let literal = self.input[start..end].to_string();
            let span = TextSpan::new(start, end, literal);
            
            Token::new(span, kind)
        })
    }

    fn is_number_start(c: &char) -> bool {
        c.is_digit(10)
    }

    fn consume_number(&mut self) -> i64 {
        let mut number: i64 = 0;
        while let Some(c) = self.current_char() {
            if c.is_digit(10) {
                self.consume_character();
                number = number * 10 + c.to_digit(10).unwrap() as i64;
            } else {
                break;
            }
        }
        number
    }

    fn consume_poctuation(&mut self) -> TokenKind {
        let c = self.consume_character().unwrap();

        match c {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '/' => TokenKind::Slash,
            '*' => TokenKind::Asterisk,
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            _ => TokenKind::Unknown
        }
    }
}