use lib::entity::level::Level;
use lib::entity::point::Point;

pub fn can_see_from(start: &Point, end: &Point, lvl: &Level) -> bool {
    let touched_points = get_touched_points_between(start, end);

    lvl.points_are_empty(&touched_points)
}

fn get_touched_points_between(start: &Point, end: &Point) -> Vec<Point> {
    let line_length =
        ((start.x - end.x).powi(2) + (start.y - end.y).powi(2) + (start.z - end.z).powi(2))
            .sqrt()
            .ceil() as usize;
    let amount_of_steps = line_length * 2;
    let mut coordinates = Vec::with_capacity(amount_of_steps);
    let x_step_size = (end.x - start.x) / amount_of_steps as f32;
    let y_step_size = (end.y - start.y) / amount_of_steps as f32;
    let z_step_size = (end.z - start.z) / amount_of_steps as f32;
    for i in 0..line_length * 2 + 1 {
        let step = i as f32;
        coordinates.push(Point::new(
            (start.x + x_step_size * step).floor(),
            (start.y + y_step_size * step).floor(),
            (start.z + z_step_size * step).floor(),
        ));
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
        get_touched_points_between(start.clone(), end.clone());
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
