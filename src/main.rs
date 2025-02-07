
//chap 1 binary search
fn search(list: Vec<i32>, num: i32) -> Option<i32>{
    let mut found: Option<i32> = None;
    let mut start = 0;

    let mut end = list.len() as isize - 1;

    while start <= end  {
        let middle = (start + end) / 2;

        if list[middle as usize] == num {
            found = Some(list[middle as usize]);
            return found;
        }

        if list[middle as usize] > num {
            end = middle - 1
        } else {
            start = middle + 1
        }
    }

    found
}

//chap 2 selection sort
fn find_smallest(list: &Vec<i32>)  -> usize{
    let mut current_smallest_index = 0;
    let mut current_smallest_element = list[0];

    for (i, num) in list.iter().enumerate().skip(1) {
        if &current_smallest_element > num {
            current_smallest_index = i;
            current_smallest_element = list[i];
        }
    }

    current_smallest_index
}

fn selection_sort(mut list: Vec<i32>, mut new_list: Vec<i32>) -> Vec<i32>{
    if list.len() == 0 { 
        return new_list;
    } else {
        let smallest = find_smallest(&list);
        new_list.push(list.remove(smallest));

        let sorted_list = selection_sort(list, new_list);

        return sorted_list;
    }
}

//chap 3 recursion
fn factorial(x: i32) -> i32{
    if x == 1 {
        return x;
    } else {
        return x*factorial(x-1);
    }
}

fn main() {
    //chap 1 binary search
    search(vec![5, 10, 15, 20, 25], 0);

    //chap 2 selection sort
    let list = vec![4,9,8,10, 2];
    let new_list: Vec<i32> = Vec::new();
    selection_sort(list, new_list);

    //chap 3 recursion
    let n = factorial(3);
    println!("{}", n);

}
