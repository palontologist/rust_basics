pub fn main() {
    let is_sunny = true;
    let water_is_warm = false;
    let no_sharks = true;

    let should_go_swimming = is_sunny && water_is_warm && no_sharks;
    let is_safe_to_swim = water_is_warm || no_sharks;
    let is_sunny_but_not_warm = is_sunny && !water_is_warm;

    println!("Should I go swimming? {}", should_go_swimming);        // false
    println!("Is it safe to swim? {}", is_safe_to_swim);             // true
    println!("Is it sunny but not warm? {}", is_sunny_but_not_warm);  // true
}
