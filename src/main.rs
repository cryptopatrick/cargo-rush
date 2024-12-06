use argh::FromArgs;
use clipboard::{ClipboardContext, ClipboardProvider};
use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    run_clipboard_code();
}

fn run_clipboard_code() {
    // Get the code from the clipboard
    let mut clipboard: ClipboardContext =
        ClipboardProvider::new().expect("Failed to access clipboard");
    let code = clipboard
        .get_contents()
        .expect("Failed to read clipboard contents");

    // Spawn the rustc process, feeding the code via stdin
    let mut rustc_process = Command::new("rustc")
        .arg("-o")
        .arg("/tmp/clipboard_program") // Temporary output location
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start rustc process");

    // Write the Rust code to rustc's stdin
    {
        let stdin = rustc_process
            .stdin
            .as_mut()
            .expect("Failed to open rustc stdin");
        stdin
            .write_all(code.as_bytes())
            .expect("Failed to write code to rustc");
    }

    // Wait for rustc to finish
    let rustc_status = rustc_process.wait().expect("Failed to wait for rustc");

    if !rustc_status.success() {
        eprintln!("Compilation failed");
        return;
    }

    // Execute the compiled program
    let run_status = Command::new("/tmp/clipboard_program")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Failed to execute compiled program");

    if !run_status.success() {
        eprintln!("Execution failed");
    }
}
