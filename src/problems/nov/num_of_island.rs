pub fn num_of_island(mut grid: Vec<Vec<char>>) -> i32 {
    let mut num_island = 0;
    let m = grid.len();
    let n = grid[0].len();

    fn dfs(grid: &mut Vec<Vec<char>>, m: usize, n: usize, i: usize, j: usize) {
        grid[i][j] = '0';
        //left
        if i > 0 && grid[i - 1][j] == '1' {
            dfs(grid, m, n, i - 1, j);
        }
        //down
        if i < m - 1 && grid[i + 1][j] == '1' {
            dfs(grid, m, n, i + 1, j);
        }
        //up
        if j > 0 && grid[i][j - 1] == '1' {
            dfs(grid, m, n, i, j - 1);
        }
        //right
        if j < n - 1 && grid[i][j + 1] == '1' {
            dfs(grid, m, n, i, j + 1);
        }
    }

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == '1' {
                num_island += 1;
                dfs(&mut grid, m, n, i, j);
            }
        }
    }

    num_island
}

#[cfg(test)]
mod test {
    use super::*;

    // Should return 1 when given a grid with a single island
    #[test]
    fn test_single_island() {
        let grid: Vec<Vec<char>> = vec![
            vec!['1', '1', '1'],
            vec!['1', '1', '1'],
            vec!['1', '1', '1'],
        ];
        assert_eq!(num_of_island(grid), 1);
    }

    // Should return the correct number of islands when given a grid with multiple islands
    #[test]
    fn test_multiple_islands() {
        let grid: Vec<Vec<char>> = vec![
            vec!['1', '0', '1'],
            vec!['0', '1', '0'],
            vec!['1', '0', '1'],
        ];
        assert_eq!(num_of_island(grid), 5);
    }
}
