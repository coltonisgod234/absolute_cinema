use crate::subtitles::{Seconds, Subtitle, Subtitles};

fn parse_time_vtt(s: &str) -> Seconds {
    let parts: Vec<&str> = s.split(&[':', '.'][..]).collect();
    let h: u64 = parts[0].parse().unwrap();
    let m: u64 = parts[1].parse().unwrap();
    let s: u64 = parts[2].parse().unwrap();
    let ms: u64 = parts[3].parse().unwrap();

    return (h * 3600 + m * 60 + s + ms / 1000) as Seconds;
}

pub fn parse_vtt(content: &str) -> Subtitles {
    let mut subs = Vec::new();
    let mut lines = content.lines().peekable();

    // skip header
    while let Some(&line) = lines.peek() {
        if line.trim().is_empty() {
            lines.next();
            break;
        }

        if line.contains("-->") {
            eprintln!("end header");
            break;
        }

        lines.next();
    }

    while lines.peek().is_some() {
        // skip index if present (VTT sometimes has it, sometimes not)
        if let Some(&line) = lines.peek() {
            if line.trim().parse::<usize>().is_ok() {
                lines.next();
            }
        }

        // parse times
        if let Some(times) = lines.next() {
            if let Some((start, end)) = times.split_once(" --> ") {
                let start = parse_time_vtt(start);
                let end = parse_time_vtt(end);

                // collect text
                let mut text_lines = Vec::new();
                while let Some(&line) = lines.peek() {
                    if line.trim().is_empty() {
                        break;
                    }
                    text_lines.push(lines.next().unwrap());
                }
                let text = text_lines.join("\n");

                subs.push(Subtitle { start, end, text });
            }
        }

        // skip empty line
        let _ = lines.next();
    }

    return Subtitles { inner: subs }
}