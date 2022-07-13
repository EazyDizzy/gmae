use crate::creature::component::physiology_description::PhysiologyDescription;
use bevy::prelude::*;
use lib::entity::level::creature::Creature;
use lib::entity::level::Level;
use lib::entity::point::Point;
use lib::util::math::round_based;


#[derive(Component, Debug)]
pub struct Locomotivity {
    position: Point,
}

impl From<&Creature> for Locomotivity {
    fn from(creature: &Creature) -> Self {
        Locomotivity::new(Point::new(
            creature.position.x,
            creature.position.y,
            creature.position.z,
        ))
    }
}

// move fns
impl Locomotivity {
    pub fn new(position: Point) -> Locomotivity {
        Locomotivity {
            position,
        }
    }

    pub fn position(&self) -> &Point {
        &self.position
    }

    pub fn move_to(
        &mut self,
        future_x: f32,
        future_z: f32,
        lvl: &Res<Level>,
        phys: &PhysiologyDescription,
    ) {
        if self.creature_not_inside_blocks(future_x, future_z, lvl, phys) {
            self.position.x = future_x;
            self.position.z = future_z;
        }
    }
}

// obstacles checks
impl Locomotivity {
    fn can_jump(&self, lvl: &Res<Level>, phys: &PhysiologyDescription) -> bool {
        self.has_fundament(lvl, phys) && !self.has_ceil(lvl)
    }

    fn should_fall(&self, lvl: &Res<Level>, phys: &PhysiologyDescription) -> bool {
        self.position.y - self.position.y.floor() != 0.0 || !self.has_fundament(lvl, phys)
    }
    fn can_fall(&self, lvl: &Res<Level>, phys: &PhysiologyDescription) -> bool {
        let future_y = self.position.y.floor();

        self.no_y_obstacles(future_y, lvl, phys)
    }

    fn has_fundament(&self, lvl: &Res<Level>, phys: &PhysiologyDescription) -> bool {
        !self.no_y_obstacles(self.position.y, lvl, phys)
    }
    fn has_ceil(&self, lvl: &Res<Level>) -> bool {
        let future_position = Point::new(
            self.position.x.round(),
            (self.position.y + 3.0).floor(),
            self.position.z.round(),
        );
        let voxel_ceil = lvl.get_voxel_by_point(&future_position);

        voxel_ceil.is_some()
    }

    pub fn creature_not_inside_blocks(
        &self,
        x: f32,
        z: f32,
        lvl: &Res<Level>,
        phys: &PhysiologyDescription,
    ) -> bool {
        let mut obstacles: Vec<Point> = get_touched_points(x, self.position.y, z, phys);

        obstacles = obstacles
            .iter()
            .map(|p| Point::new(p.x, p.y + 1.0, p.z))
            .collect();

        obstacles.extend(
            obstacles
                .iter()
                .map(|p| Point::new(p.x, p.y + 1.0, p.z))
                .collect::<Vec<Point>>(),
        );

        lvl.points_are_empty(&obstacles)
    }

    fn no_y_obstacles(&self, y: f32, lvl: &Res<Level>, phys: &PhysiologyDescription) -> bool {
        let obstacles: Vec<Point> = get_touched_points(self.position.x, y, self.position.z, phys);

        lvl.points_are_empty(&obstacles)
    }
    pub fn has_y_obstacles_on_point(
        &self,
        x: f32,
        y: f32,
        z: f32,
        lvl: &Res<Level>,
        phys: &PhysiologyDescription,
    ) -> bool {
        let obstacles: Vec<Point> = get_touched_points(x, y, z, phys);

        !lvl.points_are_empty(&obstacles)
    }
}

fn get_touched_points(x: f32, y: f32, z: f32, phys: &PhysiologyDescription) -> Vec<Point> {
    let model_radius = phys.model_radius;
    let mut points: Vec<Point> = vec![Point::new(x.floor(), y, z), Point::new(x, y, z.floor())];

    let x_gap = round_based(x - x.floor(), 1);
    if x_gap > model_radius {
        points.push(Point::new((x + model_radius).floor(), y, z.floor()));
    } else if x_gap < model_radius {
        points.push(Point::new((x - model_radius).floor(), y, z.floor()));
    };

    let z_gap = round_based(z - z.floor(), 1);
    if z_gap > model_radius {
        points.push(Point::new(x.floor(), y, (z + model_radius).floor()));
    } else if z_gap < model_radius {
        points.push(Point::new(x.floor(), y, (z - model_radius).floor()));
    };

    if x_gap > model_radius && z_gap > model_radius {
        points.push(Point::new(
            (x + model_radius).floor(),
            y,
            (z + model_radius).floor(),
        ));
    } else if x_gap < model_radius && z_gap < model_radius {
        points.push(Point::new(
            (x - model_radius).floor(),
            y,
            (z - model_radius).floor(),
        ));
    } else if x_gap > model_radius && z_gap < model_radius {
        points.push(Point::new(
            (x + model_radius).floor(),
            y,
            (z - model_radius).floor(),
        ));
    } else if x_gap < model_radius && z_gap > model_radius {
        points.push(Point::new(
            (x - model_radius).floor(),
            y,
            (z + model_radius).floor(),
        ));
    }

    points
}
