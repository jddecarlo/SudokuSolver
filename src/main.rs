mod solver;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::solver;

    #[test]
    fn test_solver() {
        let values : Vec<&str> = 
            "1,,3,4,8,,2,7,6\n\
            2,,5,,,,9,,\n\
            6,,4,9,1,2,,5,8\n\
            ,,,,9,8,4,,7\n\
            ,2,,3,,,,,\n\
            4,,,,,,,8,3\n\
            ,4,,,2,3,,6,\n\
            9,6,,8,5,,,,\n\
            8,,,,,,7,,"
                .split(&[',', '\n'][..])
                .collect();
        assert_eq!(values.len(), 81);

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

        let initial_state = solver::InitialState::new(known_values);
        match solver::solve(initial_state) {
            Some(board) => {
                println!("{board:?}");
            },
            None => {
                println!("No solution found");
                assert!(false);
            }
        }
    }
}