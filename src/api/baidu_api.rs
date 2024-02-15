use std::net::IpAddr;

use dns_lookup::lookup_host;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IpInfo {
    /// IP 地址所在的大洲。
    pub continent: String,
    /// IP 地址所在的国家。
    pub country: String,
    /// IP 地址所在的邮政编码或区域编码。
    pub zipcode: String,
    /// IP 地址所在的时区。
    pub timezone: String,
    /// IP 地址位置的准确度。
    pub accuracy: String,
    /// IP 地址的所有者。
    pub owner: String,
    /// IP 地址的互联网服务提供商（ISP）。
    pub isp: String,
    /// IP 地址信息的来源。
    pub source: String,
    /// IP 地址所在的区号。
    pub areacode: String,
    /// IP 地址所在的行政区划代码。
    pub adcode: String,
    /// IP 地址的自治系统（AS）号码。
    pub asnumber: String,
    /// IP 地址位置的纬度坐标。
    pub lat: String,
    /// IP 地址位置的经度坐标。
    pub lng: String,
    /// IP 地址位置的半径。
    pub radius: String,
    /// IP 地址所在的省份或州。
    pub prov: String,
    /// IP 地址所在的城市。
    pub city: String,
    /// IP 地址所在的区县或地区。
    pub district: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IpInfoResponse {
    /// 响应状态码
    pub code: String,
    /// IP 信息数据
    pub data: IpInfo,
    /// 是否收费
    pub charge: bool,
    /// 响应消息
    pub msg: String,
    /// 查询的 IP 地址
    pub ip: String,
    /// 坐标系统
    pub coordsys: String,
}

pub fn get_ip_info(ip: &IpAddr) -> Result<IpInfoResponse, anyhow::Error> {
    let url = "https://qifu-api.baidubce.com/ip/geo/v1/district";
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .query(&[
            ("ip", ip.to_string())
        ])
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
    println!("{:?}", ips);
    let ip_infos = ips
        .iter()
        .filter(|ip| ip.is_ipv4())
        .map(|ip: &IpAddr| {
            get_ip_info(ip).unwrap()
        })
        .collect();

    Ok(ip_infos)
}
