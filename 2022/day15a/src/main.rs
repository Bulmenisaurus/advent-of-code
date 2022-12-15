fn manhattan(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
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

    let line_scan_height = 2000000;

    // count the left-most tile that is affected and the right_most

    let sensor_reading_distances = sensors
        .iter()
        .zip(beacons.clone())
        .map(|(sensor, beacon)| (sensor, manhattan(sensor.0, sensor.1, beacon.0, beacon.1)))
        .collect::<Vec<(&(i32, i32), i32)>>();

    let sensor_x_minimum: i32 = sensor_reading_distances
        .iter()
        .map(|(sensor, distance)| sensor.0 - distance)
        .min()
        .unwrap();

    let sensor_x_maximum: i32 = sensor_reading_distances
        .iter()
        .map(|(sensor, distance)| sensor.0 + distance)
        .max()
        .unwrap();

    let mut total_occupied = 0;

    for tile_x in sensor_x_minimum..=sensor_x_maximum {
        let point_check = (tile_x, line_scan_height);

        if beacons.contains(&point_check) || sensors.contains(&point_check) {
            continue;
        }

        let is_point_part_of_scan = sensor_reading_distances.iter().any(|(sensor, distance)| {
            manhattan(point_check.0, point_check.1, sensor.0, sensor.1) <= *distance
        });

        if is_point_part_of_scan {
            total_occupied += 1;
        }

        if tile_x % 10_000 == 0 {
            println!(
                "{}/{}",
                tile_x - sensor_x_minimum,
                sensor_x_maximum - sensor_x_minimum
            );
        }
    }

    println!("{}", total_occupied);
}
