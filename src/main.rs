use music_theory::create_middle_c_note;

fn main() {
    let note = create_middle_c_note();

    println!("note key: {}", note.key());
}
