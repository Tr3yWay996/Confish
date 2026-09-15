// Color preset for error messages, warnings, informations, and sucess / confirmation messages
macro_rules! print_error {
    ($($arg:tt)*) => {
        println!("{}", format!($($arg)*).red())
    };
}
macro_rules! print_info {
    ($($arg:tt)*) => {
        println!("{}", format!($($arg)*).blue())
    };
}
macro_rules! print_warn {
    ($($arg:tt)*) => {
        println!("{}", format!($($arg)*).yellow())
    };
}
macro_rules! print_success {
    ($($arg:tt)*) => {
        println!("{}", format!($($arg)*).green())
    };
}
// Color macro for primary, secondary, tertiary message type
macro_rules! print_primary {
    ($($arg:tt)*) => {
        println!("{}", format!($($arg)*).rgb(255, 194, 159))
    };
}
macro_rules! print_secondary {
    ($($arg:tt)*) => {
        println!("{}", format!($($arg)*).rgb(255, 172, 125))
    };
}
macro_rules! print_tertiary {
    ($($arg:tt)*) => {
        println!("{}", format!($($arg)*).rgb(255, 149, 90))
    };
}
// Color macro do just about any RGB or HEX color possible ever to be used in the final executable
macro_rules! print_cyan {
    ($($arg:tt)*) => {
        println!("{}", format!($($arg)*).cyan())
    };
}
macro_rules! print_purple {
    ($($arg:tt)*) => {
        println!("{}", format!($($arg)*).rgb(208, 0, 255))
    };
}