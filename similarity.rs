use std::collections::HashMap;
use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    io::stdin().read_line(&mut input1).expect("Fail to read");
    io::stdin().read_line(&mut input2).expect("Fail to read");

    let text_concat = format!("{} {}", input1.trim(), input2.trim());

    let bag_of_word = make_bag_of_word(&text_concat);
    println!("{:?}", bag_of_word);

    let word_vector1 = make_word_vector(&bag_of_word, &input1.trim());
    let word_vector2 = make_word_vector(&bag_of_word, &input2.trim());

    println!("{:?}", word_vector1);
    println!("{:?}", word_vector2);

    println!("Similarity: {}", cos_sim(&word_vector1, &word_vector2));
}

fn make_bag_of_word(text: &String) -> HashMap<String, isize> {
    let mut map = HashMap::new();

    let mut idx = 0;

    for word in text.split_whitespace() {
        map.entry(String::from(word)).or_insert(idx);
        idx += 1;
    }

    map
}

fn make_word_vector(map: &HashMap<String, isize>, text: &str) -> Vec<isize> {
    let mut v = Vec::new();

    let mut text_map = HashMap::new();

    for word in text.split_whitespace() {
        let count = text_map.entry(word).or_insert(0);
        *count += 1;
    }

    let mut exist = false;
    for (key, _value) in map {
        exist = false;
        for (key2, value2) in &text_map {
            if key == key2 {
                exist = true;
                v.push(*value2);
                break;
            }
        }
        if exist == false {
            v.push(0);
        }
    }

    v
}

fn norm(v: &Vec<isize>) -> f64 {
    let mut result: f64 = 0.;

    for i in v {
        let j = *i as f64;
        result += j*j;
    }

    result.sqrt()
}

fn dot(v1: &Vec<isize>, v2: &Vec<isize>) -> f64 {
    if v1.len() != v2.len() {
        panic!("vectors length must be same");
    } 

    let mut result = 0;
    for (i, j) in v1.into_iter().zip(v2.into_iter()) {
        result += i*j;
    }

    result as f64
}

fn cos_sim(v1: &Vec<isize>, v2: &Vec<isize>) -> f64 {
    dot(v1, v2) / (norm(v1)*norm(v2))
}