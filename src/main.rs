use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut service = String::new();
    let mut instances: Option<String> = None;

    match args.len() {
        1 => {
            // no arguments, print Usage
            print_usage();
        }
        2 | 3 => {
            // 1 or 2 arguments, set the variables
            service = args[1].clone();
            if args.len() == 3 {
                instances = Some(args[2].clone());
            }
        }
        _ => {
            // More than 2 arguments, print usage
            print_usage();
        }
    }

    println!("Displaying session count for {}", &service);
    let node_string = match instances {
        Some(s) => s,
        None => String::from("4"),
    };
    let nodes: i32 = node_string.parse().unwrap();

    for node in 1..=nodes {
        let srv_url = format!("ome-{}-app-0{}", service, node);
        let (blue_cmd, green_cmd) = build_curl(&srv_url);

        let blue_count = Command::new(blue_cmd)
            .output()
            .expect("Failed to curl blue cluster.");

        let green_count = Command::new(green_cmd)
            .output()
            .expect("Failed to curl greencluster.");

        println!("{} {:?} {:?}", srv_url, blue_count, green_count);
    }
}

fn build_curl(server: &String) -> (String, String) {
    let grepb = "| egrep 'id:.*:blue_cluster' | wc -l";
    let grepg = "| egrep 'id:.*:green_cluster' | wc -l";
    let mcm = "mod_cluster-manager";

    let fqdn = format!("{}.prod.aws.ksu.edu", server);
    let url = format!("http://{}/{}", fqdn, mcm);

    let blue = format!("curl -s {} {}", url, grepb);
    let green = format!("curl -s {} {}", url, grepg);

    return (blue, green);
}

fn print_usage() {
    println!("Usage:");
    println!("    sessions <cluster-generation> [<count>]");
    println!("");
    println!("where:");
    println!("     'cluster' is the Wildfly cluster name");
    println!("     'gemeration' is the tier plus number");
    println!("     'count' is the optional number of servers in the cluster. Default is 4.");
    println!("");
    println!("For example:");
    println!("    sessions eprofile-p4 4");

    std::process::exit(1);
}
