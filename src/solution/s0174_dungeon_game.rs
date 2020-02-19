/**
 * [174] Dungeon Game
 *
 * <style type="text/css">table.dungeon, .dungeon th, .dungeon td {
 *   border:3px solid black;
 * }
 *
 *  .dungeon th, .dungeon td {
 *     text-align: center;
 *     height: 70px;
 *     width: 70px;
 * }
 * </style>
 * The demons had captured the princess (P) and imprisoned her in the bottom-right corner of a dungeon. The dungeon consists of M x N rooms laid out in a 2D grid. Our valiant knight (K) was initially positioned in the top-left room and must fight his way through the dungeon to rescue the princess.
 *
 * The knight has an initial health point represented by a positive integer. If at any point his health point drops to 0 or below, he dies immediately.
 *
 * Some of the rooms are guarded by demons, so the knight loses health (negative integers) upon entering these rooms; other rooms are either empty (0's) or contain magic orbs that increase the knight's health (positive integers).
 *
 * In order to reach the princess as quickly as possible, the knight decides to move only rightward or downward in each step.
 *
 *  
 *
 * Write a function to determine the knight's minimum initial health so that he is able to rescue the princess.
 *
 * For example, given the dungeon below, the initial health of the knight must be at least 7 if he follows the optimal path RIGHT-> RIGHT -> DOWN -> DOWN.
 *
 * <table class="dungeon">
 * 	<tbody>
 * 		<tr>
 * 			<td>-2 (K)</td>
 * 			<td>-3</td>
 * 			<td>3</td>
 * 		</tr>
 * 		<tr>
 * 			<td>-5</td>
 * 			<td>-10</td>
 * 			<td>1</td>
 * 		</tr>
 * 		<tr>
 * 			<td>10</td>
 * 			<td>30</td>
 * 			<td>-5 (P)</td>
 * 		</tr>
 * 	</tbody>
 * </table>
 *
 *  
 *
 * Note:
 *
 *
 * 	The knight's health has no upper bound.
 * 	Any room can contain threats or power-ups, even the first room the knight enters and the bottom-right room where the princess is imprisoned.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/dungeon-game/
// discuss: https://leetcode.com/problems/dungeon-game/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

/*
DP, 即从每个格子出发到达终点所需的最小生命值为 hp[i][j]

则显然, hp[M-1][N-1] = min(dungeon[M-1][N-1], 0) + 1;

hp[i][j] = min(min(hp[i+1][j], hp[i][j+1]) - dungeon[i][j], 1);

倒推到 hp[0][0] 即可

这里倒推很重要, 因为正推很难 dp(有后效性)

其实可以优化成 O(M+N) 空间复杂度, 从斜对角线往后推就只需要保存一个小数组, 但是下面这样更简明
*/
impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let (height, width) = (dungeon.len(), dungeon[0].len());
        // Using dummy row to simplify logic
        let mut hp = vec![vec![i32::max_value(); width + 1]; height + 1];
        hp[height][width - 1] = 1;
        hp[height - 1][width] = 1;
        for i in (0..height).rev() {
            for j in (0..width).rev() {
                hp[i][j] = i32::max(i32::min(hp[i + 1][j], hp[i][j + 1]) - dungeon[i][j], 1);
            }
        }
        hp[0][0]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_174() {
        assert_eq!(
            Solution::calculate_minimum_hp(vec![
                vec![-2, -3, 3],
                vec![-5, -10, 1],
                vec![10, 30, -5],
            ]),
            7
        );
        assert_eq!(
            Solution::calculate_minimum_hp(vec![vec![1, -4, 5, -99], vec![2, -2, -2, -1]]),
            3
        );
    }
}
