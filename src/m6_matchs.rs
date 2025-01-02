#[derive(Debug)]
enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}
fn process_message(message: Message) {
    match message {
        Message::Quit => {
            println!("I quit");
        }
        Message::ChangeColor(red, green, blue) => {
            println!("Red {}, Green {} Blue {}", red, green, blue);
        }
        Message::Move { x, y: new_name } => {
            println!("X is {}, Y as new name is {}", x, new_name);
        }
        Message::Write(text) => {
            println!("{}", text);
        }
    }
}

#[cfg(test)]

mod test {
    use super::*;
    #[test]
    fn tests_match_literals() {
        let num = 20;
        match num {
            10 => println!("It was the first"),
            1 | 20 | 3 | 5 | 7 => println!("we found it in the sequence"),
            _ => println!("It was something else"),
        }
    }

    #[test]
    fn tests_match_option() {
        let some_num: Option<i32> = Some(10);
        let prob_none: Option<i32> = None;
        // let res = match some_num{
        //   Some(i) => i,
        //   None => {panic!("There was a problem");}
        // };
        // println!("{:?}",some_num );
        // println!("{}",res );

        if let Some(i) = some_num {
            println!("{}", i);
        } else {
            panic!("There was a problem");
        }

        let my_int = if let Some(i) = some_num {
            i
        } else {
            panic!("There was a problem");
        };
        println!("my int: {}", my_int);
    }

    #[test]
    fn tests_match_result() {
        let some_res: Result<i32, &str> = Ok(50);
        let some_err: Result<i32, &str> = Err("There was en err");
        let res = match some_res {
            Ok(val) => val,
            Err(e) => panic!("{}", e),
        };
        println!("{}", res);

        let my_int = if let Ok(r) = some_res {
            r
        } else {
            panic!("There was a problem");
        };
        println!("my int: {}", my_int);
    }

    #[test]
    fn tests_match_enum() {
        let my_enum = Message::Quit;
        process_message(my_enum);
        let my_color = Message::ChangeColor(10, 30, 255);
        process_message(my_color);
        let my_move = Message::Move { x: 10, y: 20 };
        process_message(my_move);
        let my_write = Message::Write("this is a test".to_string());
        process_message(my_write);
    }

    #[test]
    fn tests_match_guard() {
        let pair = (2, -2);
        match pair {
            (x, y) if x == y => println!("they match1"),
            (x, y) if x + y == 0 => println!("they neutralize"),
            (_, y) if y == 2 => println!(" Y is indeed +2"),
            _ => println!("We are not bothered"),
        };
    }

    #[test]
    fn tests_match_struct() {
        struct Location {
            x: i32,
            y: i32,
        }
        let location = Location { x: 0, y: 20 };
        match location {
            Location { x, y: 0 } => println!("Y is on the axis"),
            Location { x: 0, y } => println!("X is on the axis"),
            Location { x, y } => println!("x and y are not on the axis"),
        };
    }
}
