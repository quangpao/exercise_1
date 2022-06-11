use std::fs::File;
use std::io::Read;

fn main() {
    let org_arr = [1, 2, 3, 5, 6, 8, 10, 15];
    let sub_arr = [1, 2, 3, 5, 6, 8];

    find_sub_array(&org_arr, &sub_arr);
    let str = "This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.";
    println!(" Count {}", count_sub_string( str, "is is"));
    println!("test");
}

fn find_sub_array(arr: &[i32], sub_arr: &[i32]) {
    let mut check = true;

    for i in sub_arr.iter() {
        if !arr.contains(&i) {
            check = false;
        }
    }

    if check {
        println!("Sub array found");
    } else {
        println!("Sub array not found");
    }
}


fn count_sub_string(str: &str, sub_str: &str) -> usize {
    let mut count = 0;
    let mut i = 0;
    let mut temp_str = str.to_string();

    // println!("{} {}", count, i);
    while i < temp_str.len() {
        i = temp_str.find(sub_str).unwrap() + 1;
        // println!("{} {}", count, i);
        if i >= 0 {
            count += 1;
            temp_str = temp_str[i..].to_string();
            //println!("{}", temp_str);
        } else {
            break;
        }
    }

    count
}

fn read_file(file_name: &str) -> String {
    let mut file = File::open(file_name).expect("File not found");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Something went wrong reading the file");

    contents
}