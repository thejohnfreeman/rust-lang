#[derive(Debug)]
enum UsState {
    Alaska,
    Texas,
}

struct A {
    a: u32,
    b: u32,
}

struct B(u32, u32,);

fn main() {
    println!("Hello, world!");
    println!("{}", if 1 == 2 { "equal" } else { "not equal" });
    let state = if 1 == 2 { UsState::Alaska } else { UsState::Texas };
    println!("{:?}", state);
    let x = 5.clone();
    println!("{}", x);
    B(1, 2);
    A{a: 1, b: 2};
}
