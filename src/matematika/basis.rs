#![allow(non_snake_case)]

/// Fungsi untuk mengonversi angka desimal ke basis lain (2-36).
/// Function to convert a decimal number to another base (2-36).
pub fn konversi_basis(mut num: u64, base: u32) -> String 
{
    if base < 2 || base > 36 {
        panic!("Basis harus antara 2 hingga 36"); // Base must be between 2 and 36
    }
    if num == 0 {
        return "0".to_string();
    }
    let mut hasil = String::new();
    while num > 0 
    {
        let rem = (num % base as u64) as u32;
        let digit = if rem < 10 {
            std::char::from_digit(rem, base).unwrap()
        } else {
            std::char::from_u32('A' as u32 + rem - 10).unwrap()
        };
        hasil.push(digit);
        num /= base as u64;
    }
    // Balik string hasil agar urutannya benar
    // Reverse the result string to get the correct order
    hasil.chars().rev().collect()
}

/// Fungsi untuk parsing string angka dari basis tertentu ke desimal.
/// Function to parse a string number from a given base to decimal.
pub fn parse_number(num_str: &str, from_base: u32) -> u64 
{
    if from_base < 2 || from_base > 36 {
        panic!("Basis harus antara 2 hingga 36"); // Base must be between 2 and 36
    }
    let mut result: u64 = 0;
    for c in num_str.chars() 
    {
        let digit = c
            .to_digit(from_base)
            .expect("Digit tidak valid untuk basis yang diberikan"); // Invalid digit for the given base
        result = result * (from_base as u64) + digit as u64;
    }
    result
}

/// === Fungsi konversi khusus antar basis ===
/// === Special conversion functions between bases ===

/// Konversi desimal ke biner
/// Convert decimal to binary
pub fn desimal_ke_biner(num: u64) -> String { konversi_basis(num, 2) }

/// Konversi desimal ke oktal
/// Convert decimal to octal
pub fn desimal_ke_oktal(num: u64) -> String { konversi_basis(num, 8) }

/// Konversi desimal ke hexadesimal
/// Convert decimal to hexadecimal
pub fn desimal_ke_hexadesimal(num: u64) -> String { konversi_basis(num, 16) }

/// Konversi biner ke desimal
/// Convert binary to decimal
pub fn biner_ke_desimal(num_str: &str) -> u64 { parse_number(num_str, 2) }

/// Konversi biner ke oktal
/// Convert binary to octal
pub fn biner_ke_oktal(num_str: &str) -> String 
{
    let dec = biner_ke_desimal(num_str);
    desimal_ke_oktal(dec)
}

/// Konversi biner ke hexadesimal
/// Convert binary to hexadecimal
pub fn biner_ke_hexadesimal(num_str: &str) -> String 
{
    let dec = biner_ke_desimal(num_str);
    desimal_ke_hexadesimal(dec)
}

/// Konversi hexadesimal ke desimal
/// Convert hexadecimal to decimal
pub fn hexadesimal_ke_desimal(num_str: &str) -> u64 { parse_number(num_str, 16) }

/// Konversi hexadesimal ke biner
/// Convert hexadecimal to binary
pub fn hexadesimal_ke_biner(num_str: &str) -> String 
{
    let dec = hexadesimal_ke_desimal(num_str);
    desimal_ke_biner(dec)
}

/// Konversi hexadesimal ke oktal
/// Convert hexadecimal to octal
pub fn hexadesimal_ke_oktal(num_str: &str) -> String 
{
    let dec = hexadesimal_ke_desimal(num_str);
    desimal_ke_oktal(dec)
}

/// Konversi oktal ke desimal
/// Convert octal to decimal
pub fn oktal_ke_desimal(num_str: &str) -> u64 { parse_number(num_str, 8) }

/// Konversi oktal ke biner
/// Convert octal to binary
pub fn oktal_ke_biner(num_str: &str) -> String 
{
    let dec = oktal_ke_desimal(num_str);
    desimal_ke_biner(dec)
}

/// Konversi oktal ke hexadesimal
/// Convert octal to hexadecimal
pub fn oktal_ke_hexadesimal(num_str: &str) -> String 
{
    let dec = oktal_ke_desimal(num_str);
    desimal_ke_hexadesimal(dec)
}
