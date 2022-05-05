mod solver;

fn main() {
    sample();
}

fn parse_comma_delimited_string(s: &str) -> solver::InitialState {
    let values : Vec<&str> = s.split(&[',', '\n'][..]).collect();

    let mut known_values = Vec::new();
    for (i, value) in values.iter().enumerate() {
        if value.is_empty() {
            continue;
        }

        let value = value.parse::<u8>().unwrap();
        let row = i / 9;
        let col = i % 9;
        known_values.push((row, col, value));
    }

    solver::InitialState::new(known_values)
}

fn sample() {
    let initial_state = parse_comma_delimited_string(
        ",,,,,,,,\n\
        ,,,,,,,,\n\
        ,,,,,,,,\n\
        ,,,,,,,,\n\
        ,,,,,,,,\n\
        ,,,,,,,,\n\
        ,,,,,,,,\n\
        ,,,,,,,,\n\
        1,,2,,,,,,");

    let solution = solver::solve(initial_state).expect("No solution found!");
    println!("{solution}");
}

#[cfg(test)]
mod tests {
    use crate::solver;

    #[test]
    fn test_easy() {
        let initial_state = crate::parse_comma_delimited_string(
            "1,,3,4,8,,2,7,6\n\
            2,,5,,,,9,,\n\
            6,,4,9,1,2,,5,8\n\
            ,,,,9,8,4,,7\n\
            ,2,,3,,,,,\n\
            4,,,,,,,8,3\n\
            ,4,,,2,3,,6,\n\
            9,6,,8,5,,,,\n\
            8,,,,,,7,,");

        let solution = solver::solve(initial_state).expect("No solution found!");
        assert!(solution.is_complete());
    }

    #[test]
    fn test_hard() {
        let initial_state = crate::parse_comma_delimited_string(
            ",,4,,6,,,,2\n\
            3,,,5,,,,,7\n\
            ,,,,,,,,\n\
            1,,,,8,,,,\n\
            ,3,,,4,,7,,8\n\
            5,,,,7,,,,6\n\
            ,,,,,,1,8,\n\
            2,,,9,,,,,3\n\
            ,1,,6,,,,2,");
        
        let solution = solver::solve(initial_state).expect("No solution found!");
        assert!(solution.is_complete());
    }

    #[test]
    fn test_evil() {
        let initial_state = crate::parse_comma_delimited_string(
            ",,8,,,,,,\n\
            ,,,,,7,,3,\n\
            6,1,,5,,,2,,\n\
            4,,,,,,,,\n\
            ,,,,5,,,,6\n\
            2,9,,6,,,1,,\n\
            9,8,,,3,,,2,\n\
            ,,4,9,,,,,\n\
            ,,5,,,,8,,");
        
        let solution = solver::solve(initial_state).expect("No solution found!");
        assert!(solution.is_complete());
    }

    #[test]
    fn test_dutch_miracle() {
        let initial_state = crate::parse_comma_delimited_string(
            ",,,,,,,,\n\
            ,,,,,,,,\n\
            ,,,,,,,,\n\
            ,,,,,,,,\n\
            ,,,,,,,,\n\
            ,,,,,,,,\n\
            ,,,,,,,,\n\
            ,,,,,,,,\n\
            1,,2,,,,,,");
        
        let solution = solver::solve(initial_state).expect("No solution found!");
        assert!(solution.is_complete());
    }
}