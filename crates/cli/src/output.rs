pub fn print_answer(answer: &str, format: bool) {
    if format {
        let skin = termimad::MadSkin::default();
        skin.print_text(answer);
    } else {
        println!("{answer}");
    }
}
