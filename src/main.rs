use std::process::ExitCode;

use std::io;

fn sub() -> Result<(), io::Error> {
    rs_sort_by_basename::stdin2strings2sorted2stdout()
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
