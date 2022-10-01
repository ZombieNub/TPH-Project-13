/*
TPH has a large list of programming projects listed here:
https://github.com/the-programmers-hangout/programming-resources/blob/master/ideas.md
We are tackling project 13:
Given an array of stock values over time, find the period of time where the stocks could have made the most money.
No examples are given, so I'm going to make some up.
 */

use std::cmp::max;
use std::time::Instant;
use rand::Rng;

// This function returns a random sample of stock values.
// Item 1 is the timestamp, item 2 is the stock value.
fn random_stock_sample(max_stock_amount: usize, min_stock_amount: usize, max_stock_value: i32, min_stock_value: i32, allow_variance: bool, variance: i32) -> Vec<(usize, i32)> {
    let mut stock_values: Vec<(usize, i32)> = Vec::new(); // This is the list of stock values we will return.
    let mut rng = rand::thread_rng(); // This is the random number generator we will use.

    // We first need to find how many stock values we will generate.
    let stock_amount: usize = rng.gen_range(min_stock_amount..=max_stock_amount);
    // If the stock amount is 0, we will return an empty list.
    if stock_amount <= 0 {
        return stock_values;
    }

    if allow_variance {
        // If we are allowing variance, we need to start with a random value.
        let mut last_value: i32 = rng.gen_range(min_stock_value..=max_stock_value);
        stock_values.push((0, last_value));
        for i in 1..stock_amount {
            // For each stock value, we need to generate a new value.
            // We will generate a random value between -variance and variance, and add it to the last value.
            let variance: i32 = rng.gen_range(-variance..=variance);
            let mut new_value = last_value + variance;
            // We need to account for the possibility that the stock value is outside of the allowed range.
            if new_value > max_stock_value {
                new_value = max_stock_value;
            } else if new_value < min_stock_value {
                new_value = min_stock_value;
            }
            // We will then add the new value to the list.
            stock_values.push((i, new_value));
            last_value = new_value;
        }
    } else {
        // If we are not allowing variance, we can just generate a bunch of random values.
        for i in 0..stock_amount {
            stock_values.push((i, rng.gen_range(min_stock_value..=max_stock_value)));
        }
    }
    stock_values
}

fn find_best_buy_and_sell_naive(stocks: &Vec<(usize, i32)>) -> (usize, usize, i32) {
    /*
    Strategy for finding the best buy and sell times:
    The best times to buy and sell are the times where the difference between the buy and sell values is largest.
    This means we need to find the lowest value of the stock, and the highest value of the stock after that.
    We will begin with a naive approach: Select each value as the buy value, and then find the highest value after that.
    This will be O(n^2).
     */
    let mut buy_sell_sample: Vec<(usize, usize, i32)> = Vec::new();

    for (time, value) in stocks.iter() {
        let mut highest_value: i32 = *value;
        let mut highest_time: usize = *time;
        for (time2, value2) in stocks.iter() {
            if *time2 > *time && *value2 > highest_value {
                highest_value = *value2;
                highest_time = *time2;
            }
        }
        buy_sell_sample.push((*time, highest_time, highest_value - value));
    }

    // We will now find the best buy and sell times by finding the largest difference.
    let mut best_buy_sell: (usize, usize, i32) = (0, 0, 0);
    for (time, time2, value) in buy_sell_sample.iter() {
        if *value > best_buy_sell.2 {
            best_buy_sell = (*time, *time2, *value);
        }
    }
    best_buy_sell
    // This is a greedy algorithm which is guaranteed to find the best buy and sell times.
    // However, it is O(n^2), which is not ideal.
    // Lets try increasing the amount of stock values generated.

    // Okay it takes far, far too long to run with 100,000 stock values.
    // Can we think of a better strategy?
}

fn find_best_buy_and_sell_algo1(stocks: &Vec<(usize, i32)>) -> (usize, usize, i32) {
    // The naive approach is O(n^2), which is not ideal.
    // However, there is a critical reason why a different approach is difficult to find.
    // The sell time must always be after the buy time. I mean we're not bears.
    // If the sell time could be before the buy time, the most optimal strategy would be to buy at the lowest value, and sell at the highest value.
    // Therefore, the algorithm must be O(1) * the O(n) of the sorting algorithm.
    // Quicksort has an average O(n log n) runtime, so the resulting algorithm would be O(n log n).
    // We will do this naive approach, assuming that the stock values are always increasing. This guarantees that the best buy time is always before the best sell time.
    // This does also mean that the best buy time is the first time, and the best sell time is the last time in the original list.
    // But we won't account for that shortcut.
    let mut cloned_stocks = stocks.clone();
    cloned_stocks.sort_unstable_by(|a, b| a.1.cmp(&b.1));
    // Now that the array is sorted, we can find the best buy and sell times by looking at the first and last values and grabbing their times.
    // Except that's not exactly true. It's entirely possible that the buy time occurs after the sell time.
    // In this case, we need to shift to the next best buy time and/or the next best sell time.
    // How exactly do we do that?

    /*
    Strategy in the case of a buy time after the sell time:
    Create three instances: Shift the buy time to the right, the sell time to the left, and both.
    Find the best buy and sell times for each instance and pick the best one that is not a buy time after the sell time.
    If none of them are, then create three more instances and repeat.
    It's exceedingly unlikely that this will happen more than 3 times, so the average runtime should be the same as the sorting algorithm.
    However, it is technically possible that the entire list will be scanned. In this case, the runtime will be O(n).
    In either situation, this is far better than O(n^2) for the naive approach.
     */
    let mut buy_index = 0;
    let mut sell_index = cloned_stocks.len() - 1;
    loop {
        if cloned_stocks[buy_index].0 < cloned_stocks[sell_index].0 {
            let difference = cloned_stocks[sell_index].1 - cloned_stocks[buy_index].1;
            return (cloned_stocks[buy_index].0, cloned_stocks[sell_index].0, difference)
        }
        else {
            // We need to check the cases of shifting the buy time to the right and the sell time to the left, and pick the one with the greater difference.
            // If both have buy times greater than the sell time, we need to shift both and restart the loop.
            let buy_index2 = buy_index + 1;
            let sell_index2 = sell_index - 1;
            if (cloned_stocks[buy_index2].0 < cloned_stocks[sell_index].0) && (cloned_stocks[buy_index].0 < cloned_stocks[sell_index2].0) {
                let difference1 = cloned_stocks[sell_index].1 - cloned_stocks[buy_index2].1;
                let difference2 = cloned_stocks[sell_index2].1 - cloned_stocks[buy_index].1;
                return if difference1 > difference2 {
                    (cloned_stocks[buy_index2].0, cloned_stocks[sell_index].0, difference1)
                } else {
                    (cloned_stocks[buy_index].0, cloned_stocks[sell_index2].0, difference2)
                }
            } else if cloned_stocks[buy_index2].0 < cloned_stocks[sell_index].0 {
                return (cloned_stocks[buy_index2].0, cloned_stocks[sell_index].0, cloned_stocks[sell_index].1 - cloned_stocks[buy_index2].1)
            } else if cloned_stocks[buy_index].0 < cloned_stocks[sell_index2].0 {
                return (cloned_stocks[buy_index].0, cloned_stocks[sell_index2].0, cloned_stocks[sell_index2].1 - cloned_stocks[buy_index].1)
            } else {
                buy_index += 1;
                sell_index -= 1;
            }
        }
        // It's pretty much guaranteed that this will eventually terminate.
        // However, if it doesn't terminate for some ungodly reason, we will eventually run out of stock values.
        // This will cause an index out of bounds error, which will terminate the program.
        // So there's not going to be an infinite loop.
        // But if there is, I'm sorry.
    }
}

fn find_best_buy_and_sell_greedy(stocks: &Vec<(usize, i32)>) -> (usize, usize, i32) {
    /*
    Unlike the previous two algorithms, this one is not guaranteed to find the best buy and sell times.
    However, it will find a pretty good buy and sell time in O(n) time.
    Since we are not sorting the stock values, we will not have to worry about timestamps. Only the values matter.
     */
    let mut i: usize = 0;
    let mut res = 0;
    let mut max_i = 0;
    let mut max_j = 0;
    for j in 0..stocks.len() {
        if stocks[i].1 < stocks[j].1 {
            if (stocks[j].1 - stocks[i].1) > res {
                res = stocks[j].1 - stocks[i].1;
                max_i = i;
                max_j = j;
            }
        } else {
            i = j;
        }
    }
    (stocks[max_i].0, stocks[max_j].0, res)
}

fn main() {
    // We will generate a random sample of stock values.
    let stock_values: Vec<(usize, i32)> = random_stock_sample(
        1000000,
        1000000,
        5000000,
        0,
        false,
        1
    );

    // We will now find the best buy and sell times.
    let start_time = Instant::now();
    let best_buy_sell = find_best_buy_and_sell_algo1(&stock_values);
    let end_time = Instant::now();
    println!("Trial for algorithm 1:");
    println!("Best buy time: {}, best sell time: {}, best buy sell value: {}", best_buy_sell.0, best_buy_sell.1, best_buy_sell.2);
    println!("Value of stock at best buy time: {}", stock_values[best_buy_sell.0].1);
    println!("Value of stock at best sell time: {}", stock_values[best_buy_sell.1].1);
    println!("Time taken: {}ms", end_time.duration_since(start_time).as_millis());

    let start_time = Instant::now();
    let best_buy_sell = find_best_buy_and_sell_greedy(&stock_values);
    let end_time = Instant::now();
    println!("Trial for greedy:");
    println!("Best buy time: {}, best sell time: {}, best buy sell value: {}", best_buy_sell.0, best_buy_sell.1, best_buy_sell.2);
    println!("Value of stock at best buy time: {}", stock_values[best_buy_sell.0].1);
    println!("Value of stock at best sell time: {}", stock_values[best_buy_sell.1].1);
    println!("Time taken: {}ms", end_time.duration_since(start_time).as_millis());
}