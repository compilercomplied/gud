use std::process::Command;

// 80 columns wide
const SEPARATOR: &str =
    "--------------------------------------------------------------------------------";

/// Executes the given command appending its arguments. Wraps errors with
/// debugging information.
/// # Panics
///
/// Panics for errors found or when stderror is not empty.
pub(crate) fn execute(program: &str, args: &Vec<&str>) -> String {
    let cmd = Command::new(program).args(args).output();

    if cmd.is_err() {
        let cmd_msg = format!("{} {}", program, args.join(" "));
        eprintln!("Execution failed");
        eprintln!("command: {}", cmd_msg);

        let err = cmd.err().expect("None error while executing");

        panic!("Failed execution of command. {}", err.to_string());
    }

    let output = cmd.unwrap();

    if !(output.stderr.is_empty()) {
        eprintln!("Printing program stderr:");
        eprintln!("{}", SEPARATOR);
        let err = String::from_utf8(output.stderr).unwrap();
        eprintln!("{}", err);
        eprintln!("{}", SEPARATOR);
        panic!();
    }
    let msg = String::from_utf8(output.stdout).expect("error reading command output");
    return msg;
}
