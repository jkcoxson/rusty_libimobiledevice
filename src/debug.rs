// jkcoxson

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug {
    ($($arg : tt) *) => {
        let file = file!().split('/').last().unwrap();
        println!("{}:{} -- {}", file, line!(), format!($($arg)*));
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug {
    ($x:expr) => {};
}
