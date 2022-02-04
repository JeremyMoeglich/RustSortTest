use std::vec;

pub fn selection_sort(lst: &mut Vec<i64>) -> &mut Vec<i64> {
    let array_length = lst.len();
    for start_index in 0..array_length {
        let mut min: i64 = i64::MAX;
        let mut min_index: usize = 0;
        for current_index in start_index..array_length {
            let value = lst[current_index];
            if value < min {
                min_index = current_index;
                min = value;
            }
        }
        lst.swap(start_index, min_index);
    }
    lst
}

fn merge(lst: &mut [i64], split_point: usize) -> &mut [i64] {
    let mut merged_vector = vec![];
    let mut i1 = 0;
    let mut i2 = 0;
    let lst_len = lst.len();
    while i1 < split_point && split_point + i2 < lst_len {
        let v1 = lst[i1];
        let v2 = lst[split_point + i2];
        if v1 < v2 {
            merged_vector.push(v1);
            i1 += 1;
        } else {
            merged_vector.push(v2);
            i2 += 1;
        }
    }
    for v in &lst[i1..split_point] {
        merged_vector.push(*v)
    }
    for v in &lst[split_point + i2..] {
        merged_vector.push(*v)
    }
    for i in 0..lst_len {
        lst[i] = merged_vector[i];
    }
    lst
}

pub fn print_lst(lst: &[f32]) {
    println!(
        "{}",
        lst.iter()
            .map(|&id| id.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    )
}

pub fn merge_sort(lst: &mut [i64]) -> &mut [i64] {
    let array_length = lst.len();
    if array_length == 1 {
        return lst;
    }
    let split_point = array_length / 2;
    merge_sort(&mut lst[split_point..]);
    merge_sort(&mut lst[..split_point]);
    merge(lst, split_point)
}

pub fn bubble_sort(lst: &mut Vec<i64>) -> &mut Vec<i64> {
    let mut updated = true;
    while updated {
        updated = false;
        for i in 0..lst.len() - 1 {
            if lst[i] > lst[i + 1] {
                updated = true;
                lst.swap(i, i + 1);
            }
        }
    }
    lst
}

struct Pair<T> {
    v1: T,
    v2: T,
}

fn pairs<T: Copy>(lst: &Vec<T>) -> Vec<Pair<T>> {
    let mut vector = vec![];
    for i in 0..lst.len() - 1 {
        vector.push(Pair {
            v1: lst[i],
            v2: lst[i + 1],
        })
    }
    vector
}

fn is_sorted(lst: &Vec<i64>) -> bool {
    let mut values_sorted = true;
    for pair in pairs(lst) {
        if pair.v1 > pair.v2 {
            values_sorted = false;
            break;
        }
    }
    values_sorted
}

pub fn bogosort(lst: &mut Vec<i64>) -> &mut Vec<i64> {
    use rand::rngs::mock::StepRng;
    use shuffle::irs::Irs;
    use shuffle::shuffler::Shuffler;

    let mut rng = StepRng::new(2, 13);
    let mut irs = Irs::default();

    while !is_sorted(lst) {
        irs.shuffle(lst, &mut rng).expect("Sort Failed");
    }
    lst
}

fn get_length(n: f64) -> f64 {
    if n <= 0.0 {
        return 0.0;
    }
    let cf = |v: f64| 2.0_f64.powf(v / 15.0).ceil();
    let start_number = get_length(n - 1.0);
    let mut ci = n;
    while start_number >= cf(ci) {
        ci += 1.0
    }
    cf(ci)
}
pub fn merge_sort_vec(lst: &mut Vec<i64>) -> &mut Vec<i64> {
    merge_sort(lst);
    lst
}

fn main() {
    use rand::Rng;
    use std::time::SystemTime;

    let mut generator = rand::thread_rng();
    let mut times: Vec<f64> = vec![0.0];
    let mut current_iteration = 0 as f64;
    let funcs = [bubble_sort, selection_sort, bogosort, merge_sort_vec];
    for func_i in 0..funcs.len() {
        let func = funcs[func_i];
        while times[times.len() - 1] < 1.0 {
            current_iteration += 1.0;
            let length = get_length(current_iteration) as usize;
            let mut arr = vec![0; length];

            for i in 0..arr.len() {
                arr[i] = generator.gen_range(0..100);
            }
            let start_time = SystemTime::now();
            func(&mut arr);
            match start_time.elapsed() {
                Ok(elapsed) => {
                    times.push(elapsed.as_secs_f64());
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        }
        println!("{} - {}", times[times.len() - 1], func_i)
    }
}
