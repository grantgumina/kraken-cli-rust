# Background
The idea behind kraken was to build a tool which allowed me to quickly fire off jobs on remote servers without having to go through the long process of starting a screen session. I also wanted to be able to go to a web interface and see the status of these jobs and their output. 

Right now, that last part doesn't exist, and the first part has some rough edges. All of this is still very much proof-of-concept, and if I had more time I'd polish it up.

# Setup
1. Install Rust via Rust website (if using MacOS - brew isn't great)
2. Make sure cargo works

# How to use
1. cd into the `cli` directory
2. Run `cargo build` to build the tool and install all dependencies
3. See how to use the command with `cargo run -- help` 
4. Good example command to use would be: `cargo run -- new job 'for i in {1..5}; do echo "iteration: $i"; sleep 2; done'`

# How this is supposed to work
The program is pretty simple. Two threads are created. One thread runs the job which is specified by the user and outputs the results to a file, while the other file monitors that file and does something with that output. At some point, the monitor thread will send the data back to a server so you can more easily monitor jobs on servers.
