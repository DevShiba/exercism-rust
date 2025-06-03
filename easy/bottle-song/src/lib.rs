fn get_word_forms(num: u32) -> (&'static str, &'static str) { 
    match num {
        10 => ("Ten", "ten"),
        9 => ("Nine", "nine"),
        8 => ("Eight", "eight"),
        7 => ("Seven", "seven"),
        6 => ("Six", "six"),
        5 => ("Five", "five"),
        4 => ("Four", "four"),
        3 => ("Three", "three"),
        2 => ("Two", "two"),
        1 => ("One", "one"),
        0 => ("No", "no"),
        _  => panic!("Number {} is out of the expected range (0-10) for this song.", num),
    }
}

fn get_bottle_plural_form(num: u32) -> &'static str { 
    if num == 1 {
       return "bottle"; 
    }
    "bottles"
}

pub fn recite(start_bottles: u32, take_down: u32) -> String { 
    let mut song_verses = Vec::new();

    for i in 0..take_down { 
        let current_bottles_count = start_bottles - i; 

        if current_bottles_count == 0 {
            break;
        }

        let bottles_after_fall_count = current_bottles_count - 1;

        let (current_bottles_word_capitalized, _) = get_word_forms(current_bottles_count);
        let current_bottle_form_str = get_bottle_plural_form(current_bottles_count);

        let(_, bottles_after_fall_word_lowercase) = get_word_forms(bottles_after_fall_count);
        let bottles_after_fall_form_str = get_bottle_plural_form(bottles_after_fall_count);

        let verse = format!(
            "{CBWC} green {CBF} hanging on the wall,\n\
             {CBWC} green {CBF} hanging on the wall,\n\
             And if one green bottle should accidentally fall,\n\
             There'll be {BAFWL} green {BAFF} hanging on the wall.",
            CBWC = current_bottles_word_capitalized, 
            CBF = current_bottle_form_str,         
            BAFWL = bottles_after_fall_word_lowercase,
            BAFF = bottles_after_fall_form_str
        );
        song_verses.push(verse); 
    }

    song_verses.join("\n\n")
}