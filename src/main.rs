fn selection_sort(lst: &mut [i64]) {
    lst[0] = 5
}

fn main() {
    let mut arr = vec![1, 2, 3];
    selection_sort(&mut arr);
    println!(
        "{}",
        arr.iter()
            .map(|&id| id.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    )
}
