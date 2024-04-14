pub struct Search<'a>{
    text: &'a [u8],
    pattern: &'a [u8]
}

impl<'a> Search<'a> {
    pub fn new(text: &'a str, pattern: &'a str) -> Self {
        Search{
            text: text.as_bytes(),
            pattern: pattern.as_bytes(),
        }
    }

    // Simple Brute force pattern matching
    pub fn patt_match_bf(&self) -> bool{

        'outer: for i in 0..=self.text.len()-self.pattern.len(){
            for p in 0..self.pattern.len(){
                if self.pattern[p] != self.text[i+p] {
                    continue 'outer;
                }
            }
            return true;
        }

        return false;
    }
}