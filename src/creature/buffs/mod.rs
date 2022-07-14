use core::fmt::Debug;
use std::time::{Duration, Instant};
use bevy::prelude::*;
use crate::creature::buffs::sprint::{buffs_add_sprint};
use crate::creature::buffs::system::{apply_buffs, clear_buffs};
use crate::creature::component::physiology_description::PhysiologyDescription;

mod sprint;
mod system;

#[allow(clippy::module_name_repetitions)]
pub struct BuffsPlugin;

impl Plugin for BuffsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::Update, buffs_add_sprint)
            .add_system_to_stage(CoreStage::PreUpdate, apply_buffs::<PhysiologyDescription>)
            .add_system_to_stage(CoreStage::PostUpdate, clear_buffs::<PhysiologyDescription>);
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum BuffTimer {
    Period(Duration),
    Frame(u8),
}

pub trait Buff<Target: Component>: Send + Sync + Debug {
    fn apply(&self, target: &mut Target);
    fn remove(&self, target: &mut Target);
}

#[derive(Debug)]
pub struct BuffClock<Target: Component> {
    buff: Box<dyn Buff<Target>>,
    timer: BuffTimer,
    start_time: Option<Instant>,
    call_amount: Option<u8>
}

#[allow(dead_code)]
impl<Target: Component> BuffClock<Target> {
    pub fn frame(buff: Box<dyn Buff<Target>>, frames: u8) -> BuffClock<Target> {
        BuffClock {
            timer: BuffTimer::Frame(frames),
            start_time: None,
            call_amount: Some(0),
            buff
        }
    }

    pub fn period(buff: Box<dyn Buff<Target>>, duration: Duration) -> BuffClock<Target> {
        BuffClock {
            timer: BuffTimer::Period(duration),
            start_time: Some(Instant::now()),
            call_amount: None,
            buff
        }
    }

    fn apply(&mut self, target: &mut Target) {
        self.call_amount = self.call_amount.map(|val| val + 1);
        self.buff.apply(target);
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

    fn remove(&self, target: &mut Target) {
        self.buff.remove(target);
    }
}


#[derive(Component, Debug)]
pub struct BuffStorage<Target: Component>  {
    buffs: Vec<BuffClock<Target>>
}

impl<Target: Component> BuffStorage<Target> {
    pub fn new() -> BuffStorage<Target> {
        BuffStorage {
            buffs: Vec::new()
        }
    }

    pub fn add(&mut self, new_buff: BuffClock<Target>) {
        self.buffs.push(new_buff);
    }

    pub fn apply(&mut self, target: &mut Target) {
        for buff in &mut self.buffs.iter_mut() {
            buff.apply(target);
        }
    }

    pub fn clean(&mut self, target: &mut Target) {
        self.buffs.retain(|buff| {
            if buff.should_remove() {
                buff.remove(target);
                return false
            }
            true
        });
    }
}
