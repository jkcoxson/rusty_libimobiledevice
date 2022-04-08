// jkcoxson

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug {
    ($($arg : tt) *) => {
        println!("{}:{} -- {}", file!().split('/').last().unwrap(), line!(), format!($($arg)*))
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug {
    ($($arg : tt) *) => {
        ()
    };
}
