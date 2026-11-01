const TEST_ADD_OVERFLOW: bool = false;
const TEST_MULTIPLY_OVERFLOW: bool = false;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MathError {
    Overflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CharError {
    NonAscii,
    NotDigit,
    NotHexDigit,
    NotLetter,
    NotPrintable,
}

// Problema 1

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }

    let mut divisor = 2;

    while divisor * divisor <= n {
        if n.is_multiple_of(divisor) {
            return false;
        }

        divisor += 1;
    }

    true
}

fn next_prime(x: u16) -> Option<u16> {
    let mut candidate = u32::from(x) + 1;

    while candidate <= u32::from(u16::MAX) {
        if is_prime(candidate) {
            return Some(candidate as u16);
        }

        candidate += 1;
    }

    None
}

// Problema 2

fn add_or_panic(a: u32, b: u32) -> u32 {
    match a.checked_add(b) {
        Some(result) => result,
        None => panic!("Overflow la adunare"),
    }
}

fn multiply_or_panic(a: u32, b: u32) -> u32 {
    match a.checked_mul(b) {
        Some(result) => result,
        None => panic!("Overflow la înmulțire"),
    }
}

// Problema 3

fn checked_add(a: u32, b: u32) -> Result<u32, MathError> {
    match a.checked_add(b) {
        Some(result) => Ok(result),
        None => Err(MathError::Overflow),
    }
}

fn checked_multiply(a: u32, b: u32) -> Result<u32, MathError> {
    match a.checked_mul(b) {
        Some(result) => Ok(result),
        None => Err(MathError::Overflow),
    }
}

fn add_then_multiply(a: u32, b: u32, c: u32) -> Result<u32, MathError> {
    let sum = checked_add(a, b)?;
    checked_multiply(sum, c)
}

// Problema 4

fn to_uppercase(c: char) -> Result<String, CharError> {
    if !c.is_alphabetic() {
        return Err(CharError::NotLetter);
    }

    Ok(c.to_uppercase().collect())
}

fn to_lowercase(c: char) -> Result<String, CharError> {
    if !c.is_alphabetic() {
        return Err(CharError::NotLetter);
    }

    Ok(c.to_lowercase().collect())
}

fn print_char(c: char) -> Result<(), CharError> {
    if c.is_control() {
        return Err(CharError::NotPrintable);
    }

    print!("{c}");
    Ok(())
}

fn char_to_number(c: char) -> Result<u32, CharError> {
    if !c.is_ascii() {
        return Err(CharError::NonAscii);
    }

    match c.to_digit(10) {
        Some(number) => Ok(number),
        None => Err(CharError::NotDigit),
    }
}

fn char_to_number_hex(c: char) -> Result<u32, CharError> {
    if !c.is_ascii() {
        return Err(CharError::NonAscii);
    }

    match c.to_digit(16) {
        Some(number) => Ok(number),
        None => Err(CharError::NotHexDigit),
    }
}

fn print_error(error: CharError) {
    match error {
        CharError::NonAscii => println!("Eroare: caracterul nu este ASCII."),
        CharError::NotDigit => println!("Eroare: caracterul nu este o cifră."),
        CharError::NotHexDigit => {
            println!("Eroare: caracterul nu este o cifră hexazecimală.")
        }
        CharError::NotLetter => println!("Eroare: caracterul nu este o literă."),
        CharError::NotPrintable => {
            println!("Eroare: caracterul nu este imprimabil.")
        }
    }
}

// Problema 5

#[derive(Debug)]
enum CalculatorError {
    InvalidNumber,
    Overflow,
}

fn calculate_sum(a: &str, b: &str) -> Result<u32, CalculatorError> {
    let first_number = match a.parse::<u32>() {
        Ok(number) => number,
        Err(_) => return Err(CalculatorError::InvalidNumber),
    };

    let second_number = match b.parse::<u32>() {
        Ok(number) => number,
        Err(_) => return Err(CalculatorError::InvalidNumber),
    };

    match checked_add(first_number, second_number) {
        Ok(sum) => Ok(sum),
        Err(_) => Err(CalculatorError::Overflow),
    }
}

fn main() {
    // Problema 1
    println!("După 10: {:?}", next_prime(10));
    println!("După 65521: {:?}", next_prime(65_521));

    let mut current = 0;

    while let Some(prime) = next_prime(current) {
        if prime > 1000 {
            break;
        }

        print!("{prime} ");
        current = prime;
    }

    println!();

    // Problema 2
    println!("10 + 20 = {}", add_or_panic(10, 20));
    println!("6 * 7 = {}", multiply_or_panic(6, 7));

    if TEST_ADD_OVERFLOW {
        add_or_panic(u32::MAX, 1);
    }

    if TEST_MULTIPLY_OVERFLOW {
        multiply_or_panic(u32::MAX, 2);
    }

    // Problema 3
    println!("Adunare reușită: {:?}", checked_add(10, 20));
    println!("Overflow la adunare: {:?}", checked_add(u32::MAX, 1));

    println!("Înmulțire reușită: {:?}", checked_multiply(6, 7));
    println!("Overflow la înmulțire: {:?}", checked_multiply(u32::MAX, 2));

    println!("Adunare apoi înmulțire: {:?}", add_then_multiply(10, 5, 2));
    println!("Eroare propagată: {:?}", add_then_multiply(u32::MAX, 1, 2));

    // Problema 4: to_uppercase.
    match to_uppercase('a') {
        Ok(result) => println!("Majusculă: {result}"),
        Err(error) => print_error(error),
    }

    match to_uppercase('1') {
        Ok(result) => println!("Majusculă: {result}"),
        Err(error) => print_error(error),
    }

    // to_lowercase.
    match to_lowercase('B') {
        Ok(result) => println!("Minusculă: {result}"),
        Err(error) => print_error(error),
    }

    match to_lowercase('-') {
        Ok(result) => println!("Minusculă: {result}"),
        Err(error) => print_error(error),
    }

    // print_char.
    match print_char('A') {
        Ok(()) => println!(" <- caracter afișat"),
        Err(error) => print_error(error),
    }

    match print_char('\n') {
        Ok(()) => println!("Caracter afișat"),
        Err(error) => print_error(error),
    }

    // char_to_number.
    println!("Cifra 7: {:?}", char_to_number('7'));

    match char_to_number('x') {
        Ok(number) => println!("Număr: {number}"),
        Err(error) => print_error(error),
    }

    // char_to_number_hex.
    println!("Cifra hex F: {:?}", char_to_number_hex('F'));

    match char_to_number_hex('G') {
        Ok(number) => println!("Număr hex: {number}"),
        Err(error) => print_error(error),
    }

    // Problema 5: mini-calculator.
    println!("12 + 30: {:?}", calculate_sum("12", "30"));
    println!("Text invalid: {:?}", calculate_sum("abc", "30"));
    println!(
        "Overflow în calculator: {:?}",
        calculate_sum("4294967295", "1")
    );
}
