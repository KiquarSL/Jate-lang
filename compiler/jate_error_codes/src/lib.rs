#[macro_export]
macro_rules! error_codes {
    ($($num:literal),* $(,)?) => {
        pub fn get_error(code: &str) -> &'static str {
            match code {
                $(
                    concat!("E", stringify!($num)) => include_str!(concat!("../errors/E", stringify!($num), ".md")),
                )*
                _ => "Unknown error code",
            }
        }
    };
}

error_codes!(0001, 0002, 0003, 0004, 0005, 0006, 0007, 0008, 0009);
