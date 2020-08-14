use music_theory::middle_c_note;

fn main() {
    let note = middle_c_note();

    println!("note key: {}", note.key());
}
