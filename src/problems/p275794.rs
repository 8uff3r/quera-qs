use std::io;

pub fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Problem in input");
    input = input.trim_end().to_string();

    fn prcess_input(
        mut curr: String,
        mut acc: usize,
        mut gomrah: usize,
        mut tahdid: usize,
    ) -> (String, usize, usize) {
        loop {
            let current_substr: &str = &curr[acc..];
            let kalan_index = current_substr.find("kalan");
            if kalan_index.is_none() {
                break;
            }
            let kalan_index = kalan_index.unwrap();
            let tar_index = current_substr[kalan_index + "kalan".len()..].find("tar");
            if tar_index.is_none() {
                break;
            }
            let tar_index = tar_index.unwrap() + kalan_index + "kalan".len();
            let mid_kalantar = &current_substr[(kalan_index + "kalan".len())..(tar_index)];
            let has_non_space = mid_kalantar.find(|c| c != ' ');
            if has_non_space.is_some() {
                acc += kalan_index + "kalan".len();
                continue;
            }

            if mid_kalantar.is_empty() {
                tahdid += 1
            } else {
                gomrah += 1
            }
            let removed_kalantar =
                curr[..kalan_index + acc].to_string() + &curr[(tar_index + "tar".len() + acc)..];
            curr = removed_kalantar.trim().to_string();
        }
        (curr, gomrah, tahdid)
    }
    let mut current_step = input;
    let mut steps_details: Vec<(usize, usize)> = Vec::new();
    loop {
        let (next_str, gomrah, tahdid) = prcess_input(current_step, 0, 0, 0);
        current_step = next_str;
        if tahdid == 0 && gomrah == 0 {
            break;
        }
        steps_details.push((tahdid, gomrah));
    }
    println!("{:?}", steps_details.len());
    steps_details
        .iter()
        .for_each(move |x| println!("{:?} {:?}", x.0, x.1));
}
