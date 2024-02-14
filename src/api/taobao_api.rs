use std::net::IpAddr;
use anyhow::Result;
use headless_chrome::Browser;

use dns_lookup::lookup_host;
use serde::Serialize;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref BROWSER: Mutex<Browser> = Mutex::new(Browser::default().unwrap());
    static ref TAB: Mutex<std::sync::Arc<headless_chrome::Tab>> = Mutex::new(BROWSER.lock().unwrap().new_tab().unwrap());
}

#[derive(Debug, Serialize, Clone)]
pub struct IpInfo {
    pub ip: String,
    pub country: String,
    pub province: String,
    pub city: String,
    pub county: String,
    pub isp: String,
}

pub fn get_ip_info(ip: &IpAddr) -> Result<IpInfo, anyhow::Error> {
    let tab = TAB.lock().unwrap();
    
    tab.navigate_to("https://ip.taobao.com/")?;
    tab.wait_until_navigated()?;
    tab.wait_for_element("#ipInfo")?
        .click()?;
    tab.type_str(&ip.to_string())?.press_key("Enter")?;
    tab.wait_for_element("#obviousIp")?;

    let result = tab.wait_for_elements("td").unwrap();

    Ok(IpInfo {
        ip: result[0].get_inner_text().unwrap(),
        country: result[1].get_inner_text().unwrap(),
        province: result[2].get_inner_text().unwrap(),
        city: result[3].get_inner_text().unwrap(),
        county: result[4].get_inner_text().unwrap(),
        isp: result[5].get_inner_text().unwrap(),
    })
}

pub fn get_domain_info(domain: &str) -> Result<Vec<IpInfo>, reqwest::Error> {
    let ips = lookup_host(domain).unwrap();
    let ip_infos = ips
        .iter()
        .filter(|ip| ip.is_ipv4())
        .map(|ip| {
            get_ip_info(&ip).unwrap()
        })
        .collect();

    Ok(ip_infos)
}
