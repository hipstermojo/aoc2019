use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").expect("Input file doesn't exist");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Unable to read content");
    let data: Vec<i32> = content
        .trim_end()
        .split(",")
        .filter_map(|token| token.parse::<i32>().ok())
        .collect();
    let mut parser = Parser { data, input: 5 };
    parser.parse();
}

struct Parser {
    data: Vec<i32>,
    input: i32,
}

impl Parser {
    fn parse(&mut self) {
        let mut cursor = 0;
        loop {
            let operation = format!("{:0>5}", self.data[cursor].to_string());
            let opcode = (&operation[3..]).parse::<usize>().unwrap();
            let modes = &operation[..3]
                .chars()
                .map(|x| x.to_digit(10).unwrap() as usize)
                .rev()
                .collect::<Vec<usize>>();
            match opcode {
                1 | 2 => {
                    let op_addr = vec![
                        if modes[0] == 0 {
                            self.data[cursor + 1] as usize
                        } else {
                            cursor + 1
                        },
                        if modes[1] == 0 {
                            self.data[cursor + 2] as usize
                        } else {
                            cursor + 2
                        },
                        self.data[cursor + 3] as usize,
                    ];
                    self.data[op_addr[2]] = if opcode == 1 {
                        self.data[op_addr[1]] + self.data[op_addr[0]]
                    } else {
                        self.data[op_addr[1]] * self.data[op_addr[0]]
                    };
                    cursor += op_addr.len() + 1;
                }
                3 => {
                    let dest_addr = self.data[cursor + 1] as usize;
                    self.data[dest_addr] = self.input;
                    cursor += 2;
                }
                4 => {
                    let read_addr = if modes[0] == 0 {
                        self.data[cursor + 1] as usize
                    } else {
                        cursor + 1
                    };
                    println!("{}", self.data[read_addr]);
                    cursor += 2;
                }
                5 | 6 => {
                    let op_addr = vec![
                        if modes[0] == 0 {
                            self.data[cursor + 1] as usize
                        } else {
                            cursor + 1
                        },
                        if modes[1] == 0 {
                            self.data[cursor + 2] as usize
                        } else {
                            cursor + 2
                        },
                    ];
                    if opcode == 5 {
                        cursor = if self.data[op_addr[0]] != 0 {
                            self.data[op_addr[1]] as usize
                        } else {
                            cursor + op_addr.len() + 1
                        };
                    } else {
                        cursor = if self.data[op_addr[0]] != 0 {
                            cursor + op_addr.len() + 1
                        } else {
                            self.data[op_addr[1]] as usize
                        };
                    }
                }
                7 | 8 => {
                    let op_addr = vec![
                        if modes[0] == 0 {
                            self.data[cursor + 1] as usize
                        } else {
                            cursor + 1
                        },
                        if modes[1] == 0 {
                            self.data[cursor + 2] as usize
                        } else {
                            cursor + 2
                        },
                        self.data[cursor + 3] as usize,
                    ];
                    if opcode == 7 {
                        self.data[op_addr[2]] = if self.data[op_addr[0]] < self.data[op_addr[1]] {
                            1
                        } else {
                            0
                        };
                    } else {
                        self.data[op_addr[2]] = if self.data[op_addr[0]] == self.data[op_addr[1]] {
                            1
                        } else {
                            0
                        };
                    }
                    cursor += op_addr.len() + 1;
                }
                99 => break,
                _ => unreachable!(),
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
            input: 0,
        };
        parser.parse();
        assert_eq!(vec![2, 0, 0, 0, 99], parser.data);
    }
    #[test]
    fn mul_opcode() {
        let mut parser = Parser {
            data: vec![2, 3, 0, 3, 99],
            input: 0,
        };
        parser.parse();
        assert_eq!(vec![2, 3, 0, 6, 99], parser.data);
    }
    #[test]
    fn halt_opcode() {
        let mut parser = Parser {
            data: vec![99, 0, 2, 3, 4, 698],
            input: 0,
        };
        parser.parse();
        assert_eq!(vec![99, 0, 2, 3, 4, 698], parser.data);
    }
    #[test]
    fn write_opcode() {
        let mut parser = Parser {
            data: vec![3, 1, 99],
            input: 254,
        };
        parser.parse();
        assert_eq!(vec![3, 254, 99], parser.data);
    }
    #[test]
    fn read_opcode() {
        let mut parser = Parser {
            data: vec![4, 0, 99],
            input: 0,
        };
        parser.parse();
        assert_eq!(vec![4, 0, 99], parser.data);
    }
    #[test]
    fn read_write_opcode() {
        let mut parser = Parser {
            data: vec![3, 0, 4, 0, 99],
            input: 2,
        };
        parser.parse();
        assert_eq!(vec![2, 0, 4, 0, 99], parser.data);
    }
    #[test]
    fn immediate_mode() {
        let mut parser = Parser {
            data: vec![1101, 100, -1, 4, 0],
            input: 0,
        };
        parser.parse();
        assert_eq!(vec![1101, 100, -1, 4, 99], parser.data);
    }
    #[test]
    fn jump_if_true_opcode() {
        let mut parser = Parser {
            data: vec![5, 2, 3, 4, 99],
            input: 0,
        };
        parser.parse();
        assert_eq!(vec![5, 2, 3, 4, 99], parser.data);
        parser.data = vec![5, 2, 4, 2, 7, 10, 3, 99];
        parser.parse();
        assert_eq!(vec![5, 2, 4, 2, 7, 10, 3, 99], parser.data);
        parser.data = vec![1105, 1, 6, 1002, 3, 1, 99, 12];
        parser.parse();
        assert_eq!(vec![1105, 1, 6, 1002, 3, 1, 99, 12], parser.data);
    }
    #[test]
    fn jump_if_false_opcode() {
        let mut parser = Parser {
            data: vec![6, 2, 1, 1101, 2, 3, 2, 99],
            input: 0,
        };
        parser.parse();
        assert_eq!(vec![6, 2, 5, 1101, 2, 3, 2, 99], parser.data);
        parser.data = vec![6, 3, 5, 0, 1102, 8, 20, 4, 99];
        parser.parse();
        assert_eq!(vec![6, 3, 5, 0, 1102, 8, 20, 4, 99], parser.data);
        parser.data = vec![1106, 0, 5, 3, 1, 99];
        parser.parse();
        assert_eq!(vec![1106, 0, 5, 3, 1, 99], parser.data);
    }
    #[test]
    fn less_than_opcode() {
        let mut parser = Parser {
            data: vec![7, 0, 3, 1, 99],
            input: 0,
        };
        parser.parse();
        assert_eq!(vec![7, 0, 3, 1, 99], parser.data);
        parser.data = vec![7, 0, 4, 1, 99];
        parser.parse();
        assert_eq!(vec![7, 1, 4, 1, 99], parser.data);
    }
    #[test]
    fn equals_opcode() {
        let mut parser = Parser {
            data: vec![1108, 2, 3, 1, 99],
            input: 0,
        };
        parser.parse();
        assert_eq!(vec![1108, 0, 3, 1, 99], parser.data);
        parser.data = vec![8, 1, 3, 1, 99];
        parser.parse();
        assert_eq!(vec![8, 1, 3, 1, 99], parser.data);
    }
}
