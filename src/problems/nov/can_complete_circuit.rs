pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    if cost.iter().sum::<i32>() > gas.iter().sum() {
        return -1;
    }
    let mut start_index = 0;
    let mut gas_tank = 0;

    for i in 0..gas.len() {
        gas_tank += gas[i] - cost[i];
        if gas_tank < 0 {
            start_index = i + 1;
            gas_tank = 0;
        }
    }
    start_index as i32
}

// The function returns 0 when gas and cost have only one element and gas is greater than or equal to cost.
#[test]
fn test_gas_greater_than_or_equal_to_cost() {
    let gas = vec![1,2,3,4,5];
    let cost = vec![3,4,5,1,2];
    assert_eq!(can_complete_circuit(gas, cost), 3);
}
