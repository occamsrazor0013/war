#![allow(non_snake_case,non_camel_case_types,dead_code)]

/*
    Below is the function stub for deal. Add as many helper functions
    as you like, but the deal function should not be modified. Just
    fill it in.
    
    Test your code by running 'cargo test' from the war_rs directory.
*/

// this was similar to how I did it in elixir but the complexity of immutability, mutability and typing made it more difficult
fn deal(shuf: &[u8; 52]) -> [u8; 52] 
{
    // given a reference to the array, I had to create a new vector, reverse it, then replace all 1's with 14's for the ace value
    let mut reversed = shuf.to_vec();
    reversed.reverse();
    reversed.iter_mut().for_each(|x| 
        if *x == 1 {
            *x = 14;
        });
    // off of that vector, create two more vectors for two players
    // vector 1 is the original vector stepping by 2
    // vector 2 is the original vector skipping first element and stepping by 2
    // take those two vectors and an empty array as input parameters for the war function
    let p1: Vec<_> = reversed.iter().step_by(2).copied().collect();
    let p2: Vec<_> = reversed.iter().skip(1).step_by(2).copied().collect();
    let mut output = war_time_all_the_time(p1, p2, Vec::new());
    // take the array that is returned and replace all 14's with 1's for real world value
    output.iter_mut().for_each(|x| 
        if *x == 14 {
            *x = 1;
        });
    //create new array, map the vector values returned by the war function into that array and return it
    let mut array: [u8; 52] = [0; 52];
    for i in 0..52 {
        array[i] = output[i];
    }
    return array
}

fn war_time_all_the_time<T: Clone + std::cmp::Ord + std::fmt::Debug>(p1: Vec<T>, p2: Vec<T>, mut stack: Vec<T>) -> Vec<T>
{
    //pattern matching
    //1st pattern, if both players piles are empty and end on a war, simply return the stack sorted descending
    //2nd pattern, if p2's pile is empty, p1 wins, return p1 pile appended with stack sorted descnding
    //3rd pattern, if p1's pile is empty, p2 wins, return p2 pile appended with stack sorted descending
    match (p1.as_slice(), p2.as_slice(), stack.clone()) {
        ([], [], _) => {
            stack.sort_by(|a, b| b.cmp(a));
            return stack
        },
        (p1, [], _) => {
            stack.sort_by(|a, b| b.cmp(a));
            let mut p1_vec = p1.to_vec();
            p1_vec.extend(stack);
            return p1_vec
        },
        ([], p2, _) => {
            stack.sort_by(|a, b| b.cmp(a));
            let mut p2_vec = p2.to_vec();
            p2_vec.extend(stack);
            return p2_vec
        },
        //4th pattern, playing game of war
        (&[_, ..], &[_, ..], _) => {
            // elixir doesn't have reliable head tail syntax for vectors, so I had to use slices of the two vectors using split_at
            // inital stack of cards contain just the heads of each pile, sorted descending
            let (h1, t1) = p1.split_at(1);
            let (h2, t2) = p2.split_at(1);
            stack.extend_from_slice(&h1);
            stack.extend_from_slice(&h2);
            stack.sort_by(|a, b| b.cmp(a));
            // if h1 is greater than h2 in rank, continue game with p1's remaining pile appended with the stack and p2's remaining pile
            if h1 > h2 {
                let mut t1_vec = t1.to_vec();
                t1_vec.extend(stack);
                let t2_vec = t2.to_vec();
                war_time_all_the_time(t1_vec, t2_vec, Vec::new())
            }
            // if h1 is less than h2 in rank, continue game with p1's remaining pile and p2's remaining pile appended with the stack
            else if h1 < h2 {
                let t1_vec = t1.to_vec();
                let mut t2_vec = t2.to_vec();
                t2_vec.extend(stack);
                war_time_all_the_time(t1_vec, t2_vec, Vec::new())
            }
            // remaining case is if they are equal, war starts, first condition check if they're empty
            // if they're not empty, pop heads off of both piles, add those to the stack, continue game
            else if !t1.is_empty() && !t2.is_empty() {
                let (facedown1, t1) = t1.split_at(1);
                let (facedown2, t2) = t2.split_at(1);
                let t1_vec = t1.to_vec();
                let t2_vec = t2.to_vec();
                stack.extend_from_slice(facedown1);
                stack.extend_from_slice(facedown2);
                war_time_all_the_time(t1_vec, t2_vec, stack)
            }
            // last case is if both tails are empty, still have to continue game
            else {
                let t1_vec = t1.to_vec();
                let t2_vec = t2.to_vec();
                war_time_all_the_time(t1_vec, t2_vec, stack)
            }
        }
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;

