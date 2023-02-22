// rust
use std::collections::HashMap;
use std::io::stdin;

pub fn exercise_3() {
    let departments: HashMap<String, Vec<String>> = add_people();

    println!("############## Exercise 3 ##############");
    retrieve_persons_by_department(departments);
    println!("########################################");
}

fn add_people() -> HashMap<String, Vec<String>> {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Enter Person Name: ");
        let mut person_name = String::new();
        stdin()
            .read_line(&mut person_name)
            .expect("Failed to read input");

        println!("Enter Person Department: ");
        let mut person_department = String::new();
        stdin()
            .read_line(&mut person_department)
            .expect("Failed to read input");

        let department_people = departments
            .entry(person_department.trim().to_string())
            .or_insert(Vec::new());

        department_people.push(person_name.trim().to_string());
        department_people.sort();

        println!("Do you want to terminate the addiction? (y/n)");
        let mut option = String::new();
        stdin()
            .read_line(&mut option)
            .expect("Failed to read input");

        if !option.trim().is_empty() && option.trim() == "y" {
            return departments;
        }
    }
}

fn retrieve_persons_by_department(data: HashMap<String, Vec<String>>) {
    for element in data.iter() {
        println!("############## {:?} ##############", element.0);
        for person in element.1.iter() {
            println!("- {:?}", person);
        }
    }
}
