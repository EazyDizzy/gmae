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
    pub start_time: Option<DateTime<Utc>>
}

impl BuffClock {
    pub fn frame(buff: Box<dyn PhysiologyBuff>, frames: u8) -> BuffClock {
        BuffClock {
            timer: BuffTimer::Frame(frames),
            start_time: None,
            buff
        }
    }

    pub fn seconds(buff: Box<dyn PhysiologyBuff>, seconds: u8) -> BuffClock {
        BuffClock {
            timer: BuffTimer::Seconds(seconds),
            start_time: Some(Utc::now()),
            buff
        }
    }

    pub fn minutes(buff: Box<dyn PhysiologyBuff>, minutes: u8) -> BuffClock {
        BuffClock {
            timer: BuffTimer::Minutes(minutes),
            start_time: Some(Utc::now()),
            buff
        }
    }

    fn apply(&self, phys: &mut PhysiologyDescription) {
        self.buff.apply(phys)
    }

    fn should_remove(&self) -> bool {
        //

        // checks if buff expired
        true
    }
}

#[derive(Debug)]
pub struct SprintBuff {}

impl SprintBuff {
    pub fn new() -> SprintBuff {
        SprintBuff{}
    }
}

impl PhysiologyBuff for SprintBuff {
    fn apply(&self, phys: &mut PhysiologyDescription) {
        phys.movement_speed = 0.3;
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
    fn apply() {
        // iterate physiology_buffs and call
    }
    fn clean(&self) {
        // if should remove - then remove from this.physiology_buffs
    }
}