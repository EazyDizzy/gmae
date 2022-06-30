use lib::entity::point::Point;

pub fn can_see_from(start_position: Point, finish_point: Point) -> bool {
    let l = finish_point.x - start_position.x;
    let m = finish_point.y - start_position.y;
    let n = finish_point.z - start_position.z;
    false
}
