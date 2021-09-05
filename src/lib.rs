use anyhow::anyhow;
use std::collections::HashMap;
use std::str::FromStr;
use std::{path::PathBuf, str::from_utf8};

#[derive(Debug, Eq, PartialEq)]
/// KObject action types
///
/// See kobject_action in include/linux/kobject.h
pub enum ActionType {
    Add,
    Remove,
    Change,
    Move,
    Online,
    Offline,
    Bind,
    Unbind,
}

impl FromStr for ActionType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ActionType::*;
        match s {
            "add" => Ok(Add),
            "remove" => Ok(Remove),
            "change" => Ok(Change),
            "move" => Ok(Move),
            "online" => Ok(Online),
            "offline" => Ok(Offline),
            "bind" => Ok(Bind),
            "unbind" => Ok(Unbind),
            _ => anyhow::bail!("Unexpected action: {}", s),
        }
    }
}

/// Linux kernel userspace event
#[derive(Debug)]
pub struct UEvent {
    /// Action happening
    pub action: ActionType,
    /// Complete Kernel Object path
    pub devpath: PathBuf,
    /// SubSystem originating the event
    pub subsystem: String,
    /// Additional arguments
    pub ext: HashMap<String, String>,
    /// Sequence number
    pub seq: u64,
}

impl UEvent {
    /// Parse a netlink packet as received from the NETLINK_KOBJECT_UEVENT broadcast
    pub fn from_netlink_packet(pkt: &[u8]) -> anyhow::Result<UEvent> {
        let mut action = None;
        let mut devpath = None;
        let mut subsystem = None;
        let mut ext = HashMap::new();
        let mut seq = None;

        for f in from_utf8(pkt)?.split('\0') {
            if let Some((key, value)) = f.split_once('=') {
                match key {
                    "ACTION" => action = Some(value.parse::<ActionType>()?),
                    "DEVPATH" => devpath = Some(value.parse::<PathBuf>()?),
                    "SUBSYSTEM" => subsystem = Some(value.to_string()),
                    "SEQNUM" => seq = Some(value.parse::<u64>()?),
                    _ => {
                        let _ = ext.insert(key.into(), value.into());
                    }
                }
            }
        }

        let action = action.ok_or_else(|| anyhow!("action not found"))?;
        let devpath = devpath.ok_or_else(|| anyhow!("devpath not found"))?;
        let subsystem = subsystem.ok_or_else(|| anyhow!("subsystem not found"))?;
        let seq = seq.ok_or_else(|| anyhow!("seq missing"))?;

        Ok(UEvent {
            action,
            devpath,
            subsystem,
            ext,
            seq,
        })
    }
}
