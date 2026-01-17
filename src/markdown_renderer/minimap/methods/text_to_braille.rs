//! Convert text density to Braille characters.
//!
//! Braille characters are used because they can represent 8 dots in a 2x4 grid,
//! allowing us to show fine-grained density information in a compact space.

/// Braille character base (U+2800 - empty Braille pattern).
const BRAILLE_BASE: u32 = 0x2800;

/// Braille dot positions (bit values):
/// ```text
/// 1  8
/// 2  16
/// 4  32
/// 64 128
/// ```
const BRAILLE_DOTS: [u32; 8] = [0x01, 0x02, 0x04, 0x40, 0x08, 0x10, 0x20, 0x80];

/// Convert a density value (0.0 to 1.0) to a Braille character.
///
/// Higher density values result in more filled Braille dots.
///
/// # Arguments
///
/// * `density` - A value from 0.0 (empty) to 1.0 (full)
///
/// # Returns
///
/// A single Braille character representing the density.
pub fn density_to_braille(density: f32) -> char {
    let density = density.clamp(0.0, 1.0);
    let dots = (density * 8.0).round() as usize;

    let mut code = BRAILLE_BASE;
    for i in 0..dots.min(8) {
        code |= BRAILLE_DOTS[i];
    }

    char::from_u32(code).unwrap_or(' ')
}

/// Convert a pair of density values to a Braille character.
///
/// This allows representing two columns of density in one character.
///
/// # Arguments
///
/// * `left_density` - Density for the left column (0.0 to 1.0)
/// * `right_density` - Density for the right column (0.0 to 1.0)
///
/// # Returns
///
/// A single Braille character representing both densities.
pub fn density_pair_to_braille(left_density: f32, right_density: f32) -> char {
    let left = left_density.clamp(0.0, 1.0);
    let right = right_density.clamp(0.0, 1.0);

    let left_dots = (left * 4.0).round() as usize;
    let right_dots = (right * 4.0).round() as usize;

    let mut code = BRAILLE_BASE;

    // Left column dots (positions 0, 1, 2, 3 -> bits 1, 2, 4, 64)
    let left_bits = [0x01, 0x02, 0x04, 0x40];
    for i in 0..left_dots.min(4) {
        code |= left_bits[i];
    }

    // Right column dots (positions 4, 5, 6, 7 -> bits 8, 16, 32, 128)
    let right_bits = [0x08, 0x10, 0x20, 0x80];
    for i in 0..right_dots.min(4) {
        code |= right_bits[i];
    }

    char::from_u32(code).unwrap_or(' ')
}

/// Convert line content to a density value based on character count.
///
/// # Arguments
///
/// * `line` - The text line to analyze
/// * `max_width` - The maximum expected line width for normalization
///
/// # Returns
///
/// A density value from 0.0 (empty) to 1.0 (full width).
pub fn line_to_density(line: &str, max_width: usize) -> f32 {
    if max_width == 0 {
        return 0.0;
    }

    let char_count = line.chars().filter(|c| !c.is_whitespace()).count();
    (char_count as f32 / max_width as f32).min(1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_density_to_braille_empty() {
        let ch = density_to_braille(0.0);
        assert_eq!(ch, '\u{2800}'); // Empty Braille
    }

    #[test]
    fn test_density_to_braille_full() {
        let ch = density_to_braille(1.0);
        assert_eq!(ch, '\u{28FF}'); // Full Braille
    }

    #[test]
    fn test_line_to_density() {
        assert_eq!(line_to_density("", 80), 0.0);
        assert_eq!(line_to_density("hello", 10), 0.5);
        assert_eq!(line_to_density("  hello  ", 10), 0.5); // whitespace ignored
    }
}
