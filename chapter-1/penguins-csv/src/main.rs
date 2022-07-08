// Executable projects require a main function.
fn main() {
    // The "\ escapes the newline character.
    // This variable holds a CSV string.
    let penguin_data = "\
        common name, length (cm)
        Little penguin, 33
        Yellow-eyed penguin, 65
        Fiorland penguin, 60
        invalid, data
    ";
    
    let records = penguin_data.lines();

    // Loops over every newline in the penguin_data string.
    for (i, record) in records.enumerate() {
        // This skips the header row.
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        // Starts with a line of the CSV.
        let fields: Vec<_> = record
            // Splits the line on commas.
            .split(',')
            // Trims whitespace is each field. This is a lambda/anonymous function.
            .map(|field| field.trim())
            // Builds a collection of fields.
            .collect();

        // Checks configuration at compile-time.
        if cfg!(debug_assertions) {
            // eprintln prints to stderr.
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        // Attempts to parse field as a float.
        if let Ok(length) = fields[1].parse::<f32>() {
            // println prints to stdout.
            println!("{} is {} cm long", name, length);
        }
    }
}
