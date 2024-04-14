use grepr_lib;

fn main() {
    let text = "नमस्ते";

    let s = grepr_lib::search::Search::new(text, "नमस्ते");
    let pm = s.patt_match_bf();

    println!("pattern matched: {}", pm);
}
