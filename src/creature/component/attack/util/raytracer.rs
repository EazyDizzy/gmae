use lib::entity::level::Level;
use lib::entity::point::Point;

pub fn can_see_from(start: &Point, end: &Point, lvl: &Level) -> bool {
    let touched_points = get_touched_points_between(start, end, true);

    lvl.points_are_empty(&touched_points)
}
pub fn get_last_seen_point(start: &Point, end: &Point, lvl: &Level) -> Option<Point> {
    let touched_points = get_touched_points_between(start, end, false);

    touched_points
        .into_iter()
        .find(|p| lvl.get_voxel_by_point(&p.floor()).is_some())
}

fn get_touched_points_between(start: &Point, end: &Point, floor: bool) -> Vec<Point> {
    let line_length =
        ((start.x - end.x).powi(2) + (start.y - end.y).powi(2) + (start.z - end.z).powi(2))
            .sqrt()
            .ceil() as usize;
    let amount_of_steps = line_length * 2;
    let mut coordinates = Vec::with_capacity(amount_of_steps);
    let x_step_size = (end.x - start.x) / amount_of_steps as f32;
    let y_step_size = (end.y - start.y) / amount_of_steps as f32;
    let z_step_size = (end.z - start.z) / amount_of_steps as f32;
    for i in 0..=(line_length * 2) {
        let step = i as f32;
        let point = if floor {
            Point::new(
                (start.x + x_step_size * step).floor(),
                (start.y + y_step_size * step).floor(),
                (start.z + z_step_size * step).floor(),
            )
        } else {
            Point::new(
                start.x + x_step_size * step,
                start.y + y_step_size * step,
                start.z + z_step_size * step,
            )
        };
        coordinates.push(point);
    }
    coordinates.dedup();

    coordinates
}

#[cfg(test)]
#[bench]
fn bench_get_touched_points_between(b: &mut test::Bencher) {
    let start = Point::new(0.0, 0.0, 0.0);
    let end = Point::new(5.0, -4.0, 11.0);

    b.iter(|| {
        get_touched_points_between(&start, &end, true);
    });
}
//
// #[test]
// fn test_get_touched_points_between() {
//     let start = Point::new(0.0, 0.0, 0.0);
//     let end = Point::new(5.0, -4.0, 11.0);
//
//     println!(
//         "result points {:?}",
//         get_touched_points_between(start.clone(), end.clone())
//     );
// }
