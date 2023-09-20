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

    let l_num_totalfingers = person_databank.len();

    let mut l_counter_blanklines = 0;
    for (_element_fingerprint, element_vector_ofPresons) in &person_databank
    {
        if element_vector_ofPresons.len() >= 2
        {
            for iter_personname in element_personname
            {
                println!("{}", iter_personname);
            }
        }
        if l_counter_blanklines < l_num_totalfingers - 1
        {
            l_counter_blanklines += 1;
            println!("");
        }
    }
}