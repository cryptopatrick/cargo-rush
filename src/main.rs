//! Sometime we just want to grab a piece of source code and run it.
//! There's playground of course, but programmers are lazy, and just the thought
//! of opening a browsertab, navigating to rust.playground, and ... well, it
//! just sounds like a lot of work.
//! But how about cargo script? Sure that's one solution, but from what I can
//! tell, you need to create a file.rs and call cargo script on that file.
//! Is there something faster?
//!
//! With cargo-rush, we can simply run rustc on whatever is on the operating
//! system clipboard. On a Mac that's: mark the code, hit Cmd-C, switch to a
//! terminal and write cargo rush.
//! That will compile the code and show the results in that same terminal.
//!
//! No one is claiming that this is a big deal, but hopefully it can help
//! someone save a few seconds of or keystrokes during their daily development.
//
use argh::FromArgs;
use clipboard::{ClipboardContext, ClipboardProvider};
use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    // Parse the command-line arguments.
    let args: Args = argh::from_env();
    // Check if the -t flag was provided and call the appropriate function.
    if args.trigger_function {
        run_clipboard_tests();
    } else {
        run_clipboard_code();
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Functionality so that cargo rush -t can be called to run tests.
#[derive(FromArgs, Debug)]
struct Args {
    #[argh(switch, short = 't')]
    /// trigger the specific function
    trigger_function: bool,
}

fn run_clipboard_tests() {
    // Get access to the clipboard.
    // TODO: improve error handling to use something like anyhow or thiserror.
    let mut clipboard: ClipboardContext =
        ClipboardProvider::new().expect("Failed to access clipboard.");

    // Get the contents of the clipboard.
    let code = clipboard
        .get_contents()
        .expect("Failed to read clipboard contents.");

    ////////////////////////////////////////////////////////////////////////////
    /// Construct the rustc process
    //
    // rustc --test filename.rs -o test_binary
    // ./test_binary
    let mut rustc_process_test = Command::new("rustc")
        .arg("--test")
        .arg("-")
        .arg("-o")
        .arg("/tmp/clipboard_test_program") // Temporary output location
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start rustc process");

    {
        let stdin_t = rustc_process_test
            .stdin
            .as_mut()
            .expect("Failed to open rustc stdin");

        stdin_t
            .write_all(code.as_bytes())
            .expect("Failed to write code to rustc");
    }

    // We wait for rustc to finish.
    let rust_status = rustc_process_test
        .wait()
        .expect("Failed to wait for rustc test");

    if !rust_status.success() {
        eprintln!("Compilation failed");
        return;
    }

    // Execute the compiled program
    let run_status = Command::new("/tmp/clipboard_test_program")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Failed to execute compiled program");

    if !run_status.success() {
        eprintln!("Execution failed");
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Grab whatever is in the Operating System Clipboard.
/// Makes no assumption that it is Rust source code.
fn run_clipboard_code() {
    // Get access to the clipboard.
    // TODO: improve error handling to use something like anyhow or thiserror.
    let mut clipboard: ClipboardContext =
        ClipboardProvider::new().expect("Failed to access clipboard.");

    // Get the contents of the clipboard.
    let code = clipboard
        .get_contents()
        .expect("Failed to read clipboard contents.");

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
