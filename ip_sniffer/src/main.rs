use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::process;
use std::thread;
use std::sync::mpsc::{Sender, channel};

#[derive(Debug, Clone)]
struct Arguments {
    flag: String,
    ip: IpAddr,
    threads: u16
}

const MAX : u16 = 65535;

impl Arguments {
    fn new (args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments passed...");
        } else if args.len() > 4 {
            return Err("Too many arguments passed...");
        }

        let f = args[1].clone();

        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments { flag: String::from(""), ip: ipaddr, threads: 4 })
        } else {
            let flag = args[1].clone();

            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!("Usage: -j to select how many threads you want
                \r\n -h or -help to see this help message");

                return Err("help");
            } else if flag.contains("-h") || flag.contains("-help") && args.len() > 2 {
                 return Err("too many arguments");
            } else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("Not a valid IpAddr; Must be IPV4 or IPV6")
                };

                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("Not a valid thread number")
                };

                return Ok(Arguments { flag, ip: ipaddr, threads })
            } else {
                return Err("out of scope");
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0);
        } else {
            eprintln!("{} problem parsing parameters {}", program, err);
            process::exit(0);
        }
    });

    let num_threads = arguments.threads;
    let addr = arguments.ip;
    let (tr, wr) = channel();
    
    for i in 0..num_threads {
        let tr = tr.clone();

        thread::spawn(move || {
            scan(tr, i, addr, num_threads);
        });
    }

    let mut out = vec![];

    drop(tr);

    for rc in wr {
        out.push(rc);
    }

    println!("");

    out.sort();

    for v in out {
        println!("{} is open", v);
    }
}


fn scan (tr: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port : u16 = start_port + 1;

    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                println!(".");
                io::stdout().flush().unwrap();

                tr.send(port).unwrap();
            }
            Err(_) => {}
        }

        if (MAX - port) <= num_threads {
            break;
        }

        port += num_threads;
    }
}