pub mod problem1 {
    pub fn sum(slice: &[i32]) -> i32 {
        let mut ans: i32 = 0;
        for num in slice {
            ans += num;
        }
        ans
    }
    pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        for n in vs {
            if !ans.contains(n) {
                ans.push(*n);
            }
        }
        ans
    }

    pub fn filter(vs: &Vec<i32>, pred: &dyn Fn(i32) -> bool) -> Vec<i32> {
        let mut ans = vec![];
        for n in vs {
            if pred(*n) {
                ans.push(*n);
            }
        }
        ans
    }
    #[cfg(test)]
    mod tests {
        use super::{sum, dedup, filter};
        #[test]
        fn test_sum_small() {
            let array = [1,2,3,4,5];
            assert_eq!(sum(&array), 15);
        }

        #[test]
        fn test_dedup_small() {
            let vs = vec![1,2,2,3,4,1];
            assert_eq!(dedup(&vs), vec![1,2,3,4]);
        }

        fn even_predicate(x: i32) -> bool {
            (x % 2) == 0
        }

        #[test]
        fn test_filter_small() {
            let vs = vec![1,2,3,4,5];
            assert_eq!(filter(&vs, &even_predicate), vec![2,4]);
        }
    }
}

pub mod problem2 {
    pub type Matrix = Vec<Vec<f32>>;

    pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
        let size = mat1.len();
        let mut ans: Matrix = vec![vec![0.; size]; size];
        for i in 0..size {
            for j in 0..size {
                let mut sum = 0.;
                for l in 0..size {
                    sum += mat1[i][l] * mat2[l][j];
                }
                ans[i][j] = sum;
            }
        }
        ans
    }

    #[cfg(test)]
    mod tests{
        use super::mat_mult;
        #[test]
        fn test_mat_mult_identity() {
            let mut mat1 = vec![vec![0.;3]; 3];
            for i in 0..mat1.len() {
                mat1[i][i] = 1.;
            }
            let mat2 = vec![vec![5.;3]; 3];
            let result = mat_mult(&mat1, &mat2);
            for i in 0..result.len() {
                for j in 0..result[i].len() {
                    assert_eq!(result[i][j], mat2[i][j]);
                }
            }
        }

        #[test]
        fn test_mat_mult_normal() {
            let mut mat1 = vec![vec![0.;3]; 3];
            let mut cnt = 0;
            // mat1:
            //   0 1 2
            //   3 4 5
            //   6 7 8
            for i in 0..mat1.len() {
                for j in 0..mat1.len() {
                    mat1[i][j] = cnt as f32;
                    cnt += 1;
                }
            }
            // mat2:
            //   0 1 2
            //   1 2 3
            //   2 3 4
            let mut mat2 = vec![vec![0.;3]; 3];
            for i in 0..mat2.len() {
                for j in 0..mat2.len() {
                    mat2[i][j] = (i + j) as f32;
                }
            }
            let result = mat_mult(&mat1, &mat2);
            let ans = vec![
                vec![5.,8.,11.],
                vec![14.,26.,38.],
                vec![23.,44.,65.]
            ];
            for i in 0..result.len() {
                for j in 0..result[i].len() {
                    assert_eq!(result[i][j], ans[i][j]);
                }
            }
        }
    }
}

// pub mod problem3 {
//     pub fn sieve(n: u32) -> Vec<u32> {
//         todo!();
//     }
//     #[cfg(test)]
//     mod tests{
//         use super::sieve;
//
//         #[test]
//         fn test_sieve_basic() {
//             assert_eq!(vec![2,3,5,7,11], sieve(12));
//         }
//     }
// }
//
// pub mod problem4 {
//
//     #[derive(Clone, Copy, Debug, Eq, PartialEq)]
//     pub enum Peg {
//         A,
//         B,
//         C,
//     }
//
//     pub fn hanoi(num_discs: u32, src: Peg, aux: Peg, dst: Peg) -> Vec<Move> {
//         todo!();
//     }
//     #[cfg(test)]
//     mod tests{
//         use super::{hanoi, Peg};
//         #[test]
//         fn test_hanoi_1_disks() {
//             let result = hanoi(1, Peg::A, Peg::B, Peg::C);
//             assert_eq!(vec![(Peg::A, Peg::C)], result);
//             assert_eq!(1, result.len());
//         }
//     }
// }
//
