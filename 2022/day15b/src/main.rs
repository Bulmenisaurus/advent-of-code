use std::cmp::{max, min};

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

    // the saddest line of code I've written in a while... :'(
    todo!()
}
