pub enum InvalidMoveError {
    OutOfBounds,
}

pub enum ParseError {
    InvalidMarker,
}

#[derive(Debug, Hash, PartialEq)]
pub struct Point {
    // We only consider valid points inside the map, hence positive coordinates
    x: usize,
    y: usize,
}

impl Eq for Point {}

impl Point {
    fn from_coords(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

#[derive(Debug)]
pub struct Map {
    width: usize,
    height: usize,
    obstacles: Vec<Point>,
}

fn is_obstacle(point: &Point, map: &Map) -> bool {
    map.obstacles.contains(point)
}

type Direction = (isize, isize);

fn move_(point: &Point, direction: &Direction, map: &Map) -> Result<Point, InvalidMoveError> {
    if let (Some(new_x), Some(new_y)) = (
        point.x.checked_add_signed(direction.0),
        point.y.checked_add_signed(direction.1),
    ) {
        if (new_x < map.width) & (new_y < map.height) {
            return Ok(Point::from_coords(new_x, new_y));
        }
    }
    Err(InvalidMoveError::OutOfBounds)
}

fn turn_direction(direction: &Direction) -> Direction {
    let (old_dx, old_dy) = (direction.0, direction.1);
    return (-old_dy, old_dx);
}

pub fn walk(map: Map, start: Point, initial_direction: Direction) -> Vec<Point> {
    let mut path: Vec<Point> = Vec::new();
    let mut direction = initial_direction.clone();

    path.push(start);
    loop {
        if let Ok(next_point) = move_(path.last().unwrap(), &direction, &map) {
            if is_obstacle(&next_point, &map) {
                // Turn direction and do nothing this time
                direction = turn_direction(&direction);
            } else {
                path.push(next_point);
            }
        } else {
            // Out of bounds, we're out
            break;
        }
    }
    return path;
}

fn find_marker(line: &str) -> Option<(usize, Direction)> {
    let valid_markers = ['^', '>', 'v', '<'];
    if let Some(index) = line.find(|c| valid_markers.contains(&c)) {
        let marker = line.chars().nth(index).unwrap();
        return match marker {
            '^' => Some((index, (0, -1))),
            '>' => Some((index, (1, 0))),
            'v' => Some((index, (0, 1))),
            '<' => Some((index, (-1, 0))),
            _ => unreachable!(),
        };
    }
    None
}

pub fn parse_input_06(contents: String) -> (Map, Point, Direction) {
    let lines = contents.lines();
    let mut obstacles: Vec<Point> = Vec::new();
    let mut initial_position = Point::from_coords(0, 0);
    let mut initial_direction = (0, 0);
    let mut width = 0;

    let mut current_line = 0;
    for line in lines {
        let obs_x = line.match_indices("#");
        obstacles.append(
            &mut obs_x
                .map(|m| Point::from_coords(m.0, current_line))
                .collect::<Vec<Point>>(),
        );

        if let Some((initial_x, direction)) = find_marker(&line) {
            if initial_direction != (0, 0) {
                panic!("Direction was already set");
            }
            initial_position.x = initial_x;
            initial_position.y = current_line;
            initial_direction = direction;
        }
        if width == 0 {
            width = line.len();
        }
        current_line += 1;
    }

    let map = Map {
        width,
        height: current_line,
        obstacles,
    };
    return (map, initial_position, initial_direction);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn turn_direction_works() {
        let direction = (1, 0);
        let direction_rot1 = turn_direction(&direction);
        let direction_rot2 = turn_direction(&direction_rot1);
        let direction_rot3 = turn_direction(&direction_rot2);
        let direction_rot4 = turn_direction(&direction_rot3);

        assert_eq!(direction_rot1, (0, 1));
        assert_eq!(direction_rot2, (-1, 0));
        assert_eq!(direction_rot3, (0, -1));
        assert_eq!(direction_rot4, direction);
    }

    #[test]
    fn walk_basic_works() {
        let map = Map {
            width: 3,
            height: 3,
            obstacles: vec![Point::from_coords(0, 2)],
        };
        let start = Point::from_coords(2, 2);
        let expected_path = vec![
            Point::from_coords(2, 2),
            Point::from_coords(1, 2),
            Point::from_coords(1, 1),
            Point::from_coords(1, 0),
        ];

        let path = walk(map, start, (-1, 0));

        assert_eq!(path, expected_path);
    }
}
