use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];

    let content = read_file(&path);

    let lines = break_file(&content);

    let res_vec = sum_line_number(&lines);

    let final_result = sum_res_vec(&res_vec);

    println!("RESULT FINAL-----> {}", final_result);
}


fn read_file(path: &String) -> String {
    return fs::read_to_string(path)
    .expect("Should have been able to read the file");
}

fn break_file(content: &String) -> Vec<&str> {
    return content.split('\n').collect();
}

fn sum_line_number(lines: &Vec<&str>) -> Vec<i32> {
    let mut result_vec:Vec<i32> = Vec::new();
    
    for l in lines {    
        let mut char_vector:Vec<char> = Vec::new();

        for c in l.chars() {

            match c {
                '1' => char_vector.push('1'),
                '2' => char_vector.push('2'),
                '3' => char_vector.push('3'),
                '4' => char_vector.push('4'),
                '5' => char_vector.push('5'),
                '6' => char_vector.push('6'),
                '7' => char_vector.push('7'),
                '8' => char_vector.push('8'),
                '9' => char_vector.push('9'),
                _ => {
                   print!("");
                }
                
            };
        }
        let mut temp_string = String::new();
        if char_vector.len() > 1 {
            temp_string.push(char_vector[0]);
            temp_string.push(char_vector[char_vector.len()-1]);

           
        } else {
            temp_string.push(char_vector[0]);
            temp_string.push(char_vector[0]);
        }
        let number = temp_string.parse::<i32>().unwrap();
        result_vec.push(number);
    }

    return result_vec;

}

fn sum_res_vec(v: &Vec<i32>) -> i32{
    let mut result = 0;
    for item in v {
        println!("{}", item);
        result=result+item;
    }

    return result;
}