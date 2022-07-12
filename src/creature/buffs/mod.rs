use core::fmt::Debug;
use serde_json::Number;
use crate::creature::component::physiology_description::PhysiologyDescription;
use chrono::{DateTime, Duration, Utc};
use bevy::prelude::*;

#[derive(Debug)]
pub enum BuffTimer {
    Seconds(u8),
    Minutes(u8),
    Frame(u8),
}

pub trait PhysiologyBuff: Send + Sync + Debug {
    fn apply(&self, phys: &mut PhysiologyDescription);
    fn remove(&self, phys: &mut PhysiologyDescription);
}

#[derive(Debug)]
pub struct BuffClock {
    pub buff: Box<dyn PhysiologyBuff>,
    pub timer: BuffTimer,
    pub start_time: Option<DateTime<Utc>>,
    pub call_amount: Option<u8>
}

impl BuffClock {
    pub fn frame(buff: Box<dyn PhysiologyBuff>, frames: u8, call_amount: u8) -> BuffClock {
        BuffClock {
            timer: BuffTimer::Frame(frames),
            call_amount: Some(call_amount),
            start_time: None,
            buff
        }
    }

    pub fn seconds(buff: Box<dyn PhysiologyBuff>, seconds: u8) -> BuffClock {
        BuffClock {
            timer: BuffTimer::Seconds(seconds),
            start_time: Some(Utc::now()),
            call_amount: None,
            buff
        }
    }

    pub fn minutes(buff: Box<dyn PhysiologyBuff>, minutes: u8) -> BuffClock {
        BuffClock {
            timer: BuffTimer::Minutes(minutes),
            start_time: Some(Utc::now()),
            call_amount: None,
            buff
        }
    }

    fn apply(&self, phys: &mut PhysiologyDescription) {
        self.buff.apply(phys)
    }

    // TODO update this logic to fit seconds & minutes
    fn should_remove(&self) -> bool {
        match self.timer {
            BuffTimer::Frame(val) => self.call_amount.unwrap() == val,
            _ => false
        }

    }

    fn delete(&self, phys: &mut PhysiologyDescription) {
        self.buff.remove(phys)
    }
}

#[derive(Debug)]
pub struct SprintBuff {
    pub speed_multiplier: f32
}

impl Default for SprintBuff {
    fn default() -> Self {
        SprintBuff {
            speed_multiplier: 1.5
        }
    }
}

impl PhysiologyBuff for SprintBuff {
    fn apply(&self, phys: &mut PhysiologyDescription) {
        phys.movement_speed *= self.speed_multiplier;
    }
    fn remove(&self, phys: &mut PhysiologyDescription) {
        phys.movement_speed = 0.1;
    }
}


#[derive(Component, Debug)]
pub struct BuffStorage {
    pub physiology_buffs: Vec<BuffClock>
}

impl BuffStorage {
    pub fn new() -> BuffStorage {
        BuffStorage {
            physiology_buffs: Vec::new()
        }
    }
    pub fn apply(&mut self, phys: &mut PhysiologyDescription) {
        for buff in self.physiology_buffs.iter_mut() {
            buff.apply(phys);
            buff.call_amount = Some(buff.call_amount.unwrap() + 1);
        }
    }

    pub fn clean(&mut self, phys: &mut PhysiologyDescription) {
        self.physiology_buffs.retain(|buff| {
            if buff.should_remove() {
                buff.delete(phys);
                return false
            }
            return true
        });
    }
}