4.1:

1.
fn main() {
    let x: i32 = 5;
    let mut y: u32 = 5;
    
    let z = 10; // Type of z ? 

    println!("Success!");
}

2.
fn main() {
    let v: u16 = 38_u8 as u16;

    println!("Success!");
}

3.
fn main() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    println!("Success!");
}
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

4.
fn main() {
    assert_eq!(i8::MAX, 127); 
    assert_eq!(u8::MAX, 255); 

    println!("Success!");
}

5.
fn main() {
   let v1 = 100_u8 + 50;              // 100 + 50 = 150 < 255
   let v2 = i8::checked_add(50, 30).unwrap(); // 50 + 30 = 80 < 127
   println!("{},{}", v1, v2);
}

6.
fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);

    println!("Success!");
}

7.
fn main() {
    let x = 1_000.000_1; // ?
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

8.
fn main() {
    let a: f64 = 0.1;
    let b: f64 = 0.2;
    let c: f64 = 0.3;

    assert!((a + b - c).abs() < f64::EPSILON);

    println!("Success!");
}

9.
fn main() {
    let mut sum = 0;
    for i in -3..0 {
        sum += i;
    }
    sum += 3;
    assert!(sum == -3);

    for c in 'a'..='z' {
        println!("{}", c);
    }
}

10.
use std::ops::{Range, RangeInclusive};

fn main() {
    assert_eq!((1..5), Range { start: 1, end: 5 });       
    assert_eq!((1..=5), RangeInclusive::new(1, 5));     

    println!("Success!");
}

11.

fn main() {
    assert!(1u32 + 2 == 3);
    assert!(1i32 - 2 == -1);
    assert!(1u8.wrapping_sub(2) == 255); 
    
    assert!(3 * 50 == 150);;

    assert!((9.6f32 / 3.2) == 3.0f32); 

    assert!(24 % 5 == 4);
    
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);
    
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
---------------------------------------------------------------------------------------------------------------------------------
4.2:

1.
use std::mem::size_of_val;

fn main() {
    let c1 = 'a';
    let c1_utf8 = c1.to_string();
    assert_eq!(size_of_val(c1_utf8.as_bytes()), 1);

    let c2 = '中';
    let c2_utf8 = c2.to_string();
    assert_eq!(size_of_val(c2_utf8.as_bytes()), 3);

    println!("Success!");
}

2.
fn main() {
    let c1 = '中';
    print_char(c1);
} 

fn print_char(c : char) {
    println!("{}", c);
}

3.
fn main() {
    let _f: bool = false;

    let t = true;
    if t {
        println!("Success!");
    }
} 

4.
fn main() {
    let f = true;
    let t = true && true;
    assert_eq!(t, f);

    println!("Success!");
}

5.
fn main() {
    let _v: () = ();

    let v = implicitly_ret_unit();
    assert_eq!(v, implicitly_ret_unit());

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

6.
use std::mem::size_of_val;

fn main() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!");
}
---------------------------------------------------------------------------------------------------------------------------------
4.3:

1.
fn main() {
    let v = {
        let mut x = 1;
        x += 2; 
        x     
    };

    assert_eq!(v, 3);
    println!("Success!");
}

2.

fn main() {
   let v = 3;

   assert!(v == 3);

   println!("Success!");
}

3.

fn main() {
    let s = sum(1 , 2);
    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
---------------------------------------------------------------------------------------------------------------------------------
4.4:

1.
fn main() {
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y 
}

2.
fn main() {
   print();
}
fn print() -> () {
   println!("Success!");
}

3.
fn main() {
    never_return();

    println!("Failed!");
}

fn never_return() -> ! {
    loop {} 
}

4.
fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
        }
        _ => {
        }
    };
    
    never_return_fn()
}

fn never_return_fn() -> ! {
    loop {}
}

5.
fn main() {
    let b = true;

    let _v = match b {
        true => 1,
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}

