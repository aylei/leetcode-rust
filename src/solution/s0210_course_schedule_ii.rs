/**
 * [210] Course Schedule II
 *
 * There are a total of n courses you have to take, labeled from 0 to n-1.
 *
 * Some courses may have prerequisites, for example to take course 0 you have to first take course 1, which is expressed as a pair: [0,1]
 *
 * Given the total number of courses and a list of prerequisite pairs, return the ordering of courses you should take to finish all courses.
 *
 * There may be multiple correct orders, you just need to return one of them. If it is impossible to finish all courses, return an empty array.
 *
 * Example 1:
 *
 *
 * Input: 2, [[1,0]]
 * Output: [0,1]
 * Explanation: There are a total of 2 courses to take. To take course 1 you should have finished   
 *              course 0. So the correct course order is [0,1] .
 *
 * Example 2:
 *
 *
 * Input: 4, [[1,0],[2,0],[3,1],[3,2]]
 * Output: [0,1,2,3] or [0,2,1,3]
 * Explanation: There are a total of 4 courses to take. To take course 3 you should have finished both     
 *              courses 1 and 2. Both courses 1 and 2 should be taken after you finished course 0.
 *              So one correct course order is [0,1,2,3]. Another correct ordering is [0,2,1,3] .
 *
 * Note:
 *
 * <ol>
 * 	The input prerequisites is a graph represented by a list of edges, not adjacency matrices. Read more about <a href="https://www.khanacademy.org/computing/computer-science/algorithms/graph-representation/a/representing-graphs" target="_blank">how a graph is represented</a>.
 * 	You may assume that there are no duplicate edges in the input prerequisites.
 * </ol>
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/course-schedule-ii/
// discuss: https://leetcode.com/problems/course-schedule-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::collections::VecDeque;
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let num = num_courses as usize;
        let mut matrix = vec![vec![false; num]; num];
        let mut in_degree = vec![0; num];
        // collects node in degree
        for pre in prerequisites.iter() {
            if !matrix[pre[1] as usize][pre[0] as usize] {
                in_degree[pre[0] as usize] += 1;
            }
            matrix[pre[1] as usize][pre[0] as usize] = true;
        }
        let mut deq = VecDeque::new();
        // BFS starts with nodes with 0 in degree
        for (node, &v) in in_degree.iter().enumerate() {
            if v == 0 {
                deq.push_back(node);
            }
        }
        let mut res = Vec::with_capacity(num);
        while let Some(node) = deq.pop_front() {
            res.push(node as i32);
            for (i, &connect) in matrix[node].iter().enumerate() {
                if connect {
                    in_degree[i] -= 1;
                    if in_degree[i] == 0 {
                        deq.push_back(i);
                    }
                }
            }
        }
        if res.len() == num {
            res
        } else {
            vec![]
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_210() {
        assert_eq!(Solution::find_order(2, vec![vec![1, 0]]), vec![0, 1]);
        assert_eq!(
            Solution::find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]),
            vec![0, 1, 2, 3]
        );
    }
}
