/**
 * [1009] Pancake Sorting
 *
 * Given an array A, we can perform a pancake flip: We choose some positive integer k <= A.length, then reverse the order of the first k elements of A.  We want to perform zero or more pancake flips (doing them one after another in succession) to sort the array A.
 *
 * Return the k-values corresponding to a sequence of pancake flips that sort A.  Any valid answer that sorts the array within 10 * A.length flips will be judged as correct.
 *
 *  
 *
 * Example 1:
 *
 *
 * Input: <span id="example-input-1-1">[3,2,4,1]</span>
 * Output: <span id="example-output-1">[4,2,4,3]</span>
 * Explanation:
 * We perform 4 pancake flips, with k values 4, 2, 4, and 3.
 * Starting state: A = [3, 2, 4, 1]
 * After 1st flip (k=4): A = [1, 4, 2, 3]
 * After 2nd flip (k=2): A = [4, 1, 2, 3]
 * After 3rd flip (k=4): A = [3, 2, 1, 4]
 * After 4th flip (k=3): A = [1, 2, 3, 4], which is sorted.
 *
 *
 * <div>
 * Example 2:
 *
 *
 * Input: <span id="example-input-2-1">[1,2,3]</span>
 * Output: <span id="example-output-2">[]</span>
 * Explanation: The input is already sorted, so there is no need to flip anything.
 * Note that other answers, such as [3, 3], would also be accepted.
 *
 *
 *  
 * </div>
 *
 * Note:
 *
 * <ol>
 * 	1 <= A.length <= 100
 * 	A[i] is a permutation of [1, 2, ..., A.length]
 * </ol>
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/pancake-sorting/
// discuss: https://leetcode.com/problems/pancake-sorting/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn pancake_sort(a: Vec<i32>) -> Vec<i32> {
        let len = a.len();
        if len <= 1 {
            return Vec::new();
        }
        let mut b = a.clone();
        let mut res: Vec<i32> = Vec::new();
        for i in 0..len {
            if i == (len - 1) {
                break;
            }
            let k = (len - i) as i32;
            let index = Solution::find_k(&b, k);
            if index == (k - 1) as usize {
                continue;
            }
            if index != 0usize {
                Solution::pancake_oper(&mut b, index, &mut res);
            }
            Solution::pancake_oper(&mut b, (k - 1) as usize, &mut res);
        }
        //        println!("{:?}", b);
        res
    }

    fn find_k(a: &Vec<i32>, k: i32) -> usize {
        for i in 0..(k - 1) {
            if a[i as usize] == k {
                return i as usize;
            }
        }
        (k - 1) as usize
    }

    pub fn pancake_oper(a: &mut Vec<i32>, index: usize, res: &mut Vec<i32>) {
        let mut helper = Vec::new();
        for i in 0..(index + 1) {
            helper.push(a[index - i]);
        }
        for i in 0..(index + 1) {
            a[i] = helper[i];
        }
        res.push((index + 1) as i32);
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
    use rand::seq::SliceRandom;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_1009() {
        for _i in 0..20 {
            let mut rng = rand::thread_rng();
            let size = rng.gen_range(0, 1000);
            let sorted_vector = make_sorted_vector(size);
            let mut shuffled_vector = make_shuffled_vector(&sorted_vector);
            let res = Solution::pancake_sort(shuffled_vector.clone());
            let oper_num = res.len();
            apply_pancake_sort_res(&mut shuffled_vector, res);
            assert_eq!(shuffled_vector, sorted_vector);
            assert!(oper_num < (size * 10) as usize);
        }
    }

    fn make_sorted_vector(i: i32) -> Vec<i32> {
        (1..i + 1).collect()
    }

    fn make_shuffled_vector(a: &Vec<i32>) -> Vec<i32> {
        let mut rng = thread_rng();
        let mut res = a.clone();
        res.shuffle(&mut rng);
        res
    }

    fn apply_pancake_sort_res(shuffled_vecter: &mut Vec<i32>, oper: Vec<i32>) {
        for i in oper {
            pancake_oper(shuffled_vecter, (i - 1) as usize);
        }
    }

    pub fn pancake_oper(a: &mut Vec<i32>, index: usize) {
        let mut helper = Vec::new();
        for i in 0..(index + 1) {
            helper.push(a[index - i]);
        }
        for i in 0..(index + 1) {
            a[i] = helper[i];
        }
    }
}
