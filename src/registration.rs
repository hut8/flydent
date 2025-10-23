/// ICAO <-> Registration (US "N")
///
/// - US block: 0xA00001 ..= 0xADF7C7
///
/// ICAO identifiers represented as [u8; 3], big-endian.
///
/// This implementation is based on the algorithm from:
/// https://github.com/guillaumemichel/icao-nnumber_converter
/// Copyright (c) Guillaume Michel, licensed under GPLv3

const US_BASE: u32 = 0xA00000;
const US_MAX: u32 = 0xADF7C7;

// Charset excludes 'I' and 'O' to avoid confusion with digits
const CHARSET: &str = "ABCDEFGHJKLMNPQRSTUVWXYZ";
const ALLCHARS: &str = "ABCDEFGHJKLMNPQRSTUVWXYZ0123456789";

// Bucket sizes for the tail number algorithm
const SUFFIX_SIZE: u32 = 1 + 24 * (1 + 24); // 601
const BUCKET4_SIZE: u32 = 1 + 24 + 10; // 35
const BUCKET3_SIZE: u32 = 10 * BUCKET4_SIZE + SUFFIX_SIZE; // 951
const BUCKET2_SIZE: u32 = 10 * BUCKET3_SIZE + SUFFIX_SIZE; // 10111
const BUCKET1_SIZE: u32 = 10 * BUCKET2_SIZE + SUFFIX_SIZE; // 101711

fn u32_to_arr3(x: u32) -> [u8; 3] {
    [(x >> 16) as u8, (x >> 8) as u8, x as u8]
}

fn arr3_to_u32(a: [u8; 3]) -> u32 {
    ((a[0] as u32) << 16) | ((a[1] as u32) << 8) | (a[2] as u32)
}

/// Get the suffix string for a given offset (0-600)
/// 0 -> ''
/// 1 -> 'A'
/// 2 -> 'AA'
/// 3 -> 'AB'
/// ...
/// 600 -> 'ZZ'
fn get_suffix(offset: u32) -> String {
    if offset == 0 {
        return String::new();
    }
    let char0_idx = ((offset - 1) / 25) as usize;
    let rem = (offset - 1) % 25;
    let char0 = CHARSET.chars().nth(char0_idx).unwrap();
    if rem == 0 {
        return char0.to_string();
    }
    let char1 = CHARSET.chars().nth((rem - 1) as usize).unwrap();
    format!("{}{}", char0, char1)
}

/// Get the offset for a given suffix string
/// Reverse of get_suffix()
fn suffix_offset(s: &str) -> Option<u32> {
    if s.is_empty() {
        return Some(0);
    }
    if s.len() > 2 {
        return None;
    }

    let chars: Vec<char> = s.chars().collect();
    let idx0 = CHARSET.find(chars[0])?;
    let mut count = (25 * idx0 + 1) as u32;

    if chars.len() == 2 {
        let idx1 = CHARSET.find(chars[1])?;
        count += (idx1 + 1) as u32;
    }

    Some(count)
}

/// Convert a US N-Number to ICAO address (u32)
fn us_n_to_icao_u32(nnumber: &str) -> Result<u32, String> {
    let nnumber = nnumber.trim().to_ascii_uppercase();

    // Must start with 'N'
    if !nnumber.starts_with('N') {
        return Err("Must start with N".into());
    }

    if nnumber.len() > 6 {
        return Err("N-Number too long (max 6 chars)".into());
    }

    // Verify all characters are valid
    for c in nnumber.chars() {
        if !ALLCHARS.contains(c) {
            return Err(format!("Invalid character: {}", c));
        }
    }

    // Verify format: no letters in the middle (only at end)
    if nnumber.len() > 3 {
        let chars: Vec<char> = nnumber.chars().collect();
        for i in 1..(nnumber.len() - 2) {
            if CHARSET.contains(chars[i]) {
                return Err("Letters can only appear as suffix".into());
            }
        }
    }

    let mut count = 1u32; // Start at 1 (N1 = a00001)

    if nnumber.len() == 1 {
        // Just "N" = a00001
        return Ok(US_BASE + count);
    }

    let rest = &nnumber[1..];
    let chars: Vec<char> = rest.chars().collect();

    for i in 0..chars.len() {
        let c = chars[i];

        if i == 4 {
            // Last possible character (position 5 in N-Number, position 4 in rest)
            let idx = ALLCHARS.find(c).ok_or("Invalid character")?;
            count += (idx + 1) as u32;
            break;
        } else if CHARSET.contains(c) {
            // First alphabetical character - this is the suffix
            let suffix = &rest[i..];
            count += suffix_offset(suffix).ok_or("Invalid suffix")?;
            break;
        } else {
            // Digit
            let digit = c.to_digit(10).ok_or("Invalid digit")?;
            match i {
                0 => count += (digit - 1) * BUCKET1_SIZE,
                1 => count += digit * BUCKET2_SIZE + SUFFIX_SIZE,
                2 => count += digit * BUCKET3_SIZE + SUFFIX_SIZE,
                3 => count += digit * BUCKET4_SIZE + SUFFIX_SIZE,
                _ => return Err("N-Number format error".into()),
            }
        }
    }

    let icao = US_BASE + count;
    if icao > US_MAX {
        return Err("Out of US range".into());
    }

    Ok(icao)
}

/// Convert ICAO address (u32) to US N-Number
fn icao_u32_to_us(icao: u32) -> Result<String, String> {
    if !(US_BASE + 1..=US_MAX).contains(&icao) {
        return Err("Not in US allocation".into());
    }

    let i = icao - US_BASE - 1;
    let mut output = String::from("N");

    // Digit 1
    let dig1 = i / BUCKET1_SIZE + 1;
    let mut rem = i % BUCKET1_SIZE;
    output.push_str(&dig1.to_string());

    if rem < SUFFIX_SIZE {
        return Ok(output + &get_suffix(rem));
    }

    // Digit 2
    rem -= SUFFIX_SIZE;
    let dig2 = rem / BUCKET2_SIZE;
    rem = rem % BUCKET2_SIZE;
    output.push_str(&dig2.to_string());

    if rem < SUFFIX_SIZE {
        return Ok(output + &get_suffix(rem));
    }

    // Digit 3
    rem -= SUFFIX_SIZE;
    let dig3 = rem / BUCKET3_SIZE;
    rem = rem % BUCKET3_SIZE;
    output.push_str(&dig3.to_string());

    if rem < SUFFIX_SIZE {
        return Ok(output + &get_suffix(rem));
    }

    // Digit 4
    rem -= SUFFIX_SIZE;
    let dig4 = rem / BUCKET4_SIZE;
    rem = rem % BUCKET4_SIZE;
    output.push_str(&dig4.to_string());

    if rem == 0 {
        return Ok(output);
    }

    // Last character
    let last_char = ALLCHARS.chars().nth((rem - 1) as usize)
        .ok_or("Invalid last character")?;
    output.push(last_char);

    Ok(output)
}

// === Public API ===

pub fn registration_to_icao(reg: &str) -> Result<[u8; 3], String> {
    if reg.starts_with('N') {
        us_n_to_icao_u32(reg).map(u32_to_arr3)
    } else {
        Err("Unsupported registration prefix (only US 'N' supported)".into())
    }
}

pub fn icao_to_registration(icao: [u8; 3]) -> Result<String, String> {
    let icao_u32 = arr3_to_u32(icao);
    if (US_BASE + 1..=US_MAX).contains(&icao_u32) {
        icao_u32_to_us(icao_u32)
    } else {
        Err("ICAO not in US range".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn us_roundtrip() {
        let reg = "N456TS";
        let icao = registration_to_icao(reg).unwrap();
        assert_eq!(icao_to_registration(icao).unwrap(), reg);
    }

    #[test]
    fn us_icao_to_registration_ab8e4f() {
        // Convert ICAO code AB8E4F to registration N8437D
        let icao = [0xAB, 0x8E, 0x4F];
        let reg = icao_to_registration(icao).unwrap();
        assert_eq!(reg, "N8437D");
    }

    #[test]
    fn us_registration_to_icao_n8437d() {
        // Convert registration N8437D to ICAO code AB8E4F
        let reg = "N8437D";
        let icao = registration_to_icao(reg).unwrap();
        assert_eq!(icao, [0xAB, 0x8E, 0x4F]);
    }

    #[test]
    fn reject_other() {
        assert!(registration_to_icao("G-ABCD").is_err());
        assert!(icao_to_registration([0x00, 0x12, 0x34]).is_err());
    }

    #[test]
    fn test_n1() {
        // N1 should be the first valid registration
        let icao = registration_to_icao("N1").unwrap();
        assert_eq!(icao, [0xA0, 0x00, 0x01]);
        assert_eq!(icao_to_registration(icao).unwrap(), "N1");
    }

    #[test]
    fn test_n99999() {
        // N99999 should be the last valid registration
        let reg = "N99999";
        let icao = registration_to_icao(reg).unwrap();
        assert_eq!(icao, [0xAD, 0xF7, 0xC7]);
        assert_eq!(icao_to_registration(icao).unwrap(), reg);
    }
}
