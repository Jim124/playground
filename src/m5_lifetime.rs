#[allow(dead_code, unused_variables)]
fn example_0() {
    let r: &i32;

    let x: i32 = 5;
    r = &x;

    println!("r: {}", r);
}

#[allow(dead_code, unused_variables)]
fn example_1() {
    // allocate space in memory
    let highest_age: i32;

    // initialize vars
    let alice_age: i32 = 20;
    {
        let bob_age: i32 = 21;
        highest_age = largest(&alice_age, &bob_age);
    }

    // call function

    //Print output
    println!("Highest age is {}", highest_age);

    fn largest(compare_1: &i32, compare_2: &i32) -> i32 {
        if compare_1 > compare_2 {
            *compare_1
        } else {
            *compare_2
        }
    }
}

#[allow(dead_code, unused_variables)]
fn example_2() {
    // allocate space in memory
    let highest_age: &i32;
    let new_age: i32;

    // initialize vars
    let alice_age: i32 = 20;
    {
        let bob_age: i32 = 21;
        // call function
        highest_age = largest(&alice_age, &bob_age);
        new_age = *highest_age;
    }

    //Print output
    println!("Highest age is {}", new_age);

    // rust use the shortest lifetime
    fn largest<'a, 'b: 'a>(compare_1: &'a i32, compare_2: &'b i32) -> &'a i32 {
        if compare_1 > compare_2 {
            compare_1
        } else {
            compare_2
        }
    }
}

#[allow(dead_code, unused_variables)]
fn example_3_generic() {
    // allocate space in memory
    let highest_age: &i32;
    let new_age: i32;

    // initialize vars
    let alice_age: i32 = 20;
    {
        let bob_age: i32 = 21;
        // call function
        highest_age = largest(&alice_age, &bob_age);
        new_age = *highest_age;
    }

    //Print output
    println!("Highest age is {}", new_age);

    // rust use the shortest lifetime
    fn largest<'a, T: PartialOrd>(compare_1: &'a T, compare_2: &'a T) -> &'a T {
        if compare_1 > compare_2 {
            compare_1
        } else {
            compare_2
        }
    }
}
#[allow(dead_code, unused_variables)]
struct Person<'p> {
    name: &'p str,
    point: &'p f32,
}

#[allow(dead_code, unused_variables)]
fn example_4_with_struct() {
    // allocate space in memory
    let highest_age: &f32;
    let new_age: f32;

    // initialize vars
    let alice = Person {
        name: "alice",
        point: &50.2,
    };
    {
        let bob = Person {
            name: "bob",
            point: &23.4,
        };
        // call function
        highest_age = largest(alice.point, bob.point);
        new_age = *highest_age;
    }

    //Print output
    println!("Highest age is {}", new_age);

    // rust use the shortest lifetime
    fn largest<'a, T: PartialOrd>(compare_1: &'a T, compare_2: &'a T) -> &'a T {
        if compare_1 > compare_2 {
            compare_1
        } else {
            compare_2
        }
    }
}
