use std::{collections::HashMap, usize};

fn main() {
    let coins = vec![1, 3, 5];
    let n = 11;
    let mut memory = HashMap::new();

    println!("Ways to make {}: {}", n, coin_combinations(&coins, n, 0, &mut memory));
}


fn coin_combinations(
    coins: &Vec<usize>,
    target: usize,
    value_index: usize,
    memory: &mut HashMap<(usize, usize), usize>,
) -> usize {
    //0 can allways be reached in only 1 way
    if target == 0 {
        return 1;
    }
    //if all types of coins have been tried
    if value_index >= coins.len() {
        return 0;
    }

    // if already tested that combination
    if let Some(combination) = memory.get(&(target, value_index)) {
        return *combination;
    }

    let current_coin = coins[value_index];
    //check number of combinatiosn without coins[value_index]
    let ways_excluding = coin_combinations(coins, target, value_index + 1, memory);

    //check number of combinations with coin[value_index]
    let ways_including: usize = if target >= current_coin  {
        coin_combinations(coins, target - current_coin, value_index, memory)
    } else {
        return 0;
    };

    let combinations = ways_including + ways_excluding;

    memory.insert((target, value_index), combinations);

    return combinations;
}
