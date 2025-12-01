macro_rules! add {
    ($a:expr, $b: expr) => {
        $a + $b
    };
}

macro_rules! add_as {
    ($a: expr, $b: expr, $t: ty) => {
        $a as $t + $b as $t
    };
}

fn main() {
    println!("Hello, world!");
    // let x: i32 = add!(1, 2);
    println!("{}", add!(1, 2));
    let x: i32 = 1;
    println!("{}", add_as!(x, 2, u8));
}
