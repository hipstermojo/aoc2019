use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").expect("Input file does not exist");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Unable to read content");
    let orig_data: Vec<usize> = content
        .split(",")
        .filter_map(|token| token.parse::<usize>().ok())
        .collect();
    let mut parser = Parser {
        data: orig_data.clone(),
    };
    parser.data[1] = 12;
    parser.data[2] = 2;
    parser.parse();
    println!("Value at position 0 is {}", parser.data[0]);
    let (noun, verb) = get_noun_and_verb(&orig_data, 19690720);
    println!(
        "Target noun and verb are {} and {}\n100 * noun + verb = {}",
        noun,
        verb,
        100 * noun + verb
    );
}

fn get_noun_and_verb(data: &Vec<usize>, target: usize) -> (usize, usize) {
    let mut parser = Parser { data: data.clone() };
    let mut result: (usize, usize) = (0, 0);
    'running: for noun in 0..=99 {
        for verb in 0..=99 {
            parser.data[1] = noun;
            parser.data[2] = verb;
            parser.parse();
            if parser.data[0] == target {
                result.0 = noun;
                result.1 = verb;
                break 'running;
            }
            parser.data = data.clone();
        }
    }
    result
}

struct Parser {
    data: Vec<usize>,
}

impl Parser {
    fn parse(&mut self) {
        let mut cursor = 0;
        loop {
            let value = self.data[cursor];
            if value == 1 || value == 2 {
                let op_addr_a = self.data[cursor + 1];
                let op_addr_b = self.data[cursor + 2];
                let destination = self.data[cursor + 3];
                if value == 1 {
                    self.data[destination] = self.data[op_addr_a] + self.data[op_addr_b];
                    cursor += 4;
                } else {
                    self.data[destination] = self.data[op_addr_a] * self.data[op_addr_b];
                    cursor += 4;
                }
            } else if value == 99 {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_opcode() {
        let mut parser = Parser {
            data: vec![1, 0, 0, 0, 99],
        };
        parser.parse();
        assert_eq!(vec![2, 0, 0, 0, 99], parser.data);
    }
    #[test]
    fn mul_opcode() {
        let mut parser = Parser {
            data: vec![2, 3, 0, 3, 99],
        };
        parser.parse();
        assert_eq!(vec![2, 3, 0, 6, 99], parser.data);
    }
    #[test]
    fn halt_opcode() {
        let mut parser = Parser {
            data: vec![99, 0, 2, 3, 4, 698],
        };
        parser.parse();
        assert_eq!(vec![99, 0, 2, 3, 4, 698], parser.data);
    }
    #[test]
    fn parser_complete_prog() {
        let mut parser = Parser {
            data: vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
        };
        parser.parse();
        assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], parser.data);
        parser = Parser {
            data: vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
        };
        parser.parse();
        assert_eq!(
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
            parser.data
        );
        parser = Parser {
            data: vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
        };
        parser.parse();
        assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], parser.data);
        parser = Parser {
            data: vec![2, 4, 4, 5, 99, 0],
        };
        parser.parse();
        assert_eq!(vec![2, 4, 4, 5, 99, 9801], parser.data);
    }
    #[test]
    fn get_target_noun_and_verb() {
        let mut data = vec![
            1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 10, 1, 19, 1, 6, 19, 23, 2, 23, 6,
            27, 1, 5, 27, 31, 1, 31, 9, 35, 2, 10, 35, 39, 1, 5, 39, 43, 2, 43, 10, 47, 1, 47, 6,
            51, 2, 51, 6, 55, 2, 55, 13, 59, 2, 6, 59, 63, 1, 63, 5, 67, 1, 6, 67, 71, 2, 71, 9,
            75, 1, 6, 75, 79, 2, 13, 79, 83, 1, 9, 83, 87, 1, 87, 13, 91, 2, 91, 10, 95, 1, 6, 95,
            99, 1, 99, 13, 103, 1, 13, 103, 107, 2, 107, 10, 111, 1, 9, 111, 115, 1, 115, 10, 119,
            1, 5, 119, 123, 1, 6, 123, 127, 1, 10, 127, 131, 1, 2, 131, 135, 1, 135, 10, 0, 99, 2,
            14, 0, 0,
        ];
        let (noun, verb) = get_noun_and_verb(&data, 8017076);
        data[1] = noun;
        data[2] = verb;
        let mut parser = Parser { data };
        parser.parse();
        assert_eq!(8017076, parser.data[0]);
    }
}
