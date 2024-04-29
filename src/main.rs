use template::math;

fn main() {
    let x: i32 = 3;
    let y: i32 = 5;

    let z = math::add_two_i32(x, y);

    println!("The sum of {} and {} is {}.", x, y, z);
}
