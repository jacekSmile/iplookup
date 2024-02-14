use std::net::IpAddr;

use serde::Serialize;
use std::convert::TryFrom;

mod baidu_api;
mod taobao_api;
mod ipinfo_api;

#[derive(Debug, Serialize)]
pub enum IpInfo {
    Baidu(baidu_api::IpInfo),
    Taobao(taobao_api::IpInfo),
    Ipinfo(ipinfo_api::IpInfo),
}

#[derive(Debug, Clone)]
pub enum Api {
    Baidu,
    Ipinfo,
    Taobao,
}

impl std::str::FromStr for Api {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "baidu" => Ok(Api::Baidu),
            "ipinfo" => Ok(Api::Ipinfo),
            "taobao" => Ok(Api::Taobao),
            _ => Err("Invalid API".to_string()),
        }
    }
}

impl std::convert::TryFrom<baidu_api::IpInfo> for IpInfo {
    type Error = anyhow::Error;

    fn try_from(value: baidu_api::IpInfo) -> Result<Self, Self::Error> {
        Ok(IpInfo::Baidu(value))
    }
}

impl std::convert::TryFrom<ipinfo_api::IpInfo> for IpInfo {
    type Error = anyhow::Error;

    fn try_from(value: ipinfo_api::IpInfo) -> Result<Self, Self::Error> {
        Ok(IpInfo::Ipinfo(value))
    }
}

impl std::convert::TryFrom<taobao_api::IpInfo> for IpInfo {
    type Error = anyhow::Error;

    fn try_from(value: taobao_api::IpInfo) -> Result<Self, Self::Error> {
        Ok(IpInfo::Taobao(value))
    }
}

pub fn get_ip_info(api: &Api, ip: &IpAddr) -> Result<IpInfo, anyhow::Error> {
    match api {
        Api::Baidu => baidu_api::get_ip_info(ip).map(|result| {
            Ok(IpInfo::try_from(result.data)?)
        }),
        Api::Ipinfo => ipinfo_api::get_ip_info(ip).map(|result| {
            Ok(IpInfo::try_from(result.data)?)
        }),
        Api::Taobao => taobao_api::get_ip_info(ip).map(|result: taobao_api::IpInfo| {
            Ok(IpInfo::try_from(result)?)
        }),
    }?
}

pub fn get_domain_info(api: &Api, domain: &str) -> Result<Vec<IpInfo>, anyhow::Error> {
    match api {
        Api::Baidu => baidu_api::get_domain_info(domain).map(|result| {
            Ok(result
                .iter()
                .map(|x| IpInfo::Baidu(x.data.clone()))
                .collect())
        }),
        Api::Ipinfo => ipinfo_api::get_domain_info(domain).map(|result| {
            Ok(result
                .iter()
                .map(|x| IpInfo::Ipinfo(x.data.clone()))
                .collect())
        }),
        Api::Taobao => taobao_api::get_domain_info(domain).map(|result| {
            Ok(result
                .iter()
                .map(|x| IpInfo::Taobao(x.clone()))
                .collect())
        }),
    }?
}

