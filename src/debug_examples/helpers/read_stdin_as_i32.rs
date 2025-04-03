use std::io;

pub fn read_stdin_as_i32() -> io::Result<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input.trim().parse().map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Not a valid number: {}", e),
        )
    })
}
