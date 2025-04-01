struct Solution;

use std::collections::HashMap;

impl Solution {
    // Exercise 1
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {

        let mut map = HashMap::new();
        for i in arr {
            let count = map.entry(i).or_insert(0);
            *count += 1;
        }
        let mut set = std::collections::HashSet::new();
        for (_, &v) in map.iter() {
            if !set.insert(v) {
                return false;
            }
        }
        true
        
    }



    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut row_map: HashMap<Vec<i32>, i32> = HashMap::new();
        let n = grid.len();

        // Conta le occorrenze di ogni riga
        for row in &grid {
            *row_map.entry(row.clone()).or_insert(0) += 1;
        }

        let mut count = 0;

        // Confronta ogni colonna con le righe
        for col in 0..n {
            let mut column = Vec::new();
            for row in 0..n {
                column.push(grid[row][col]);
            }
            if let Some(&freq) = row_map.get(&column) {
                count += freq;
            }
        }

        count
    }

    }


fn main() {
    let arr = vec![1,2,2,1,1,3];
    let result = Solution::unique_occurrences(arr);
    println!("{}", result);

    let grid: Vec<Vec<i32>> = vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]];
    println!("{}", Solution::equal_pairs(grid.to_vec()));
}
