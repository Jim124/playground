trait Attacker {
    fn choose_style(&self) -> String;
}

enum Character {
    Warrior,
    Archer,
    Wizard,
}
impl Attacker for Character {
    fn choose_style(&self) -> String {
        match self {
            Character::Warrior => "wing chun".to_string(),
            Character::Archer => "kung fu".to_string(),
            Character::Wizard => "thia chi".to_string(),
        }
    }
}

#[cfg(test)]

mod test {
    use super::*;
    #[test]
    fn tests_traits() {
        let character_string = Character::Archer.choose_style();
        dbg!(character_string);
    }
}
