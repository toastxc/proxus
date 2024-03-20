use crate::Result;
use serde::{Deserialize, Serialize};
use validator::Validate;
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Conf {
    pub reconnect: Option<Reconnect>,
    pub data: Vec<ConfData>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Reconnect {
    pub retry_time: Option<u64>,
    pub watchdog_timer: Option<u64>,
    pub reset_after_success: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConfData {
    pub a1: String,
    pub a2: String,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone, Validate)]
pub struct PortBindings {
    #[validate(length(min = 2, max = 2))]
    pub ip: Vec<String>,
    pub bind: Vec<u32>,
}
#[derive(Serialize, Deserialize, Debug, Default, Clone, Validate)]
pub struct BindCast {
    #[validate(length(min = 1, max = 2))]
    pub ip: Vec<String>,
    pub bind: Vec<u32>,
    pub cast: Vec<u32>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EncapHolder {
    pub reconnect: Option<Reconnect>,
    pub data: Vec<DataEncap>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum DataEncap {
    BindCast(BindCast),
    PortBinding(PortBindings),
    PtP(ConfData),
}

impl From<DataEncap> for Vec<ConfData> {
    fn from(value: DataEncap) -> Self {
        match value {
            DataEncap::PtP(data) => vec![data],
            DataEncap::PortBinding(data) => data.into(),
            DataEncap::BindCast(data) => data.into(),
        }
    }
}

impl From<PortBindings> for Vec<ConfData> {
    fn from(value: PortBindings) -> Self {
        value
            .bind
            .into_iter()
            .map(|port| ConfData {
                a1: format!("{}:{}", value.ip.first().unwrap(), port),
                a2: format!(
                    "{}:{}",
                    value.ip.get(1).unwrap_or(value.ip.first().unwrap()),
                    port
                ),
            })
            .collect()
    }
}
impl From<BindCast> for Vec<ConfData> {
    fn from(value: BindCast) -> Self {
        value
            .bind
            .into_iter()
            .enumerate()
            .map(|(iter, bind)| ConfData {
                a1: format!("{}:{}", value.ip[0], bind),
                a2: format!(
                    "{}:{}",
                    value.ip.get(1).unwrap_or(value.ip.first().unwrap()),
                    value.cast[iter]
                ),
            })
            .collect()
    }
}

impl Conf {
    pub fn from_file(path: &str) -> Result<Self> {
        let encap: EncapHolder = toml::from_str(&String::from_utf8(std::fs::read(path)?)?)?;

        Ok(Conf {
            reconnect: encap.reconnect,
            data: encap
                .data
                .into_iter()
                .map(|conf| conf.into())
                .collect::<Vec<Vec<ConfData>>>()
                .concat(),
        })
    }
}
