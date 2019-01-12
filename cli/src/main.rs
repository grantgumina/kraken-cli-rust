extern crate clap;
extern crate chrono;
extern crate daemonize;
extern crate regex;
extern crate subprocess;
extern crate haikunator;
extern crate hostname;
extern crate notify;
extern crate dirs;
extern crate futures;
extern crate serde;
extern crate hyper_tls;

#[macro_use] extern crate hyper;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate prettytable;

use std::fs::File;
use std::io::{Write, BufRead, BufReader};
use std::fs::OpenOptions;

use clap::{Arg, App, ArgMatches, SubCommand};
use chrono::prelude::*;
use daemonize::Daemonize;
use haikunator::{Haikunator};
use hostname::get_hostname;
use subprocess::Exec;
use std::sync::atomic::{AtomicBool, Ordering};

use std::time;

use std::io::prelude::*;
use std::thread;

pub mod kraken_utils;
pub mod krephis;

// Atomic variable used by both the command execution and monitoring threads
static ATOMIC_COMMAND_DONE: AtomicBool = AtomicBool::new(false);

// Run once a new job is created
fn new(matches: &ArgMatches) {

    let hostname = get_hostname().unwrap();
    let haikunator = Haikunator::default();
    let mut unique_job_name = format!("{}-{}", hostname, haikunator.haikunate());
    let mut job_description = "";
    let job_daemon_output_file_path = format!("/tmp/kraken-job-{}.out", unique_job_name);
    let job_daemon_error_file_path = format!("/tmp/kraken-job-{}.err", unique_job_name);
    let job_daemon_pid_file_path = format!("/tmp/kraken-job-{}.pid", unique_job_name);

    println!("Local output/error files below:\n{}\n{}", job_daemon_output_file_path, job_daemon_error_file_path);

    match matches.subcommand() {

        ("job", Some(command)) => {

            // Check if the user supplied their own job name

            if let Some(name) = command.value_of("JOB_NAME") {
                unique_job_name = name.to_string();
            }

            if let Some(desc) = command.value_of("DESCRIPTION") {
                job_description = desc;
            }

            krephis::new_job(&hostname, &unique_job_name, &job_description);

            let c = command.value_of("COMMAND").unwrap().to_string();

            // Create a daemon for this job
            let job_daemon = Daemonize::new()
                .pid_file(job_daemon_pid_file_path) // Every method except `new` and `start`
                .chown_pid_file(true)      // is optional, see `Daemonize` documentation
                .working_directory("/tmp") // for default behaviour.
                .privileged_action(move || {
                    // This action gets run inside the daemon
                    let jdofp = job_daemon_output_file_path.to_string();

                    let _job_stdout = File::create(&jdofp).unwrap();
                    let _job_stderr = File::create(&jdofp).unwrap();

                    let cmd = c.to_string();
                    let ujn = unique_job_name.to_string();

                    let mut threads = Vec::new();

                    // Run the command
                    threads.push(thread::spawn(move || {

                        let stream_object = Exec::shell(&cmd).stream_stdout().unwrap();
                        let reader = BufReader::new(stream_object);

                        let mut file = OpenOptions::new()
                            .write(true)
                            .append(true)
                            .open(&jdofp)
                            .unwrap();

                        let s = format!("Kraken - Job - {}\n======\n$> {}\n", Utc::now(), &cmd);
                        let _ = writeln!(file, "{}", s);

                        krephis::new_log(&ujn, &s);

                        // Write output stream to the file
                        for line in reader.lines() {

                            file = OpenOptions::new()
                                .write(true)
                                .append(true)
                                .open(&jdofp)
                                .unwrap();

                            let x = format!("{}", line.unwrap());
                            let _ = writeln!(file, "{}", x);
                            krephis::new_log(&ujn, &x);

                        }

                        let _ = writeln!(file, "exit");
                        krephis::new_log(&ujn, "exit");

                        // Kill the watcher thread by updating the atomic variable
                        ATOMIC_COMMAND_DONE.store(true, Ordering::Relaxed);

                    }));

                    for t in threads {
                        t.join().unwrap();
                    }

                });

            match job_daemon.start() {
                Ok(_) => {

                },
                Err(e) => eprintln!("Kraken - Job - Error - {}\n======\n{}\n", Utc::now(), e),
            }

        },
        
        _ => println!("Use `kraken new -h` for help"),
    }
}

// Show
fn show(matches: &ArgMatches) {
    match matches.subcommand() {
        ("jobs", Some(_command)) => {
            krephis::show_jobs();
        },

        ("job", Some(command)) => {
            let job_name = command.value_of("JOB_NAME").unwrap().to_string();
            let line_limit = command.value_of("LINE_LIMIT").unwrap().to_string();

            krephis::show_job(&job_name, &line_limit);
        },
        _ => println!("Use `kraken show -h` for help"),
    }
}

// Remove
fn remove(matches: &ArgMatches) {
    match matches.subcommand() {

        ("job", Some(command)) => {

            if command.is_present("ALL") {
                krephis::remove_all_jobs();
            } else {
                let job_name = command.value_of("JOB_NAME").unwrap().to_string();
                krephis::remove_job(&job_name);
            }

        },
        _ => println!("Use `kraken remove -h` for help"),

    }
}

// Authentication
fn login(matches: &ArgMatches) {

    let email = matches.value_of("email").unwrap();
    let password = matches.value_of("password").unwrap();

    krephis::login(email.to_string(), password.to_string());

}

fn logout() {
    // kraken_utils::store_token("");
    krephis::logout();
}

fn main() {

    let app = App::new("Kraken")
        .version("0.1.0")
        .author("Grant Gumina")
        .about("Monitor jobs being run on remote machines")
        
        // Show Commands
        .subcommand(
            SubCommand::with_name("show")
                .subcommand(SubCommand::with_name("jobs"))
                
                .subcommand(
                    SubCommand::with_name("job")
                        .arg(Arg::with_name("JOB_NAME").required(true))
                        .arg(Arg::with_name("LINE_LIMIT")
                            .default_value("10")
                            .hide_default_value(false))
                )
        )

        // New Commands
        .subcommand(
            SubCommand::with_name("new")
                .subcommand(
                    SubCommand::with_name("job")
                        .arg(Arg::with_name("COMMAND").required(true))
                        .arg(Arg::with_name("DESCRIPTION")
                            .required(false)
                            .short("d")
                            .long("description")
                            .takes_value(true))
                        .arg(Arg::with_name("JOB_NAME")
                            .required(false)
                            .short("n")
                            .long("name")
                            .takes_value(true))
                )
        )

        // Remove Commands
        .subcommand(
            SubCommand::with_name("remove")
                .subcommand(SubCommand::with_name("job")
                    .arg(Arg::with_name("JOB_NAME"))
                    .arg(Arg::with_name("ALL")
                        .short("a")
                        .long("all")
                        .help("Removes all jobs. Any provided job name will be ignored.")
                    )
                )
        )
    
        // Auth Commands
        // Login
        .subcommand(
            SubCommand::with_name("login")
                .arg(Arg::with_name("email").required(true))
                .arg(Arg::with_name("password").required(true))
                .help("Login to your Kraken account. Token will be stored in `~/.krakenrc`.")
        )

        // Logout
        .subcommand(
            SubCommand::with_name("logout")
                .help("Logout of your Kraken account. Token will be stored removed from `~/.krakenrc`.")
        );

    let matches = app.get_matches();

    // Parse out commands
    match matches.subcommand() {
        ("new", Some(m)) => new(m),
        ("login", Some(m)) => login(m),
        ("logout", Some(_m)) => logout(),
        ("show", Some(m)) => show(m),
        ("remove", Some(m)) => remove(m),
        _ => println!("Use `kraken -h` for help"),
    }
}