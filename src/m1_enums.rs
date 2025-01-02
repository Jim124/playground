#[derive(Debug)]
enum CarColor {
    Red,
    Blue,
    Green,
    Silver,
}

fn create_car_color_blue() -> CarColor {
    let car_color_blue = CarColor::Blue;
    car_color_blue
}
fn check_result_built_in(num: u8) -> Result<u8, String> {
    if num < 5 {
        Ok(num)
    } else {
        Err("Not less 5".to_string())
    }
}
fn reminder_option_built_in(num: f32) -> Option<f32> {
    let reminder = num % 10.0;
    if reminder == 0.0 {
        Some(num)
    } else {
        None
    }
}

#[cfg(test)]

mod test {
    use super::*;
    #[test]
    fn tests_enums() {
        let car_color: CarColor = create_car_color_blue();
        dbg!(car_color);
        let check_result = check_result_built_in(4);
        dbg!(&check_result);
        if check_result.is_ok() {
            println!("the result is {}", check_result.unwrap());
        }
        let reminder = reminder_option_built_in(12.2);
        dbg!(reminder);
    }
}
