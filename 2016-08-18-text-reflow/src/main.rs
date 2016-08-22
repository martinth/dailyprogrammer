struct Reflowed<'a> {
    text: &'a mut Iterator<Item=&'a str>,
    line_width: usize,
    buffer: String
}
impl<'a> Reflowed<'a> {
    fn reset(&mut self) -> String {
        let result = self.buffer.clone();
        self.buffer = String::with_capacity(self.line_width);
        result
    }
}
impl<'a> Iterator for Reflowed<'a> {
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
                },
                None        => {
                    if self.buffer.len() > 0 {
                        return Some(self.reset())
                    } else {
                        return None
                    }

                }
            }
        }

    }
}

fn reflow<'a>(input: &'a mut Iterator<Item=&'a str>, line_width: usize) -> Reflowed<'a> {
    return Reflowed {
        text: input,
        line_width: line_width,
        buffer: String::with_capacity(line_width)
    }
}


fn main() {
    let line_width = 40;
    let input: &str = "In the beginning God created the heavens and the earth. Now the earth was
formless and empty, darkness was over the surface of the deep, and the Spirit of
God was hovering over the waters.

And God said, 'Let there be light', and there was light. God saw that the light
was good, and he separated the light from the darkness. God called the light
'day', and the darkness he called 'night'. And there was evening, and there was
morning - the first day.";


    for line in reflow(& mut input.split_whitespace(), line_width) {
        println!("{}", line);
    }

}