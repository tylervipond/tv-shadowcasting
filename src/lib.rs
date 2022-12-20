use std::collections::HashSet;

const MULT: [[i32; 8]; 4] = [
    [1, 0, 0, -1, -1, 0, 0, 1],
    [0, 1, -1, 0, 0, -1, 1, 0],
    [0, 1, 1, 0, 0, -1, -1, 0],
    [1, 0, 0, 1, -1, 0, 0, -1],
];

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Point {
    x: usize,
    y: usize,
}

fn cast_light(
    los_cache: &mut HashSet<Point>,
    get_allows_light: fn(Point) -> bool,
    cx: usize,
    cy: usize,
    row: usize,
    start: f32,
    end: f32,
    radius: u32,
    xx: i32,
    xy: i32,
    yx: i32,
    yy: i32,
) {
    let mut start = start;
    if start < end {
        return;
    }
    let radius_squared = radius * radius;
    for j in row..=radius as usize {
        let mut dx: f32 = -(j as f32) - 1.0;
        let dy: f32 = -(j as f32);
        let mut blocked = false;
        let mut new_start = None;
        while dx <= 0.0 {
            dx += 1.0;
            let map_x = cx as i32 + dx as i32 * xx + dy as i32 * xy;
            let map_y = cy as i32 + dx as i32 * yx + dy as i32 * yy;
            let map_point = Point {
                x: map_x as usize,
                y: map_y as usize,
            };
            let left_slope = (dx as f32 - 0.5) / (dy as f32 + 0.5);
            let right_slope = (dx as f32 + 0.5) / (dy as f32 - 0.5);
            if start < right_slope {
                continue;
            }
            if end > left_slope {
                break;
            }
            if dx * dx + dy * dy < radius_squared as f32 {
                los_cache.insert(map_point);
            }
            if blocked {
                if !get_allows_light(map_point) {
                    new_start = Some(right_slope);
                    continue;
                } else {
                    blocked = false;
                    start = new_start.unwrap()
                }
            } else {
                if !get_allows_light(map_point) && j < radius as usize {
                    blocked = true;
                    cast_light(
                        los_cache,
                        get_allows_light,
                        cx,
                        cy,
                        j + 1,
                        start,
                        left_slope,
                        radius,
                        xx,
                        xy,
                        yx,
                        yy,
                    );
                    new_start = Some(right_slope)
                }
            }
            if blocked {
                break;
            }
        }
    }
}

pub fn get_visible_points(
    vantage_point: Point,
    get_allows_light: fn(Point) -> bool,
    max_distance: u32,
) -> HashSet<Point> {
    let mut los_cache = HashSet::new();
    los_cache.insert(vantage_point);
    for region in 0..8 {
        cast_light(
            &mut los_cache,
            get_allows_light,
            vantage_point.x,
            vantage_point.y,
            1,
            1.0,
            0.0,
            max_distance,
            MULT[0][region],
            MULT[1][region],
            MULT[2][region],
            MULT[3][region],
        );
    }
    los_cache
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
