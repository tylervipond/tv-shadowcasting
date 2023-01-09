#![feature(test)]

/**
 * This shadowcasting algorithm is more or less adapted from https://www.albertford.com/shadowcasting/
 */
extern crate test;

/**
 * Row is typed as [f32; 3] to squeeze some extra nano seconds
 * [0] = depth
 * [1] = start_slope
 * [2] = end_slope
 */
type Row = [f32; 3];

#[inline]
fn slope(depth: f32, column: i32) -> f32 {
    (2.0 * column as f32 - 1.0) / (2.0 * depth)
}

#[inline]
fn is_symmetric(depth: f32, start_slope: f32, end_slope: f32, col: f32) -> bool {
    col >= depth * start_slope && col <= depth * end_slope
}

#[inline]
fn transform_tile_to_map_position(
    depth: i32,
    col: i32,
    map_width: usize,
    position: usize,
    cardinal: Cardinal,
) -> usize {
    (match cardinal {
        Cardinal::North => position as i32 - depth * map_width as i32 + col,
        Cardinal::East => position as i32 + depth * map_width as i32 + col,
        Cardinal::West => position as i32 + col * map_width as i32 + depth,
        Cardinal::South => position as i32 + col * map_width as i32 - depth,
    }) as usize
}

#[inline]
fn get_next_row(depth: f32, start_slope: f32, end_slope: f32) -> Row {
    [depth + 1.0, start_slope, end_slope]
}

#[inline]
fn get_row_columns(depth: f32, start_slope: f32, end_slope: f32) -> std::ops::RangeInclusive<i32> {
    let min_col = f32::floor(depth * start_slope + 0.5) as i32;
    let max_col = f32::ceil(depth * end_slope - 0.5) as i32;
    min_col..=max_col
}

#[derive(Copy, Clone, Debug)]
pub enum Cardinal {
    North,
    East,
    West,
    South,
}

const CARDINALS: [Cardinal; 4] = [
    Cardinal::North,
    Cardinal::East,
    Cardinal::West,
    Cardinal::South,
];

fn scan(
    row: Row,
    cardinal: Cardinal,
    map: &Vec<u8>,
    map_width: usize,
    fov_map: &mut Vec<u8>,
    position: usize,
    radius: u32,
) {
    let mut rows: Vec<Row> = Vec::with_capacity(radius as usize);
    rows.push(row);
    while let Some(mut row) = rows.pop() {
        let [depth, start_slope, end_slope] = row;
        let mut previous_tile_blocks = None;
        for column in get_row_columns(depth, start_slope, end_slope) {
            let tile_position =
                transform_tile_to_map_position(depth as i32, column, map_width, position, cardinal);
            let distance = (position % map_width).abs_diff(tile_position % map_width)
                + (position / map_width).abs_diff(tile_position / map_width);
            let tile_blocks = map[tile_position] == 1;
            if distance < radius as usize
                && (tile_blocks || is_symmetric(depth, start_slope, end_slope, column as f32))
            {
                fov_map[tile_position] = 1;
            }
            if let Some(previous_tile_blocks) = previous_tile_blocks {
                if previous_tile_blocks && !tile_blocks {
                    row[1] = slope(depth, column);
                }
                if !previous_tile_blocks && tile_blocks {
                    // don't use the destructured values for the next row, mutations to the current row can happen and should be used
                    let mut next_row = get_next_row(row[0], row[1], row[2]);
                    next_row[2] = slope(depth, column);
                    rows.push(next_row);
                }
            }
            previous_tile_blocks = Some(tile_blocks);
        }
        if let Some(previous_tile_blocks) = previous_tile_blocks {
            if !previous_tile_blocks {
                // don't use the destructured values for the next row, mutations to the current row can happen and should be used
                rows.push(get_next_row(row[0], row[1], row[2]));
            }
        }
    }
}

pub fn get_visible_idxs(position: usize, map: &Vec<u8>, map_width: usize, radius: u32) -> Vec<u8> {
    let mut fov_map: Vec<u8> = vec![0; map.len()];
    fov_map[position] = 1;
    for cardinal in CARDINALS {
        let first_row = [1.0, -1.0, 1.0];
        scan(
            first_row,
            cardinal,
            map,
            map_width,
            &mut fov_map,
            position,
            radius,
        );
    }
    fov_map
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        #[rustfmt::skip]
        let map = vec![
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 0, 0, 0, 0, 0, 0, 1, 1,
            1, 1, 0, 0, 0, 0, 0, 0, 1, 1,
            1, 1, 0, 0, 0, 0, 0, 0, 1, 1,
            1, 1, 0, 0, 0, 0, 0, 0, 1, 1,
            1, 1, 0, 0, 0, 0, 0, 0, 1, 1,
            1, 1, 0, 0, 0, 0, 0, 0, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ];
        let map_width = 10;
        let position = 35;
        #[rustfmt::skip]
        let expected = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 1, 1, 1, 1, 1, 1, 1, 1, 0,
            0, 1, 1, 1, 1, 1, 1, 1, 1, 0,
            0, 1, 1, 1, 1, 1, 1, 1, 1, 0,
            0, 1, 1, 1, 1, 1, 1, 1, 1, 0,
            0, 1, 1, 1, 1, 1, 1, 1, 1, 0,
            0, 1, 1, 1, 1, 1, 1, 1, 1, 0,
            0, 1, 1, 1, 1, 1, 1, 1, 1, 0,
            0, 1, 1, 1, 1, 1, 1, 1, 1, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ];
        let result = get_visible_idxs(position, &map, map_width, 10);
        assert_eq!(result, expected);
    }

    #[test]

    fn it_occludes_the_north_west_corner() {
        #[rustfmt::skip]
        let map = vec![
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 0, 0, 0, 0, 0, 0, 1, 1,
            1, 1, 0, 1, 0, 0, 0, 0, 1, 1,
            1, 1, 0, 0, 0, 0, 0, 0, 1, 1,
            1, 1, 0, 0, 0, 0, 0, 0, 1, 1,
            1, 1, 0, 0, 0, 0, 0, 0, 1, 1,
            1, 1, 0, 0, 0, 0, 0, 0, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ];
        let map_width = 10;
        let position = 45;
        #[rustfmt::skip]
        let expected = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 1, 1, 1, 1, 1, 1, 1, 1, 0,
            0, 0, 0, 1, 1, 1, 1, 1, 1, 0,
            0, 1, 0, 1, 1, 1, 1, 1, 1, 0,
            0, 1, 1, 1, 1, 1, 1, 1, 1, 0,
            0, 1, 1, 1, 1, 1, 1, 1, 1, 0,
            0, 1, 1, 1, 1, 1, 1, 1, 1, 0,
            0, 1, 1, 1, 1, 1, 1, 1, 1, 0,
            0, 1, 1, 1, 1, 1, 1, 1, 1, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ];
        let result = get_visible_idxs(position, &map, map_width, 10);
        assert_eq!(result, expected);
    }

    #[bench]
    fn bench_corner(b: &mut Bencher) {
        #[rustfmt::skip]
        let map = vec![
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 0, 0, 0, 0, 0, 0, 1, 1,
            1, 1, 0, 1, 0, 0, 0, 0, 1, 1,
            1, 1, 0, 0, 0, 0, 0, 0, 1, 1,
            1, 1, 0, 0, 0, 0, 0, 0, 1, 1,
            1, 1, 0, 0, 0, 0, 0, 0, 1, 1,
            1, 1, 0, 0, 0, 0, 0, 0, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ];
        let map_width = 10;
        let position = 45;
        b.iter(|| get_visible_idxs(position, &map, map_width, 10));
    }
}
