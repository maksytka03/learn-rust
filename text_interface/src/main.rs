use std::collections::HashMap;
use std::io::stdin;

fn main() {
    println!("Department Employee Management\n------------------------------\n");
    println!("To add employee into a department, enter 'Add *Name* to *Department*'.\nType 'list' to list all entries.\nPress 'q' to quit.\n");

    let mut data_table: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        if input.to_lowercase().trim() == "q" {
            println!("\n");
            break;
        } else if input.to_lowercase().trim() == "list" {
            println!("\n");
            for (key, value) in &data_table {
                println!("{key} --- {:?}", value);
            }
            println!("\n");
        }

        let words: Vec<&str> = input.split_whitespace().collect();

        if words.len() > 1 {
            let name = words.get(1).unwrap().trim().to_string();
            let department = words.get(3).unwrap().trim().to_string();

            add_employees(&mut data_table, &department, &name);
        }
    }
}

fn add_employees(data_table: &mut HashMap<String, Vec<String>>, department: &String, name: &String) {
    let mut names_list = vec![];
    if data_table.get(department).is_some() {
        names_list = data_table.get(department).unwrap().to_owned();
        names_list.push(name.to_owned());
    } else {
        names_list.push(name.to_owned());
    }
    data_table.insert(department.to_owned(), names_list);
    println!("Added\n");
}
