// rust
use std::collections::HashMap;


pub fn exercise_1_result (list: &Vec<i32>) {
    median(list);
    mode(list);
}

fn median (list: &Vec<i32>) {
    if list.len() == 0 { 
        println!("List is empty");
        return;
    }

    let mut list_cloned = list.clone();
    let mut median: f32 = 0.0;

    list_cloned.sort();

    match list.len() % 2 {
        0 => {
            let n = list.len() / 2;

            median = ((list_cloned[n - 1] + list_cloned[n]) / 2) as f32;
        },

        _ => {
            let n = (list.len() / 2) as f32;
            let idx = (n.floor()) as usize;

            median = list_cloned[idx] as f32;
        }
    }

    println!("Median: {}", median);
}

fn mode (list: &Vec<i32>) {
    if list.len() == 0 { 
        println!("List is empty");
        return;
    }

    let mut mode: i32 = 0;

    let mut list_cloned = list.clone();
    
    let mut hash_table = HashMap::new();
    
    for element in list_cloned {
        let count = hash_table.entry(element.to_string())
                                        .or_insert(0);
        *count += 1;
    }

    let mode_max = hash_table.iter()
                                                      .max_by(|a, b| a.1.cmp(&b.1));

    match mode_max {
        Some(mode_max) => mode = mode_max.0.parse::<i32>()
                                                            .unwrap_or(0),
        None => mode = 0
    }

    println!("Mode: {}", mode);
}