macro_rules! error {
    ($($tt:tt)*) => {
        {
            eprintln!($($tt)*);
            std::process::exit(-1);
        }
    };
}
