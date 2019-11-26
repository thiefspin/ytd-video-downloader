use std::{env, fs};
use threadpool::ThreadPool;
use std::process::Command;
use std::collections::HashMap;
use std::path::Path;

static INPUT: &str = "input";

fn main() {
    if os_incompatible() {
        println!("Windows is not supported");
        ::std::process::exit(1);
    }

    if check_binary() {
        let pool = ThreadPool::new(20);
        let options = get_args();
        let output_path = Path::new(options.get("output").map_or_else(|| ".", |v| v));
        assert!(env::set_current_dir(&output_path).is_ok());
        println!("{:?}", output_path.display());

        options.get(INPUT).map(|filename| {
            println!("Reading file: {:?}", filename);
            return fs::read_to_string(filename)
                .expect("Something went wrong reading the file");
        }).map(|content| {
            let lines: Vec<&str> = content.split("\n").collect();
            for l in lines {
                let cl = l.to_owned();
                pool.execute(move || {
                    let mut cmd = "youtube-dl ".to_string();
                    cmd.push_str(&cl);
                    println!("{:?}", cmd);
                    let result = Command::new("sh")
                        .arg("-c")
                        .arg(cmd)
                        .output()
                        .expect("failed to execute process");
                    println!("{:?}", result);
                });
            }
        });

        pool.join();
    } else {
        println!("youtube-dl binary is missing. sudo apt install youtube-dl or brew install youtube-dl on MacOS")
    }
}

fn os_incompatible() -> bool {
    if cfg!(target_os = "windows") {
        return true;
    } else {
        return false;
    };
}

fn check_binary() -> bool {
    Command::new("sh")
        .arg("-c")
        .arg("youtube-dl --version")
        .output()
        .expect("failed to execute process")
        .status
        .success()
}

fn get_args() -> HashMap<String, String> {
    let mut hashmap = HashMap::new();
    let args = env::args()
        .collect::<Vec<String>>();
    for arg in args {
        let options: Vec<&str> = arg.split("=").collect();
        if options.len() == 2 {
            hashmap.insert(options[0].to_string(), options[1].to_string());
        }
    }
    hashmap
}
