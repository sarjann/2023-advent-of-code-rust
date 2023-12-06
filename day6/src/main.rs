use utils::load;

fn get_nums_from_line(line: String) -> Vec<i64> {
    let values: Vec<i64> = line
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    values
}

fn get_points(time: i64, distance: i64) -> i64 {
    // Solving quadratic equation and distance between intercepts
    println!("time:{time}-distance:{distance}");
    let quad_sqrt_term = ((time.pow(2) - 4 * distance) as f64).sqrt();

    let mut lb_float: f64 = ((time as f64) - quad_sqrt_term) / 2.;
    let mut ub_float: f64 = ((time as f64) + quad_sqrt_term) / 2.;

    // Have to account for float ends
    if lb_float.fract() == 0.0 {
        lb_float += 1.;
    };

    if ub_float.fract() == 0.0 {
        ub_float -= 1.;
    };

    let lb = lb_float.ceil() as i64;
    let ub = ub_float.floor() as i64;

    let range: i64 = ub - lb + 1;
    range
}

fn main() {
    let input = load("input.txt");
    let data: Vec<&str> = input.lines().collect();
    let times = get_nums_from_line(data[0].to_string());
    let distances = get_nums_from_line(data[1].to_string());

    let length = times.len();

    let mut total_points_p1: i64 = 1;
    for idx in 0..length {
        let time = times[idx];
        let distance = distances[idx];
        let points = get_points(time, distance);
        total_points_p1 *= points
    }

    println!("Total for Part 1 is {total_points_p1}")
}
