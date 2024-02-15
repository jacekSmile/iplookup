use std::net::IpAddr;

use dns_lookup::lookup_host;
use serde::{Deserialize, Serialize};
use reqwest::header;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IpInfo {
    pub ip: String,
    pub city: String,
    pub region: String,
    pub country: String,
    pub loc: String,
    pub org: String,
    pub postal: String,
    pub timezone: String,
    pub asn: Asn,
    pub company: Company,
    pub privacy: Privacy,
    pub abuse: Abuse,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Asn {
    pub asn: String,
    pub name: String,
    pub domain: String,
    pub route: String,
    #[serde(rename = "type")]
    pub type_info: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Company {
    pub name: String,
    pub domain: String,
    #[serde(rename = "type")]
    pub type_info: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Privacy {
    pub vpn: bool,
    pub proxy: bool,
    pub tor: bool,
    pub relay: bool,
    pub hosting: bool,
    pub service: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Abuse {
    pub address: String,
    pub country: String,
    pub email: String,
    pub name: String,
    pub network: String,
    pub phone: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IpInfoResponse {
    pub input: String,
    pub data: IpInfo,
}

pub fn get_ip_info(ip: &IpAddr) -> Result<IpInfoResponse, anyhow::Error> {
    let mut headers = header::HeaderMap::new();
    headers.insert("User-Agent", header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36 Edg/118.0.0.0"));
    headers.insert("Accept", header::HeaderValue::from_static("*/*"));
    headers.insert("Accept-Language", header::HeaderValue::from_static("en-US,en;q=0.9,zh-CN;q=0.8,zh;q=0.7"));
    headers.insert("Referer", header::HeaderValue::from_static("https://ipinfo.io/"));
    let url = format!("https://ipinfo.io/widget/demo/{}", ip.to_string());
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .headers(headers)
        .send()?
        .json();

    if let Ok(response) = response {
        return Ok(response);
    } else {
        // 休眠 0.1 秒，防止查询过于频繁
        std::thread::sleep(std::time::Duration::from_millis(100));
        return Ok(get_ip_info(ip)?)
    }
}

pub fn get_domain_info(domain: &str) -> Result<Vec<IpInfoResponse>, reqwest::Error> {
    let ips = lookup_host(domain).unwrap();
    let ip_infos = ips
        .iter()
        .filter(|ip| ip.is_ipv4())
        .map(|ip: &IpAddr| {
            get_ip_info(&ip).unwrap()
        })
        .collect();

    Ok(ip_infos)
}
