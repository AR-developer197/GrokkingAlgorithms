
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

fn main() {
    //chap 1 binary search
    let s = search(vec![5, 10, 15, 20, 25], 0);
    println!("{:#?}", s);

    
}
