struct Point(u32, u32, u32);

/// Doc comment
// simple comment

fn main() {
    let y = Something::One(2);
    let x: Option<u32>;

    let a = 0x1 + 2 + 0o3 + 0b100;
    let b = 3;
    let mut c = a + b;

    let z: bool = true;

    let y: f32 = 2.98;

    let str = "This is an example of quoted text \n with excape characters!";
    let raw_str = r"This is an example of raw string\n";
    let chr = 'a';

    let reference: &usize;

    panic!();

    unsafe {
        let b: *mut u32;
    }

    let x = match y {
        Something::One(a) => a,
        Something::Two { first, second } => first as u32,
    };
}
