pub fn run(mut list: Vec<usize>) -> Vec<usize> {
    let mut i = 0;
    let mut j = 0;

    while i < list.len() - 1 {
        while j < list.len() - i - 1 {
            if list[j] > list[j + 1] {
                list.swap(j, j + 1);
            }
            j += 1;
        }
        i += 1;
        j = 0;
    }

    list
}
