use std::env;
use std::process;

fn print_usage() {
    eprintln!("USAGE: runner [filename] [build / run]");
}

fn run_command(
    command: String,
    args: Vec<String>,
    on_success: impl Fn(process::Output),
    on_fail: impl Fn(process::Output)
) {
    let output = process::Command::new(command.clone())
        .args(args.iter())
        .output()
        .expect(&format!("ERROR: Failed to execute command: \"{}\"", command.clone()));

    if output.status.success() {
        on_success(output);
    } else {
        on_fail(output);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
                
        // TODO(calco): Figure out the proper exit code.
        process::exit(1);
    }

    let run = if args.len() < 3 {
        true
    } else {
        args[2] == "run"
    };
    
    run_command("rustc".to_string(),
        vec![args[1].clone(), "--out-dir".to_string(), "bin".to_string()],
        |_output| {
            if run {
                run_command(format!("./bin/{}", &args[1][..args[1].len()-3].to_string()), vec![], |output| {
                    println!("{}", 
                        String::from_utf8(output.stdout)
                            .expect("ERROR: Could not get string from utf8 bytes.")
                    );

                    process::exit(0);
                }, |output| {
                    eprintln!("ERROR: Could not run the program!\noutput:\n{}",
                        String::from_utf8(output.stderr)
                            .expect("ERROR: Could not get string from utf8 bytes.")
                    );

                    process::exit(2);
                });
            }
        }, |output| {
            eprintln!("ERROR: Could not compile code!\nrustc output:\n{}",
                String::from_utf8(output.stderr)
                .expect("ERROR: Could not get string from utf8 bytes.")
            );

            process::exit(1);
        }
    );
}
