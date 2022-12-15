fn manhattan(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn points_between_lines(a: (i32, i32), b: (i32, i32)) -> Vec<(i32, i32)> {
    let x_distance = b.0 - a.0;
    let y_distance = b.1 - a.1;

    let distance = std::cmp::max(x_distance.abs(), y_distance.abs());

    debug_assert!(x_distance.abs() == y_distance.abs() || x_distance == 0 || y_distance == 0);

    (0..distance)
        .map(|i| {
            let x_offset = i * x_distance.signum();
            let y_offset = i * y_distance.signum();

            (a.0 + x_offset, a.1 + y_offset)
        })
        .collect()
}

fn main() {
    let mut sensors: Vec<(i32, i32)> = vec![];
    let mut beacons: Vec<(i32, i32)> = vec![];
    include_str!("./input.txt").lines().for_each(|reading| {
        let parsed_reading = reading
            .replace(":", "")
            .replace("x=", "")
            .replace("y=", "")
            .replace(",", "");

        let parsed_reading = parsed_reading.split(" ").collect::<Vec<&str>>();

        let sensor_x = parsed_reading[2].parse::<i32>().unwrap();
        let sensor_y = parsed_reading[3].parse::<i32>().unwrap();

        let beacon_x = parsed_reading[8].parse::<i32>().unwrap();
        let beacon_y = parsed_reading[9].parse::<i32>().unwrap();

        sensors.push((sensor_x, sensor_y));
        beacons.push((beacon_x, beacon_y));
    });

    let sensor_reading_distances = sensors
        .iter()
        .zip(beacons.clone())
        .map(|(sensor, beacon)| (sensor, manhattan(sensor.0, sensor.1, beacon.0, beacon.1)))
        .collect::<Vec<(&(i32, i32), i32)>>();

    // All beacons must outside 1 tile of an edge of an observed area. Why?
    // Suppose a beacon doesn't follow this rule. Where could it be?

    // 1) If the beacon is inside of a revealed area, then either the sensor would pick it up, or
    //    it would be the same distance, but ties are forbidden in the problem statement
    //
    // 2) If the beacon is >1 tile outside of an edge, than either:
    //
    //    2.a) There is another wall in between that wall and the beacon, in which case the
    //         original property is satisfied.
    //    2.b) Or the beacon is more than 1 tile away from every single wall, which would mean that
    //         there would be four other valid spaces around that beacon, in which case the
    //         property that there is only one tile which satisfies the condition is broken

    let mut i = 0;
    let mut candidate_tiles = sensor_reading_distances
        .iter()
        .filter_map(|(sensor, distance)| {
            i += 1;
            println!("[{}]", i);
            let offsets = [
                (0, distance + 1),
                ((distance + 1), 0),
                (0, -(distance + 1)),
                (-(distance + 1), 0),
            ];

            let outside_point_vertices = offsets
                .iter()
                .map(|(x, y)| (sensor.0 + x, sensor.1 + y))
                .collect::<Vec<(i32, i32)>>();

            let outside_points = (0..outside_point_vertices.len() + 1)
                .map(|i| {
                    let start_i = i % outside_point_vertices.len();
                    let end_i = (i + 1) % outside_point_vertices.len();

                    let start_point = outside_point_vertices[start_i];
                    let end_point = outside_point_vertices[end_i];

                    let points_along_line = points_between_lines(start_point, end_point);

                    points_along_line
                })
                .flatten();

            let mut candidates = outside_points.filter(|point| {
                let matches_bound_check =
                    point.0 >= 0 && point.0 <= 4000000 && point.1 >= 0 && point.1 <= 4000000;
                if !matches_bound_check {
                    return false;
                }

                // check if our candidate is in the range of one of the sensors
                let is_point_in_range =
                    sensor_reading_distances.iter().any(|(sensor, distance)| {
                        manhattan(point.0, point.1, sensor.0, sensor.1) <= *distance
                    });

                // the candidate point is not in the range of any beacons

                return !is_point_in_range;
            });

            match candidates.nth(0) {
                None => None,
                Some(item) => Some(item.clone()),
            }
        })
        .collect::<Vec<(i32, i32)>>();

    candidate_tiles.dedup();

    assert!(candidate_tiles.len() == 1);

    let result_point = candidate_tiles[0];

    let result_x: u64 = result_point.0 as u64;
    let result_y: u64 = result_point.1 as u64;

    println!("{:?}", result_x * 4000000 + result_y);
}
