fn add_space(s: &mut String, mut n: i32) {
    while n > 0 {
        s.push(' ');
        n -= 1;
    }
}
fn add_str(s: &mut String, st: &str) {
    s.push_str(st);
}

fn add_integer(s: &mut String, nr: i32) {
    let mut a = nr;
    if a < 0 {
        s.push('-');
        a = -a;
    }
    let mut temp = String::new();
    if a == 0 {
        temp.push('0');
    }

    while a > 0 {
        let digit = a % 10;
        let c = (digit as u8 + b'0') as char;
        temp.push(c);
        a /= 10;
    }

    let mut i = temp.len();

    while i > 0 {
        i -= 1;
        s.push(temp.as_bytes()[i] as char);
        if i > 0 && i.is_multiple_of(3) {
            s.push('_');
        }
    }
}
fn add_float(s: &mut String, nr: f64) {
    let n = (nr.abs() * 1000.0).round() as u64;

    if nr < 0.0 {
        s.push('-');
    }

    let int_part = n / 1000;
    let mut dec_part = n % 1000;

    add_integer(s, int_part as i32);

    if dec_part > 0 {
        s.push('.');

        let mut div = 100;

        while div > 0 {
            let digit = dec_part / div;
            s.push((digit as u8 + b'0') as char);
            dec_part %= div;
            div /= 10;
        }
    }
}

fn main() {
    let mut s = String::new();

    add_space(&mut s, 39);
    add_str(&mut s, "I 💚\n");

    add_space(&mut s, 39);
    add_str(&mut s, "RUST.\n\n");

    add_space(&mut s, 4);
    add_str(&mut s, "Most");

    add_space(&mut s, 12);
    add_str(&mut s, "crate");

    add_space(&mut s, 4);
    add_integer(&mut s, 306437968);

    add_space(&mut s, 8);
    add_str(&mut s, "and");

    add_space(&mut s, 4);
    add_str(&mut s, "lastest");

    add_space(&mut s, 7);
    add_str(&mut s, "is\n");

    add_space(&mut s, 9);
    add_str(&mut s, "downloaded");

    add_space(&mut s, 7);
    add_str(&mut s, "has");

    add_space(&mut s, 9);
    add_str(&mut s, "downloads");

    add_space(&mut s, 5);
    add_str(&mut s, "the");

    add_space(&mut s, 7);
    add_str(&mut s, "version");

    add_space(&mut s, 3);
    add_float(&mut s, 2.038);

    println!("{}", s);
}
