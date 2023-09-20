use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn main() {
    let mut person_databank: HashMap<String, Vec<String>> = HashMap::new();

    for line in io::stdin().lock().lines()
     {
        let l_input_string = line.expect("Failed to read line!");

        let mut l_string_parts = l_input_string.trim().split_whitespace();

        if let Some(fingerprint_string) = l_string_parts.next() 
        {
            let l_fingerprint = fingerprint_string.to_string();
            
            let mut l_person_name = String::new();

            if let Some(name_string) = l_string_parts.next() 
            {
                l_person_name = name_string.to_string();
            }
            
            //Insert Exception Handling
            if l_fingerprint.len() >= 512
            {
                continue;
            }
            else
            {
                let l_user_input = person_databank.entry(l_fingerprint.clone()).or_insert(vec![]);
                l_user_input.push(l_person_name.clone());
            }
        }
    }

    //A Vector, of Tuples, which the first element is the fingerprint, names associated with fingerprint
    let mut printing_fingerprint_groups: Vec<(String, Vec<String>)> = Vec::new();

    for (element_fingerprint, person_names) in &person_databank
    {
        if person_names.len() >= 2
        {
            printing_fingerprint_groups.push( (element_fingerprint.clone(), person_names.clone()) );
        }
    }

    let mut newline_counter =  0;

    for (_, names) in &printing_fingerprint_groups
    {
        for name_element in names
        {
            println!("{}", name_element);
        }
        if newline_counter != printing_fingerprint_groups.len()-1
        {
            println!();
            newline_counter += 1;
        }
    }
}