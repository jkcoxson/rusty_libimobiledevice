// jkcoxson

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug {
    ($x:expr) => {
        dbg!($x)
    };
}
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug_print {
    ($($arg : tt) *) => {
        println!("DEBUG: {}", format!($($arg)*));
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug {
    ($x:expr) => {};
}
#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug_print {
    () => {};
    ($($arg : tt) *) => {};
}
