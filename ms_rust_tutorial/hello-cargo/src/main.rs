fn main() {
    // println!("Hello, world!");
    // println!("1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}", 1u32 + 2, 8i32 - 5, 15 * 3);
    // Specify the data type "char"
    let character_1: char = 'S';
    let character_2: char = 'f';
    
    // Compiler interprets a single item in quotations as the "char" data type
    let smiley_face = '😃';

    // Compiler interprets a series of items in quotations as a "str" data type and creates a "&str" reference
    let string_1 = "miley ";

    // Specify the data type "str" with the reference syntax "&str"
    let string_2: str = "ace";

    println!("{} is a {}{}{}{}.", smiley_face, character_1, string_1, character_2, string_2);
}

