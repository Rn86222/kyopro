/// 指定された要素以上の値が現れる最小のインデックス<br>条件を満たす値がない場合は 0 を返す
fn lower_bound<T: PartialOrd>(vec: &Vec<T>, bound: T) -> usize {
    let mut left: i64 = 0;
    let mut right: i64 = vec.len() as i64 - 1;
    while left <= right {
        let mid = (left + right) / 2;
        if vec[mid as usize] < bound {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    left as usize
}

/// 指定された要素より大きい値が現れる最小のインデックス<br>条件を満たす値がない場合は vec.len() を返す
fn upper_bound<T: PartialOrd>(vec: &Vec<T>, bound: T) -> usize {
    let mut left: i64 = 0;
    let mut right: i64 = vec.len() as i64 - 1;
    while left <= right {
        let mid = (left + right) / 2;
        if vec[mid as usize] <= bound {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    left as usize
}
