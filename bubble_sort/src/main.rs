fn main() {
    let mut arr = Vec::from([9, 3, 4, 0, 12, 5, 7, 2]);
    bubble_sort(&mut arr);
    println!("{:?}", arr);
}

fn bubble_sort(arr: &mut Vec<i32>) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                let tmp: i32 = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = tmp;
            }
        }
    }
}
