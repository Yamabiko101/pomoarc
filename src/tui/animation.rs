pub fn progress_bar(percent: f64, width: usize, frame: u64, animated: bool) -> String {
    let full = ((percent.clamp(0.0, 1.0) * width as f64).round() as usize).min(width);
    let pulse = if animated && frame % 4 < 2 {
        '▓'
    } else {
        '█'
    };
    let mut bar = String::with_capacity(width);
    for index in 0..width {
        bar.push(if index < full { pulse } else { '░' });
    }
    bar
}
