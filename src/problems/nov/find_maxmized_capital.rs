use std::{cmp::Reverse, collections::BinaryHeap};
pub fn find_maximized_capital(mut k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
    let mut capital_bases_pqueue: BinaryHeap<(Reverse<i32>, i32)> = BinaryHeap::new();
    let mut profits_based_pqueue: BinaryHeap<i32> = BinaryHeap::new();

    let mut current_capital = w;

    for (profit, cap) in profits.into_iter().zip(capital) {
        if cap <= current_capital {
            profits_based_pqueue.push(profit);
        } else {
            capital_bases_pqueue.push((Reverse(cap), profit));
        }
    }

    while k > 0 {
        if let Some(profit) = profits_based_pqueue.pop() {
            current_capital += profit
        }

        while let Some(&(Reverse(cap), profit)) = capital_bases_pqueue.peek() {
            if cap <= current_capital {
                profits_based_pqueue.push(profit);
                capital_bases_pqueue.pop();
            } else {
                break;
            }
        }
        k -= 1;
    }

    current_capital
}

#[test]
fn test_1() {
    let k = 2;
    let w = 0;
    let profits = vec![1, 2, 3];
    let capital = vec![0, 1, 1];

    assert_eq!(4, find_maximized_capital(k, w, profits, capital));
}

// returns the correct maximum capital when k is greater than the number of available projects
#[test]
fn test_max_capital_k_greater_than_available_projects() {
    let k = 5;
    let w = 10;
    let profits = vec![1, 2, 3];
    let capital = vec![0, 1, 1];

    let result = find_maximized_capital(k, w, profits, capital);
    let expected = 16;
    assert_eq!(expected, result);
}

#[test]
fn test_max_capital_all_projects_capital_greater_than_w() {
    let k = 3;
    let w = 2;
    let profits = vec![1, 2, 3];
    let capital = vec![3, 4, 5];

    let result = find_maximized_capital(k, w, profits, capital);
    let expected = 2; // The initial capital should remain the same as no project can be started
    assert_eq!(expected, result, "Expected maximum capital to be {} but got {}", expected, result);
}