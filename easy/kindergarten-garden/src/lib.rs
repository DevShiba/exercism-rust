fn plant_name(code: char) -> &'static str {
    match code {
        'G' => "grass",
        'C' => "clover",
        'R' => "radishes",
        'V' => "violets",
        _ => panic!("Invalid plant code"),
    }
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let students = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet",
        "Ileana", "Joseph", "Kincaid", "Larry",
    ];

    let student_index = students.iter().position(|&s| s == student);

    if student_index.is_none(){
        return vec![];
    }

    let index = student_index.unwrap();

    let rows: Vec<&str> = diagram.lines().collect();
    if rows.len() != 2 {
        panic!("Diagram must have two rows.");
    }

    let row1_chars: Vec<char> = rows[0].chars().collect();
    let row2_chars: Vec<char> = rows[1].chars().collect();

    let start_char_index = index * 2;

    let plant_codes = [
        row1_chars[start_char_index],
        row1_chars[start_char_index + 1],
        row2_chars[start_char_index],
        row2_chars[start_char_index + 1],
    ];

    plant_codes.iter().map(|&code| plant_name(code)).collect()
}
