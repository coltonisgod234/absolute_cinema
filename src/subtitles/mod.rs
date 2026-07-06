pub mod vtt;

pub type Seconds = f64;

#[derive(Clone)]
pub struct Subtitle {
    pub start: Seconds,
    pub end: Seconds,
    pub text: String
}

#[derive(Clone)]
pub struct Subtitles {
    pub inner: Vec<Subtitle>,
}

impl Subtitles {
    pub fn to_iterator(&self) -> SubtitlesIter {
        return SubtitlesIter {
            subs: self.clone(),
            current_idx: 0
        }
    }
}

pub struct SubtitlesIter {
    subs: Subtitles,  // owned because borrows are HELL
    current_idx: usize,
}

impl SubtitlesIter {
    pub fn at(&mut self, t: Seconds) -> Option<&Subtitle> {
        // fast forward if necessary
        while self.current_idx < self.subs.inner.len() && t > self.subs.inner[self.current_idx].end {
            self.current_idx += 1;
        }

        // check if current subtitle contains t
        if self.current_idx < self.subs.inner.len() {
            let s = &self.subs.inner[self.current_idx];
            if t >= s.start && t <= s.end {
                return Some(s);
            }
        }

        return None
    }
}
