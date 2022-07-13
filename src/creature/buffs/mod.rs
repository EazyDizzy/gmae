use core::fmt::Debug;
use std::time::{Duration, Instant};
use crate::creature::component::physiology_description::PhysiologyDescription;
use bevy::prelude::*;
use crate::creature::buffs::sprint::{apply_buffs, buffs_add_sprint, clear_buffs};

pub mod sprint;

#[allow(dead_code)]
#[derive(Debug)]
pub enum BuffTimer {
    Period(Duration),
    Frame(u8),
}

pub trait PhysiologyBuff: Send + Sync + Debug {
    fn apply(&self, phys: &mut PhysiologyDescription);
    fn remove(&self, phys: &mut PhysiologyDescription);
}

#[derive(Debug)]
pub struct BuffClock {
    buff: Box<dyn PhysiologyBuff>,
    timer: BuffTimer,
    start_time: Option<Instant>,
    call_amount: Option<u8>
}

#[allow(dead_code)]
impl BuffClock {
    pub fn frame(buff: Box<dyn PhysiologyBuff>, frames: u8) -> BuffClock {
        BuffClock {
            timer: BuffTimer::Frame(frames),
            start_time: None,
            call_amount: Some(0),
            buff
        }
    }

    pub fn period(buff: Box<dyn PhysiologyBuff>, duration: Duration) -> BuffClock {
        BuffClock {
            timer: BuffTimer::Period(duration),
            start_time: Some(Instant::now()),
            call_amount: None,
            buff
        }
    }

    fn apply(&mut self, phys: &mut PhysiologyDescription) {
        self.call_amount = self.call_amount.map(|val| val + 1);
        self.buff.apply(phys);
    }

    fn should_remove(&self) -> bool {
        match self.timer {
            BuffTimer::Frame(val) => self.call_amount.unwrap_or(0) == val,
            BuffTimer::Period(val) => {
                let now = Instant::now();
                now.duration_since(self.start_time.unwrap()) >= val
            },
        }

    }

    fn remove(&self, phys: &mut PhysiologyDescription) {
        self.buff.remove(phys);
    }
}

#[derive(Debug)]
pub struct SprintBuff {
    speed_multiplier: f32
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
        for buff in &mut self.physiology_buffs.iter_mut() {
            buff.apply(phys);
        }
    }

    pub fn clean(&mut self, phys: &mut PhysiologyDescription) {
        self.physiology_buffs.retain(|buff| {
            if buff.should_remove() {
                buff.remove(phys);
                return false
            }
            true
        });
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct BuffsPlugin;

impl Plugin for BuffsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::Update, buffs_add_sprint)
            .add_system_to_stage(CoreStage::PreUpdate, apply_buffs)
            .add_system_to_stage(CoreStage::PostUpdate, clear_buffs);
    }
}
