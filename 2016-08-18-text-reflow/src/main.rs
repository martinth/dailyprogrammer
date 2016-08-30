
struct ReflowIter<'a> {
    text: &'a mut Iterator<Item = &'a str>,
    line_width: usize,
    buffer: String,
}
impl<'a> ReflowIter<'a> {
    fn reset(&mut self) -> String {
        let result = self.buffer.clone();
        self.buffer = String::with_capacity(self.line_width);
        result
    }
}
impl<'a> Iterator for ReflowIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        loop {
            match self.text.next() {
                Some(token) => {
                    if self.buffer.len() + token.len() < self.line_width {
                        if self.buffer.len() > 0 {
                            self.buffer.push_str(" ");
                        }
                        self.buffer.push_str(token);
                    } else {
                        let result = self.reset();
                        self.buffer.push_str(token);
                        return Some(result);
                    }
                }
                None => {
                    if self.buffer.len() > 0 {
                        return Some(self.reset());
                    } else {
                        return None;
                    }
                }
            }
        }

    }
}

///
/// Take an iterator over words and turn it into an iterator over
/// lines. The lines will have at most `line_width` characters.
///
fn reflow<'a>(input: &'a mut Iterator<Item = &'a str>, line_width: usize) -> ReflowIter<'a> {
    return ReflowIter {
        text: input,
        line_width: line_width,
        buffer: String::with_capacity(line_width),
    };
}

///
/// Calculate the size of the fillers to fit a line of length `length`
/// into exactly `total` characters if you have `gaps` gaps to fill up.
///
fn calculate_fillers(total: u32, length: u32, gaps: u32) -> Vec<u32> {
    let delta = total - length;
    let per_gap = delta / gaps;
    let remaining = delta - (gaps * per_gap);
    let mut fillers = Vec::with_capacity(gaps as usize);

    for _ in 0..remaining {
        fillers.push(per_gap + 2);
    }
    for _ in 0..(gaps - remaining) {
        fillers.push(per_gap + 1);
    }

    return fillers;
}

///
/// Take a string (containing words a spaces) and make
/// it `line_width` wide by expanding the spaces to fit.
///
fn justify(line: String, line_width: u32) -> String {

    let spaces_to_insert = line_width - line.len() as u32;
    if spaces_to_insert > 0 {
        let mut result = String::with_capacity(line_width as usize);
        let words: Vec<&str> = line.split_whitespace().collect();
        let gaps = words.len() - 1;

        let calculate_fillerss =
            calculate_fillers(line_width as u32, line.len() as u32, gaps as u32);

        for (idx, word) in words.iter().enumerate() {
            result.push_str(word);
            if let Some(calculate_fillers_length) = calculate_fillerss.get(idx) {
                result.push_str((0..*calculate_fillers_length)
                    .map(|_| " ")
                    .collect::<String>()
                    .as_str());
            }
        }
        return result;
    } else {
        return line;
    }

}



fn main() {
    let line_width = 40;
    let input: &str = "In the beginning God created the heavens and the earth. Now the earth was
    \
                       formless and empty, darkness was over the surface of the deep, and the \
                       Spirit of
    God was hovering over the waters.

    And God said, 'Let \
                       there be light', and there was light. God saw that the light
    was good, \
                       and he separated the light from the darkness. God called the light
    \
                       'day', and the darkness he called 'night'. And there was evening, and \
                       there was
    morning - the first day.";

    let iter = &mut input.split_whitespace();

    for line in reflow(iter, line_width).map(|line| justify(line, line_width as u32)) {
        println!("{}", line);
    }
}
