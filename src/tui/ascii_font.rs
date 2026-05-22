use std::collections::HashMap;

pub const FONTS: &[&str] = &[
    "digital", "block", "tiny", "minimal", "rounded", "shadow", "big", "slant",
];

pub fn render_time(value: &str, font: &str, max_width: u16) -> Vec<String> {
    if max_width < 42 || matches!(font, "tiny" | "minimal") {
        return vec![value.to_string()];
    }
    let glyphs = digital_glyphs();
    let mut lines = vec![
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
    ];
    for ch in value.chars() {
        let glyph = glyphs
            .get(&ch)
            .unwrap_or_else(|| glyphs.get(&' ').expect("space glyph exists"));
        for (index, line) in glyph.iter().enumerate() {
            lines[index].push_str(line);
            lines[index].push(' ');
        }
    }
    lines
}

fn digital_glyphs() -> HashMap<char, [&'static str; 5]> {
    HashMap::from([
        ('0', [" ███ ", "█   █", "█   █", "█   █", " ███ "]),
        ('1', ["  █  ", " ██  ", "  █  ", "  █  ", " ███ "]),
        ('2', ["████ ", "    █", " ███ ", "█    ", "█████"]),
        ('3', ["████ ", "    █", " ███ ", "    █", "████ "]),
        ('4', ["█  █ ", "█  █ ", "█████", "   █ ", "   █ "]),
        ('5', ["█████", "█    ", "████ ", "    █", "████ "]),
        ('6', [" ███ ", "█    ", "████ ", "█   █", " ███ "]),
        ('7', ["█████", "   █ ", "  █  ", " █   ", "█    "]),
        ('8', [" ███ ", "█   █", " ███ ", "█   █", " ███ "]),
        ('9', [" ███ ", "█   █", " ████", "    █", " ███ "]),
        (':', ["     ", "  █  ", "     ", "  █  ", "     "]),
        (' ', ["     ", "     ", "     ", "     ", "     "]),
    ])
}
