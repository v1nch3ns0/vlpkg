use std::env;
use std::process;
use colored::Colorize;
use std::fs;
mod state;
mod pkg;

fn print_usage() {
    println!("\nUsage: vlpkg {{COMMAND}} [ARG]");
    println!("{}", "\nCOMMANDS".bold());
    println!(" install <archive>     Installs a package from an archive");
    println!(" remove <package>      Removes a package");
    println!(" list                  List installed packages");
    println!(" init                  Initializes vlpkg directories (USE ON FIRST RUN)\n");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    match args[1].as_str() {
        "init" => {
            if let Err(_) = pkg::init() {
                eprintln!("Failed to initialize directories.");
                process::exit(1);
            } else {
                println!("[*] Directories initialized.");
            }
        }

        "install" => {
            if args.len() < 3 {
                eprintln!("Error: missing archive argument.");
                print_usage();
                process::exit(1);
            }
            let archive = &args[2];
            if let Err(_) = pkg::install(archive) {
                eprintln!("Failed to install archive: {}", archive);
                process::exit(1);
            } else {
                println!("Installed {}", archive);
            }
        }

        "remove" => {
            if args.len() < 3 {
                eprintln!("Error: missing package name.");
                print_usage();
                process::exit(1);
            }
            let name = &args[2];
            if let Err(_) = pkg::remove(name) {
                eprintln!("Failed to remove package: {}", name);
                process::exit(1);
            } else {
                println!("Removed {}", name);
            }
        }

        "list" => {
            let contents = fs::read_to_string("/var/lib/vlpkg/installed.log");
            println!("{:?}", contents);
        }

        "help" => print_usage(),

        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
            process::exit(1);
        }
    }
}
