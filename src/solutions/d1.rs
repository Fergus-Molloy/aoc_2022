use crate::Input;
use crate::solutions::Solution;

pub struct D1;

impl Solution for D1 {
    fn pt_1(inp: Input) -> u64 {
        inp.lines().map(|s| s.parse().map(|x: u64| (x/3)-2)).into_iter().collect::<Result<Vec<u64>,_>>().unwrap().iter().sum()
    }
    fn pt_2(inp: crate::input::Input) -> u64 {
        let calc_fuel = |x: u64| {
            let mut fuel = (x/3-2) as i64;
            let mut acc= fuel;
            while fuel>0 {
                fuel = fuel/3-2;
                acc += if fuel > 0 {fuel} else {0}
            }
            acc as u64
        };
        inp.lines().map(|s| s.parse().map(calc_fuel)).into_iter().collect::<Result<Vec<u64>,_>>().unwrap().iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Input;

    #[test]
    fn simple_tests(){
        let result = D1::pt_1(Input::new("12".to_string()));
        assert_eq!(result,2);
    }

    #[test]
    fn simple_tests_2(){
        let result = D1::pt_2(Input::new("12".to_string()));
        assert_eq!(result,2);
    }

    #[test]
    fn run_pt1(){
        let result = D1::pt_1(Input::load_from_day(1));
        assert_eq!(result, 3239890);
    }
    #[test]
    fn run_pt2(){
        let result = D1::pt_2(Input::load_from_day(1));
        assert_eq!(result, 4856963);
    }
}