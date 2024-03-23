use std::fmt;

#[derive(Debug, Clone)]
pub struct ProbabilityError;

impl fmt::Display for ProbabilityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Probability value error")
    }
}

pub fn to_class(prob: [f32; 4]) -> Result<String, ProbabilityError> {
    let class: String;
    if (prob[0] + prob[1] + prob[2] + prob[3]) != 1.0 {
        return Err(ProbabilityError);
    }
    if (prob[0] < 0.0) || (prob[1] < 0.0) || (prob[2] < 0.0) || (prob[3] < 0.0) {
        return Err(ProbabilityError)
    }
    if prob[0] > 0.5 {
        class = String::from("car");
    } else if prob[1] > 0.5 {
        class = String::from("human");
    } else if prob[2] > 0.5 {
        class = String::from("animal");
    } else if prob[3] > 0.5 {
        class = String::from("other");
    } else {
        class = String::from("undefined");
    }
    return Ok(class);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c2_01() {
        let result = to_class([1.0, 1.0, 1.0, 1.0]);
        assert!(result.is_err());
    }

    #[test]
    fn c2_03() {
        let result = to_class([-0.2, 0.0, 1.0, 0.0]);
        assert!(result.is_err());
    }

    #[test]
    fn c2_04() {
        let result = to_class([0.8, 0.05, 0.1, 0.05]);
        assert_eq!(result.unwrap(), String::from("car"));
    }

    #[test]
    fn c2_05() {
        let result = to_class([0.2, 0.6, 0.1, 0.1]);
        assert_eq!(result.unwrap(), String::from("human"));
    }

    #[test]
    fn c2_06() {
        let result = to_class([0.1, 0.1, 0.7, 0.1]);
        assert_eq!(result.unwrap(), String::from("animal"));
    }

    #[test]
    fn c2_07() {
        let result = to_class([0.02, 0.02, 0.02, 0.94]);
        assert_eq!(result.unwrap(), String::from("other"));
    }

    #[test]
    fn c2_08() {
        let result = to_class([0.5, 0.5, 0.0, 0.0]);
        assert_eq!(result.unwrap(), String::from("undefined"));
    }
    
    #[test]
    fn c2_09() {
        let result = to_class([0.25, 0.25, 0.25, 0.25]);
        assert_eq!(result.unwrap(), String::from("undefined"));
    }
}
