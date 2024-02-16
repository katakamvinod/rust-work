fn move_to_end(arr: &mut Vec<i32>, toMove: i32) -> Vec<i32> {
    let mut i = 0;
    let mut j = arr.len() - 1;
    while(i < j) {
        while(i < j && arr[j]==toMove) {
            j -= 1;
        }
        if (arr[i] == toMove) {
            let tmp = arr[j];
            arr[j] = arr[i];
            arr[i] = tmp;
        }
        i += 1
    }
    return arr.to_vec();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_move_to_end() {
        let mut v = vec![2, 1, 2, 2, 2, 3, 4, 2];
        let so = vec![4, 3, 1, 2, 2, 2, 2, 2];
        let arr = move_to_end(&mut v, 2);
        print!("{:#?}", arr);
    }
}
