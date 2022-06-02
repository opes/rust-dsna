pub fn run(mut list: Vec<usize>) -> Vec<usize> {
    let mut i = 0;
    let mut j = 0;

    while i < list.len() - 1 {
        println!("Outer, i: {}", &i);
        while j < list.len() - i - 1 {
            println!(
                "\tlist[{}] = {}\n\tlist[{}] = {}",
                &j,
                &list[j],
                &j + 1,
                &list[j + 1]
            );
            if list[j] > list[j + 1] {
                println!("\t\tSwap!");
                list.swap(j, j + 1);
            }
            j += 1;
        }
        i += 1;
        j = 0;
    }

    list
}
