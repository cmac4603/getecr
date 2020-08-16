use std::io::{self, Write};

use ansi_term::Style;
use ansi_term::Colour::{Green, Red, Yellow};

pub fn write(output: Result<String, String>) {
    //! Write to stout or stderr. Exits cleanly if error writing to stdout or stderr.
    if output.is_ok() {
        match writeln!(&mut io::stdout(), "{}", Style::new().bold().fg(Green).paint(output.unwrap())) {
            Ok(_) => (),
            Err(_) => ::std::process::exit(0),
        }
    } else {
        match writeln!(&mut io::stderr(), "{} {}", Style::new().on(Red).fg(Yellow).paint("Error:"), output.unwrap_err()) {
            Ok(_) => (),
            Err(_) => ::std::process::exit(0),
        }
    };
}
