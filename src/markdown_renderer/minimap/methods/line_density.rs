//! Calculate line density for minimap rendering.
//!
//! Provides functions to analyze text content and compute density values
//! that can be converted to Braille representations.

use super::super::Minimap;

impl<'a> Minimap<'a> {
    /// Calculate the maximum line width in the content.
    ///
    /// Used for normalizing density calculations.
    pub fn max_line_width(&self) -> usize {
        self.content
            .lines()
            .map(|line| line.chars().filter(|c| !c.is_whitespace()).count())
            .max()
            .unwrap_or(0)
    }

    /// Get density values for all lines in the content.
    ///
    /// # Returns
    ///
    /// A vector of density values (0.0 to 1.0) for each line.
    pub fn line_densities(&self) -> Vec<f32> {
        let max_width = self.max_line_width().max(1);

        self.content
            .lines()
            .map(|line| {
                let char_count = line.chars().filter(|c| !c.is_whitespace()).count();
                (char_count as f32 / max_width as f32).min(1.0)
            })
            .collect()
    }

    /// Check if a minimap line index falls within the viewport.
    ///
    /// # Arguments
    ///
    /// * `minimap_line` - The line index in the minimap
    /// * `minimap_height` - Total height of the minimap in lines
    ///
    /// # Returns
    ///
    /// True if the minimap line represents content within the viewport.
    pub fn is_in_viewport(&self, minimap_line: usize, minimap_height: usize) -> bool {
        if self.total_lines == 0 || minimap_height == 0 {
            return false;
        }

        // Map minimap line to source line range
        let lines_per_minimap_line = (self.total_lines as f32 / minimap_height as f32).ceil();
        let source_start = (minimap_line as f32 * lines_per_minimap_line) as usize;
        let source_end = ((minimap_line + 1) as f32 * lines_per_minimap_line) as usize;

        // Check if this range overlaps with viewport
        source_start < self.viewport_end && source_end > self.viewport_start
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::Minimap;

    #[test]
    fn test_max_line_width() {
        let content = "short\na much longer line here\nmed";
        let minimap = Minimap::new(content);
        // "amuchlongerlinehere" = 19 characters (without spaces)
        assert_eq!(minimap.max_line_width(), 19);
    }

    #[test]
    fn test_line_densities() {
        let content = "hello\n\nworld";
        let minimap = Minimap::new(content);
        let densities = minimap.line_densities();

        assert_eq!(densities.len(), 3);
        assert_eq!(densities[0], 1.0); // "hello" is max
        assert_eq!(densities[1], 0.0); // empty line
        assert_eq!(densities[2], 1.0); // "world" same length
    }

    #[test]
    fn test_is_in_viewport() {
        let content = "line1\nline2\nline3\nline4\nline5";
        // viewport(1, 3, 5) means viewing lines 1-2 (viewport_end is exclusive)
        let minimap = Minimap::new(content).viewport(1, 3, 5);

        // With 5 lines and minimap height 5, each minimap line = 1 source line
        // Minimap line 0 = source line 0, not in viewport 1-3
        assert!(!minimap.is_in_viewport(0, 5));
        // Minimap line 1 = source line 1, in viewport 1-3
        assert!(minimap.is_in_viewport(1, 5));
        // Minimap line 2 = source line 2, in viewport 1-3
        assert!(minimap.is_in_viewport(2, 5));
        // Minimap line 3 = source line 3, not in viewport 1-3
        assert!(!minimap.is_in_viewport(3, 5));
        // Minimap line 4 = source line 4, not in viewport 1-3
        assert!(!minimap.is_in_viewport(4, 5));
    }
}
