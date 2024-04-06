use std::fmt;

#[derive(Debug, Clone)]
pub struct SizeMisMatchError;

impl fmt::Display for SizeMisMatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Matrix size mismatch")
    }
}

pub fn matmul(a: Vec<Vec<f32>>, b: Vec<Vec<f32>>) -> Result<Vec<Vec<f32>>, SizeMisMatchError> {
    if a[0].len() != b.len() {
        return Err(SizeMisMatchError);
    } 
    
    let mut res = vec![vec![0.0; b[0].len()]; a.len()];

    for i in 0..a.len() {
        for j in 0..b[0].len() {
            let mut sum: f32 = 0.0;
            for k in 0..a[0].len() {
                sum += a[i][k] + b[k][j];
            }
            res[i][j] = sum;
        }
    }

    return Ok(res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = matmul(vec![vec![1.0 as f32; 10]; 12], vec![vec![2.0 as f32; 5]; 10]);
        assert_eq!(result.unwrap(), vec![vec![30.0 as f32; 5]; 12]);
    }
}
