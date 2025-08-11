fn main() {
    /* task: You have an array of integers, and for each index 
    you want to find the product of every integer except the integer 
    at that index. Create a function that takes an array of integers 
    and returns an array of the products */
    
    println!("{:?}", get_products(&vec![1, 2, 9, 3, 4, 5, 1]));
    println!("{:?}", get_products(&vec![0, 1, 2, 3, 4]));
    println!("{:?}", get_products(&vec![1, 0, 1, 2, 4, 0, 2]));
}

fn get_products(array: &Vec<i32>) -> Vec<i32> {
    if array.is_empty() {
        return Vec::new();
    }
    let mut amount_of_zeros: usize = 0 as usize;
    let mut indices_of_zeros: Vec<usize> = Vec::new();
    for (idx, num) in array.iter().enumerate() {
        if *num == 0 {
            amount_of_zeros += 1;
            indices_of_zeros.push(idx);
        }
    }
    if amount_of_zeros > 1 {
        return vec![0; array.len()];
    }
    let mut zeroless_product: i32 = array.iter().filter(|num| **num != 0)
    .fold(1, |acc: i32, next: &i32| acc * *next);

    if amount_of_zeros == 1 {
        let mut result: Vec<i32> = vec![0; array.len()];
        result.insert(indices_of_zeros[0], zeroless_product);
        return result;
    }
    return array.into_iter().map(|num: &i32| zeroless_product / *num).collect::<Vec<i32>>();


}

