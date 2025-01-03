#[macro_export]
macro_rules! my_vec1 {
  ($($x: expr),+) => {
      {
        let mut temp_vec = Vec::new();
        $(
          temp_vec.push($x);
        )+
        temp_vec
      }
  };
}


#[cfg(test)]

// macro captures
/*
expr 
matches to a valid rust expression
"hello".to_string(), 
*/

/*
  stmt
  matches to a rust statement
  let x = 5, x.push(1), return Some(x)
*/

/*
  ident
  matches to a rust identifier
  variable name function name module name
*/
/*
  ty 
  matches to a rust type
  i32,u8 &str
*/

/*
  path
  matches to a rust path
  std::collections::HashMap;
*/

/*
  repetition specifier

  * - match zero or more repetition
  + - match one or more repetition
  ? - match zero or one repetition
*/
mod test{
  #[macro_export]
  macro_rules! mad_skills {
      ($x: expr) => {
          format!("You sent an expression: {}",$x)
      };
  }
  macro_rules! mad_skills_1 {
      ($x:ty) => {
          match stringify!($x){
            "i32" => "You sent an i32 type".to_string(),
            _ => "You sent something else.".to_string(),
          }
      };
  }
  macro_rules! my_vec {
      ($($x: expr),+) => {
          {
            let mut temp_vec = Vec::new();
            $(
              temp_vec.push($x);
            )+
            temp_vec
          }
      };
  }
  #[test]
  fn tests_declarative_macro(){
    let x = vec![1,2,3];
    let formatted = format!("hello 3 with vec {:?}",x);
    dbg!(formatted);
    let some_var = mad_skills!(1 +2);
    println!("{}",some_var );
    let some_var1 = mad_skills_1!(i32);
    dbg!(some_var1);
    let mut y = my_vec!(1,2);
    y.push(3);
    dbg!(y);
  }
}