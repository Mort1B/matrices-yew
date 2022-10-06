pub struct Matrix {
    input: Vec<Vec<i32>>,
}

impl Matrix {
    pub fn new(input: Vec<Vec<i32>>) -> Matrix {
        return Self { input };
    }

    pub fn det(&self) -> Option<i32> {
        //this is to get the determinants of the matrix
        // passes it to the sub function that is recursive
        self.det_calc(&self.input)
    }
    fn det_calc(&self, input: &Vec<Vec<i32>>) -> Option<i32> {
        //this is to get the determinants of the matrix
        let rows = input.len();

        //check if there is something there
        if rows == 0 {
            return None;
        }

        // check if all cols have the same legth as the total number of rows
        for row in input {
            if row.len() != rows {
                return None;
            }
        }

        // now at this point we know its a n * m array
        if rows == 1 {
            return Some(input[0][0]);
        }

        // now for the recursive part
        let mut sum: i32 = 0;

        for (position, number) in input[0].iter().enumerate() {
            let mut sub_vec: Vec<Vec<i32>> = vec![];

            for (i, row) in input.iter().enumerate() {
                if i == 0 {
                    continue;
                }
                let mut sub_vec_row: Vec<i32> = vec![];
                for (j, col) in row.into_iter().enumerate() {
                    if j == position {
                        continue;
                    }
                    sub_vec_row.push(*col);
                }
                sub_vec.push(sub_vec_row);
            }

            // if calculating the sub determite fails then pass the failure up
            if let Some(sub_det) = self.det_calc(&sub_vec) {
                let sign: i32 = if position % 2 == 0 { 1 } else { -1 };
                sum += number * sub_det * sign
            } else {
                return None;
            }
        }
        Some(sum)
    }

    pub fn transpose(&self) -> Vec<Vec<i32>> {
        let mut result = vec![];

        let mut max_rows = 0;

        for (i, row) in self.input.iter().enumerate() {
            if i == 0 {
                max_rows = row.len();

                for _ in 0..max_rows {
                    result.push(vec![])
                }
            }

            for (j, col) in row.iter().enumerate() {
                if j >= max_rows {
                    continue;
                }
                result[j].push(*col)
            }
        }

        result
    }

    pub fn add(&self, other: &Vec<Vec<i32>>) -> Option<Vec<Vec<i32>>> {
        //check rows are the same
        if self.input.len() != other.len() {
            return None;
        }

        //will pop the results here
        let mut result = self.input.clone();

        for (i, row) in self.input.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if j >= other[i].len() {
                    return None;
                }
                result[i][j] = *col + other[i][j]
            }
        }

        Some(result)
    }

    pub fn mul_scalar(&self, k: i32) -> Vec<Vec<i32>> {
        let mut result = self.input.clone();

        for (i, row) in self.input.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                result[i][j] = *col * k;
            }
        }

        result
    }
}
