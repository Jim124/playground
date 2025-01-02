#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u32,
    active: bool,
}

impl User {
    fn increment_sign_in_account(&mut self) {
        self.sign_in_count += 1;
    }
    fn change_email(&mut self, new_email: &str) {
        self.email = String::from(new_email);
    }
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn tests_structs() {
        let user_1 = User {
            username: String::from("jim"),
            email: String::from("example.com"),
            sign_in_count: 1,
            active: true,
        };
        dbg!(user_1);
        let mut user_2 = User {
            username: String::from("someusername"),
            email: String::from("example.com"),
            sign_in_count: 1,
            active: true,
        };
        user_2.increment_sign_in_account();
        user_2.change_email("another_new.email.com");
        dbg!(user_2);
    }
}
