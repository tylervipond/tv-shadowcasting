# tv_shadowcasting
This is a shadow-casting implementation based on [this article](https://www.albertford.com/shadowcasting/) and optimized for maps represented as 1 dimensional u8 arrays. Given a map represented as 0s and 1s (1 for light-blocking), it will return a visibility map of 0s and 1s (1 for visible). This is primarily built for usage in my own games.

## API
*get_visible_idxs* - (position: usize, map: &Vec<u8>, map_width: usize, radius: u32) -> Vec<u8>

## Usage
The usage is simple:

```rust
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
```