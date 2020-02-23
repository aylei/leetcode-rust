/**
 * [207] Course Schedule
 *
 * There are a total of n courses you have to take, labeled from 0 to n-1.
 *
 * Some courses may have prerequisites, for example to take course 0 you have to first take course 1, which is expressed as a pair: [0,1]
 *
 * Given the total number of courses and a list of prerequisite pairs, is it possible for you to finish all courses?
 *
 * Example 1:
 *
 *
 * Input: 2, [[1,0]]
 * Output: true
 * Explanation: There are a total of 2 courses to take.
 *              To take course 1 you should have finished course 0. So it is possible.
 *
 * Example 2:
 *
 *
 * Input: 2, [[1,0],[0,1]]
 * Output: false
 * Explanation: There are a total of 2 courses to take.
 *              To take course 1 you should have finished course 0, and to take course 0 you should
 *              also have finished course 1. So it is impossible.
 *
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

// problem: https://leetcode.com/problems/course-schedule/
// discuss: https://leetcode.com/problems/course-schedule/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// topology sort, BFS
use std::collections::VecDeque;
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
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
        let mut cnt = 0;
        while let Some(node) = deq.pop_front() {
            cnt += 1;
            for (i, &connect) in matrix[node].iter().enumerate() {
                if connect {
                    in_degree[i] -= 1;
                    if in_degree[i] == 0 {
                        deq.push_back(i);
                    }
                }
            }
        }
        cnt == num_courses
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_207() {
        assert_eq!(Solution::can_finish(2, vec![vec![1, 0]]), true);
        assert_eq!(Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]), false);
        assert_eq!(
            Solution::can_finish(
                8,
                vec![
                    vec![1, 0],
                    vec![2, 6],
                    vec![1, 7],
                    vec![6, 4],
                    vec![7, 0],
                    vec![0, 5]
                ]
            ),
            true
        );
    }
}
