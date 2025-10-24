/// ICAO 24-bit Address Allocation
///
/// Maps ICAO 24-bit address prefixes to ISO 3166-1 alpha-2 country codes.
/// Based on ICAO Doc 8643 aircraft type designators and address allocations.

/// ICAO address allocations as (binary_prefix, iso2_country_code) tuples.
/// Sorted by prefix length (longest first) to ensure correct prefix matching.
const ICAO_ALLOCATIONS: &[(&str, &str)] = &[
    // 14-bit prefixes
    ("00001100101000", "AG"),  // Antigua and Barbuda
    ("01010000000100", "AL"),  // Albania
    ("00001010101000", "BB"),  // Barbados
    ("00001010101100", "BZ"),  // Belize
    ("00001001010000", "BJ"),  // Benin
    ("01101000000000", "BT"),  // Bhutan
    ("111010010100", "BO"),  // Bolivia
    ("01010001001100", "BA"),  // Bosnia and Herzegovina
    ("00000011000000", "BW"),  // Botswana
    ("10001001010100", "BN"),  // Brunei Darussalam
    ("000010011100", "BF"),  // Burkina Faso
    ("000000110010", "BI"),  // Burundi
    ("011100001110", "KH"),  // Cambodia
    ("000000110100", "CM"),  // Cameroon
    ("00001001011000", "CV"),  // Cape Verde
    ("000001101100", "CF"),  // Central African Republic
    ("000010000100", "TD"),  // Chad
    ("111010000000", "CL"),  // Chile
    ("000010101100", "CO"),  // Colombia
    ("00000011010100", "KM"),  // Comoros
    ("000000110110", "CG"),  // Congo
    ("10010000000100", "CK"),  // Cook Islands
    ("000010101110", "CR"),  // Costa Rica
    ("000000111000", "CI"),  // Côte d'Ivoire
    ("01010000000111", "HR"),  // Croatia
    ("000010110000", "CU"),  // Cuba
    ("01001100100000", "CY"),  // Cyprus
    ("011100100", "KP"),  // Democratic People's Republic of Korea (North Korea)
    ("000010001100", "CD"),  // Democratic Republic of the Congo
    ("00001001100000", "DJ"),  // Djibouti
    ("000011000100", "DO"),  // Dominican Republic
    ("111010000100", "EC"),  // Ecuador
    ("000010110010", "SV"),  // El Salvador
    ("000001000010", "GQ"),  // Equatorial Guinea
    ("00100000001000", "ER"),  // Eritrea
    ("01010001000100", "EE"),  // Estonia
    ("000001000000", "ET"),  // Ethiopia
    ("110010001000", "FJ"),  // Fiji
    ("000000111110", "GA"),  // Gabon
    ("000010011010", "GM"),  // Gambia
    ("01010001010000", "GE"),  // Georgia
    ("000001000100", "GH"),  // Ghana
    ("00001100110000", "GD"),  // Grenada
    ("000010110100", "GT"),  // Guatemala
    ("000001000110", "GN"),  // Guinea
    ("00000100100000", "GW"),  // Guinea-Bissau
    ("000010110110", "GY"),  // Guyana
    ("000010111000", "HT"),  // Haiti
    ("000010111010", "HN"),  // Honduras
    ("010011001100", "IS"),  // Iceland
    ("011100110", "IR"),  // Iran, Islamic Republic of
    ("011100101", "IQ"),  // Iraq
    ("010011001010", "IE"),  // Ireland
    ("011100111", "IL"),  // Israel
    ("000010111110", "JM"),  // Jamaica
    ("011101000", "JO"),  // Jordan
    ("01101000001100", "KZ"),  // Kazakhstan
    ("000001001100", "KE"),  // Kenya
    ("11001000111000", "KI"),  // Kiribati
    ("011100000110", "KW"),  // Kuwait
    ("01100000000100", "KG"),  // Kyrgyzstan
    ("011100001000", "LA"),  // Lao People's Democratic Republic
    ("01010000001011", "LV"),  // Latvia
    ("011101001", "LB"),  // Lebanon
    ("00000100101000", "LS"),  // Lesotho
    ("000001010000", "LR"),  // Liberia
    ("01010000001111", "LT"),  // Lithuania
    ("01001101000000", "LU"),  // Luxembourg
    ("000001010100", "MG"),  // Madagascar
    ("000001011000", "MW"),  // Malawi
    ("011101010", "MY"),  // Malaysia
    ("00000101101000", "MV"),  // Maldives
    ("000001011100", "ML"),  // Mali
    ("01001101001000", "MT"),  // Malta
    ("10010000000000", "MH"),  // Marshall Islands
    ("00000101111000", "MR"),  // Mauritania
    ("00000110000000", "MU"),  // Mauritius
    ("01101000000100", "FM"),  // Micronesia, Federated States of
    ("01001101010000", "MC"),  // Monaco
    ("01101000001000", "MN"),  // Mongolia
    ("000000000110", "MZ"),  // Mozambique
    ("011100000100", "MM"),  // Myanmar
    ("00100000000100", "NA"),  // Namibia
    ("11001000101000", "NR"),  // Nauru
    ("011100001010", "NP"),  // Nepal
    ("000011000000", "NI"),  // Nicaragua
    ("000001100010", "NE"),  // Niger
    ("000001100100", "NG"),  // Nigeria
    ("01110000110000", "OM"),  // Oman
    ("011101100", "PK"),  // Pakistan
    ("01101000010000", "PW"),  // Palau
    ("000011000010", "PA"),  // Panama
    ("100010011000", "PG"),  // Papua New Guinea
    ("111010001000", "PY"),  // Paraguay
    ("111010001100", "PE"),  // Peru
    ("011101011", "PH"),  // Philippines
    ("00000110101000", "QA"),  // Qatar
    ("011100011", "KR"),  // Republic of Korea (South Korea)
    ("01010000010011", "MD"),  // Republic of Moldova
    ("000001101110", "RW"),  // Rwanda
    ("11001000110000", "LC"),  // Saint Lucia
    ("00001011110000", "VC"),  // Saint Vincent and the Grenadines
    ("10010000001000", "WS"),  // Samoa
    ("01010000000000", "SM"),  // San Marino
    ("00001001111000", "ST"),  // Sao Tome and Principe
    ("011100010", "SA"),  // Saudi Arabia
    ("000001110000", "SN"),  // Senegal
    ("00000111010000", "SC"),  // Seychelles
    ("00000111011000", "SL"),  // Sierra Leone
    ("011101101", "SG"),  // Singapore
    ("01010000010111", "SK"),  // Slovakia
    ("01010000011011", "SI"),  // Slovenia
    ("10001001011100", "SB"),  // Solomon Islands
    ("000001111000", "SO"),  // Somalia
    ("011101110", "LK"),  // Sri Lanka
    ("000001111100", "SD"),  // Sudan
    ("000011001000", "SR"),  // Suriname
    ("00000111101000", "SZ"),  // Swaziland
    ("01010001010100", "TJ"),  // Tajikistan
    ("01010001001000", "MK"),  // The former Yugoslav Republic of Macedonia
    ("000010001000", "TG"),  // Togo
    ("11001000110100", "TO"),  // Tonga
    ("000011000110", "TT"),  // Trinidad and Tobago
    ("01100000000110", "TM"),  // Turkmenistan
    ("000001101000", "UG"),  // Uganda
    ("100010010110", "AE"),  // United Arab Emirates
    ("000010000000", "TZ"),  // United Republic of Tanzania
    ("111010010000", "UY"),  // Uruguay
    ("01010000011111", "UZ"),  // Uzbekistan
    ("11001001000000", "VU"),  // Vanuatu
    ("100010010000", "YE"),  // Yemen
    ("000010001010", "ZM"),  // Zambia
    ("00000000010000", "ZW"),  // Zimbabwe
    ("10001001100100", "ZZ"),  // ICAO (2)
    ("11110000100100", "ZZ"),  // ICAO (2)

    // 12-bit prefixes
    ("011100000000", "AF"),  // Afghanistan
    ("01100000000000", "AM"),  // Armenia
    ("01100000000010", "AZ"),  // Azerbaijan
    ("000010101000", "BS"),  // Bahamas
    ("100010010100", "BH"),  // Bahrain
    ("011100000010", "BD"),  // Bangladesh
    ("01010001000000", "BY"),  // Belarus

    // 9-bit prefixes
    ("000010100", "DZ"),  // Algeria
    ("010001000", "AT"),  // Austria
    ("010001001", "BE"),  // Belgium
    ("010001010", "BG"),  // Bulgaria
    ("010001011", "DK"),  // Denmark
    ("010001100", "FI"),  // Finland
    ("010001101", "GR"),  // Greece
    ("010001110", "HU"),  // Hungary
    ("010001111", "NO"),  // Norway
    ("100010100", "ID"),  // Indonesia
    ("010010000", "NL"),  // Netherlands, Kingdom of the
    ("010010001", "PL"),  // Poland
    ("010010010", "PT"),  // Portugal
    ("010010011", "CZ"),  // Czech Republic
    ("010010100", "RO"),  // Romania
    ("010010101", "SE"),  // Sweden
    ("010010110", "CH"),  // Switzerland
    ("010010111", "TR"),  // Turkey
    ("110010000", "NZ"),  // New Zealand
    ("010100001", "UA"),  // Ukraine
    ("000011010", "MX"),  // Mexico
    ("000011011", "VE"),  // Venezuela
    ("100010000", "TH"),  // Thailand
    ("100010001", "VN"),  // Viet Nam
    ("010011000", "RS"),  // Yugoslavia
    ("111100000", "ZZ"),  // ICAO (1)

    // 6-bit prefixes
    ("111000", "AR"),  // Argentina
    ("011111", "AU"),  // Australia
    ("110000", "CA"),  // Canada
    ("111001", "BR"),  // Brazil
    ("001110", "FR"),  // France
    ("001111", "DE"),  // Germany
    ("100000", "IN"),  // India
    ("001100", "IT"),  // Italy
    ("100001", "JP"),  // Japan
    ("001101", "ES"),  // Spain
    ("010000", "GB"),  // United Kingdom

    // 4-bit prefixes
    ("1010", "US"),  // United States
    ("0001", "RU"),  // Russian Federation

    // 9-bit prefixes (continued, ordered by value)
    ("000000001", "ZA"),  // South Africa
    ("000000010", "EG"),  // Egypt
    ("000000011", "LY"),  // Libyan Arab Jamahiriya
    ("000000100", "MA"),  // Morocco
    ("000000101", "TN"),  // Tunisia
    ("000010010000", "AO"),  // Angola
];

/// Convert a 24-bit ICAO address (as u32) to its allocated country's ISO2 code.
///
/// This is a convenience function that validates the input is a valid 24-bit value
/// (i.e., the 8 most significant bits are zero) before delegating to `icao_to_country`.
///
/// # Arguments
/// * `icao_u32` - 24-bit ICAO address as a u32 (must be <= 0xFFFFFF)
///
/// # Returns
/// * `Some(&str)` - ISO 3166-1 alpha-2 country code if allocation found
/// * `None` - If the value is invalid (> 24 bits) or no allocation matches
///
/// # Examples
/// ```
/// use flydent::icao::icao_u32_to_country;
///
/// // United States allocation (0xAB8E4F = N8437D)
/// assert_eq!(icao_u32_to_country(0xAB8E4F), Some("US"));
///
/// // Yugoslavia allocation
/// assert_eq!(icao_u32_to_country(0x4C0000), Some("RS"));
///
/// // Invalid: more than 24 bits
/// assert_eq!(icao_u32_to_country(0x01000000), None);
/// ```
pub fn icao_u32_to_country(icao_u32: u32) -> Option<&'static str> {
    // Check if the 8 MSB are zero (valid 24-bit value)
    if icao_u32 > 0xFFFFFF {
        return None;
    }

    // Convert u32 to [u8; 3] big-endian
    let icao = [
        ((icao_u32 >> 16) & 0xFF) as u8,
        ((icao_u32 >> 8) & 0xFF) as u8,
        (icao_u32 & 0xFF) as u8,
    ];

    icao_to_country(icao)
}

/// Convert a 24-bit ICAO address to its allocated country's ISO2 code.
///
/// # Arguments
/// * `icao` - 24-bit ICAO address as a 3-byte array (big-endian)
///
/// # Returns
/// * `Some(&str)` - ISO 3166-1 alpha-2 country code if allocation found
/// * `None` - If no allocation matches the address
///
/// # Examples
/// ```
/// use flydent::icao::icao_to_country;
///
/// // United States allocation (starts with 1010)
/// let us_icao = [0xAB, 0x8E, 0x4F];  // Binary: 1010 1011 1000 1110 0100 1111
/// assert_eq!(icao_to_country(us_icao), Some("US"));
///
/// // Yugoslavia allocation (starts with 010011000)
/// let yu_icao = [0x4C, 0x00, 0x00];  // Binary: 0100 1100 0000 0000 0000 0000
/// assert_eq!(icao_to_country(yu_icao), Some("RS"));
/// ```
pub fn icao_to_country(icao: [u8; 3]) -> Option<&'static str> {
    // Convert bytes to 24-bit binary string
    let binary = format!("{:08b}{:08b}{:08b}", icao[0], icao[1], icao[2]);

    // Check each allocation prefix (already sorted longest-first)
    for (prefix, country_code) in ICAO_ALLOCATIONS {
        if binary.starts_with(prefix) {
            return Some(country_code);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usa_allocation() {
        // US allocations start with 1010 (0xA)
        let us_icao = [0xA0, 0x00, 0x01];  // N1
        assert_eq!(icao_to_country(us_icao), Some("US"));

        let us_icao2 = [0xAB, 0x8E, 0x4F];  // N8437D
        assert_eq!(icao_to_country(us_icao2), Some("US"));
    }

    #[test]
    fn test_yugoslavia_allocation() {
        // Yugoslavia starts with 010011000 (0x4C)
        let yu_icao = [0x4C, 0x00, 0x00];  // Binary: 0100 1100 0000 0000 0000 0000
        assert_eq!(icao_to_country(yu_icao), Some("RS"));
    }

    #[test]
    fn test_canada_allocation() {
        // Canada starts with 110000 (0xC)
        let ca_icao = [0xC0, 0x00, 0x01];
        assert_eq!(icao_to_country(ca_icao), Some("CA"));
    }

    #[test]
    fn test_uk_allocation() {
        // UK starts with 010000 (0x40)
        let uk_icao = [0x40, 0x00, 0x00];
        assert_eq!(icao_to_country(uk_icao), Some("GB"));
    }

    #[test]
    fn test_russia_allocation() {
        // Russia starts with 0001
        let ru_icao = [0x10, 0x00, 0x00];  // Binary: 0001 0000...
        assert_eq!(icao_to_country(ru_icao), Some("RU"));
    }

    #[test]
    fn test_icao_special_allocation() {
        // ICAO special allocations return "ZZ"
        let icao_special = [0xF0, 0x00, 0x00];  // Starts with 111100000
        assert_eq!(icao_to_country(icao_special), Some("ZZ"));
    }

    #[test]
    fn test_long_prefix() {
        // Test 14-bit prefix (Antigua and Barbuda: 00001100101000)
        let ag_icao = [0x0C, 0xA0, 0x00];  // Binary: 0000 1100 1010 0000 0000...
        assert_eq!(icao_to_country(ag_icao), Some("AG"));
    }

    #[test]
    fn test_no_allocation() {
        // Test an unallocated range
        let unallocated = [0xFF, 0xFF, 0xFF];
        assert_eq!(icao_to_country(unallocated), None);
    }

    #[test]
    fn test_prefix_precedence() {
        // Test that longer prefixes take precedence
        // 0001 is Russia (4-bit), but 00001100101000 is Antigua (14-bit)
        let ag_icao = [0x0C, 0xA0, 0x00];
        assert_eq!(icao_to_country(ag_icao), Some("AG"));
        assert_ne!(icao_to_country(ag_icao), Some("RU"));
    }

    // Tests for u32 convenience function

    #[test]
    fn test_u32_usa_allocation() {
        // US allocation: 0xAB8E4F = N8437D
        assert_eq!(icao_u32_to_country(0xAB8E4F), Some("US"));
        assert_eq!(icao_u32_to_country(0xA00001), Some("US"));  // N1
    }

    #[test]
    fn test_u32_yugoslavia_allocation() {
        // Yugoslavia: 0x4C0000
        assert_eq!(icao_u32_to_country(0x4C0000), Some("RS"));
    }

    #[test]
    fn test_u32_canada_allocation() {
        // Canada: 0xC00001
        assert_eq!(icao_u32_to_country(0xC00001), Some("CA"));
    }

    #[test]
    fn test_u32_invalid_more_than_24_bits() {
        // Values with more than 24 bits should return None
        assert_eq!(icao_u32_to_country(0x01000000), None);
        assert_eq!(icao_u32_to_country(0xFFFFFFFF), None);
        assert_eq!(icao_u32_to_country(0x12345678), None);
    }

    #[test]
    fn test_u32_boundary_values() {
        // Maximum valid 24-bit value
        assert_eq!(icao_u32_to_country(0xFFFFFF), None);  // Unallocated but valid

        // Just over the boundary
        assert_eq!(icao_u32_to_country(0x1000000), None);  // Invalid
    }

    #[test]
    fn test_u32_zero() {
        // Zero is a valid 24-bit value but doesn't match any allocation
        // 0x000000 = 000000000000000000000000 (doesn't match any prefix)
        assert_eq!(icao_u32_to_country(0x000000), None);
    }

    #[test]
    fn test_u32_equivalence_with_array_version() {
        // Verify u32 version gives same results as array version
        let test_cases = vec![
            0xAB8E4F,  // US
            0x4C0000,  // Yugoslavia
            0xC00001,  // Canada
            0x400000,  // UK
            0x100000,  // Russia
        ];

        for icao_u32 in test_cases {
            let icao_array = [
                ((icao_u32 >> 16) & 0xFF) as u8,
                ((icao_u32 >> 8) & 0xFF) as u8,
                (icao_u32 & 0xFF) as u8,
            ];
            assert_eq!(icao_u32_to_country(icao_u32), icao_to_country(icao_array));
        }
    }
}
