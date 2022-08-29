use crate::lexer::LexerInput;
use std::ops::Range;

pub fn error_print(input: LexerInput, span: Range<usize>, msg: &str) {
    let mut line = 1;
    let mut column = 1;
    let mut line_begin = 0;
    for (i, char) in input.clone().enumerate() {
        if i == span.start {
            break;
        }
        if char == '\n' {
            line += 1;
            column = 1;
            line_begin = i + 1;
        } else {
            column += 1;
        }
    }
    let mut line_len = 0;
    for char in input.clone().skip(line_begin) {
        if char == '\n' {
            break;
        }
        line_len += 1;
    }
    let mut print_area = String::new();
    for char in input.skip(line_begin) {
        if char == '\n' {
            break;
        }
        print_area.push(char);
    }
    eprintln!("{}", print_area);
    eprintln!(
        "{}{}",
        " ".repeat(column - 1),
        "^".repeat((span.end - span.start).min(line_len - (column - 1)))
    );
    #[cfg(debug_assertions)]
    eprintln!("{}:{} ({:?}): {}", line, column, span, msg);
    #[cfg(not(debug_assertions))]
    eprintln!("{}:{}: {}", line, column, msg);
}

macro_rules! error {
    ($input:expr, $span:expr, $($tt:tt)*) => {
        {
            $crate::error::error_print($input, $span, format!($($tt)*).as_str());
            std::process::exit(-1);
        }
    };
}
