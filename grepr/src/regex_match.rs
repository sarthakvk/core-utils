use regex::Regex;

pub fn match_regex_pat(pat: &Regex, hay: &str) -> Vec<usize> {

    pat.find_iter(hay).map(|m: regex::Match<'_>| m.start()).collect()
}