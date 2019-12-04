fn main() {
    let valid_count = (236491..713787).filter(is_valid).count();
    println!("There are {} valid passwords", valid_count);
    let valid_count = (236491..713787).filter(is_valid_with_pair).count();
    println!("There are {} valid passwords with pairs", valid_count);
}

fn is_valid_with_pair(password: &u32) -> bool {
    let mut result: Validator = Validator {
        is_repeating: false,
        is_never_decreasing: true,
    };
    let password = password.to_string();
    let numbers = password
        .split("")
        .filter(|c| c != &"")
        .collect::<Vec<&str>>();
    let mut prev = numbers[0];
    let mut repeat = 1;
    let mut is_pair = false;
    for i in 1..numbers.len() {
        if prev == numbers[i] {
            result.is_repeating = true;
            repeat += 1;
        } else {
            if !is_pair && repeat == 2 {
                is_pair = true;
            }
            repeat = 1;
        }
        if prev > numbers[i] {
            result.is_never_decreasing = false;
            break;
        }
        prev = numbers[i];
    }
    (is_pair || repeat == 2) && result.is_never_decreasing && result.is_repeating
}

fn is_valid(password: &u32) -> bool {
    let mut result: Validator = Validator {
        is_repeating: false,
        is_never_decreasing: true,
    };
    let password = password.to_string();
    let numbers = password
        .split("")
        .filter(|c| c != &"")
        .collect::<Vec<&str>>();
    let mut prev = numbers[0];
    for i in 1..numbers.len() {
        if prev == numbers[i] {
            result.is_repeating = true;
        }
        if prev > numbers[i] {
            result.is_never_decreasing = false;
            break;
        }
        prev = numbers[i];
    }
    result.is_never_decreasing && result.is_repeating
}

struct Validator {
    is_repeating: bool,
    is_never_decreasing: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_returns_false_for_decreasing() {
        assert!(!is_valid(&1002));
        assert!(!is_valid(&2004));
        assert!(!is_valid(&1220));
    }

    #[test]
    fn test_returns_true_for_non_decreasing() {
        assert!(is_valid(&1999));
        assert!(is_valid(&1124));
        assert!(is_valid(&344567789));
        assert!(is_valid(&111111));
    }

    #[test]
    fn test_returns_false_for_non_repeating() {
        assert!(!is_valid(&1234));
        assert!(!is_valid(&134678));
        assert!(!is_valid(&123789))
    }

    #[test]
    fn test_returns_true_for_repeating() {
        assert!(is_valid(&1124));
        assert!(is_valid(&344567789));
        assert!(is_valid(&111111));
    }

    #[test]
    fn test_part2_has_a_pair() {
        assert!(is_valid_with_pair(&112233));
        assert!(is_valid_with_pair(&111122));
        assert!(is_valid_with_pair(&123345));
        assert!(is_valid_with_pair(&113456));
        assert!(is_valid_with_pair(&566789));
    }

    #[test]
    fn test_part2_has_no_pair() {
        assert!(!is_valid_with_pair(&111111));
        assert!(!is_valid_with_pair(&123444));
        assert!(!is_valid_with_pair(&333358));
    }
}
