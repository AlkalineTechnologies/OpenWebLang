use crate::lexer::LexerInput;
use std::ops::Range;

pub fn error_print(input: &mut LexerInput, span: Range<usize>, msg: &str) {
    let mut line = String::new();
    for (i, ch) in input.enumerate().skip(span.start) {
        if i == span.end {
            break;
        } else {
            line.push(ch);
        }
    }
    eprintln!("{}", line);
    eprintln!("{}", msg);
}

macro_rules! error {
    ($input:expr, $span:expr, $($tt:tt)*) => {
        {
            $crate::error::error_print($input, $span, format!($($tt)*).as_str());
            std::process::exit(-1);
        }
    };
}
