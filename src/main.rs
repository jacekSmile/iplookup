pub mod api;

use clap::{CommandFactory, Parser};
use api::{get_ip_info, Api};

use crate::api::get_domain_info;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Args {
    /// The API [baidu, ipinfo, taobao] to use
    /// Example: iplookup -a baidu -i 127.0.0.1
    #[arg(short, long, default_value = "baidu", verbatim_doc_comment)]
    api: Api,

    /// The IP address to query
    /// Example: iplookup -i 127.0.0.1
    #[arg(short, long, verbatim_doc_comment)]
    ip: Option<std::net::IpAddr>,

    /// The domain to query
    /// Example: iplookup -d example.com
    #[arg(short, long, verbatim_doc_comment)]
    domain: Option<String>,

    /// The multi-ip address to query
    /// Example: iplookup -I 127.0.0.1 -I 127.0.0.1
    #[arg(short = 'I', long, verbatim_doc_comment)]
    multi_ip: Option<Vec<std::net::IpAddr>>,

    /// The multi-domain to query
    /// Example: iplookup -D example.com -D example.com
    #[arg(short = 'D', long, verbatim_doc_comment)]
    multi_domain: Option<Vec<String>>,
}

fn search_ip(api: &Api, ip: &std::net::IpAddr) {
    if let Ok(result )= get_ip_info(api, ip) {
        println!("query IP: {}", ip);
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
        return;
    } else {
        eprintln!("You are querying too frequently, please try again later.");
    }
}

fn search_domain(api: &Api, domain: &str) {
    if let Ok(result) = get_domain_info(api, domain) {
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
        return;
    } else {
        eprintln!("You are querying too frequently, please try again later.");
    }
}

fn main() {
    let args = Args::parse();
    if let Some(ip) = args.ip {
        search_ip(&args.api, &ip);
    } else if let Some(domain) = args.domain {
        search_domain(&args.api, &domain);
    } else if let Some(ips) = args.multi_ip {
        for ip in ips {
            search_ip(&args.api, &ip);
        }
    } else if let Some(domains) = args.multi_domain {
        for domain in domains {
            search_domain(&args.api, &domain);
        }
    } else {
        Args::command().print_help().unwrap();
    }
}
