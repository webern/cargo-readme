/// Helper macro used in tests to concatenate strings using '\n' as a separator
///
/// This is used to simulate how strings would be read from a file
#[cfg(test)]
macro_rules! concat_lines {
    // no trailing comma
    ( $( $line:expr ),+ ) => {
        concat!( $( $line, "\n", )* );
    };
    // trailing comma
    ( $( $line:expr ),+, ) => {
        concat!( $( $line, "\n", )* );
    };
}
