fn insertion_sort<T: Clone + Ord>(a: &[T]) -> Box<[T]> {
    fn insert<T: Clone + Ord>(e: T, rest: &[T]) -> Box<[T]> {
        match rest {
            [] => Box::new([e]),
            [h, tail @ ..] => {
                if e <= *h {
                    let mut tmp: Vec<T> = vec![e];
                    tmp.append(&mut rest.to_owned());
                    tmp.into_boxed_slice()
                } else {
                    let mut tmp: Vec<T> = vec![h.to_owned()];
                    tmp.append(&mut insert(e, tail).to_vec());
                    tmp.into_boxed_slice()
                }
            }
        }
    }
    match a {
        [] | [_] => Box::from(a),
        [e, rest @ ..] => insert(e.to_owned(), &insertion_sort(rest)),
    }
}

fn merge_sort<T: Clone + Ord>(a: &[T]) -> Box<[T]> {
    fn merge<T: Clone + Ord>(l1: &[T], l2: &[T]) -> Box<[T]> {
        match (l1, l2) {
            ([], l) | (l, []) => Box::from(l),
            ([e1, tail1 @ ..], [e2, tail2 @ ..]) => {
                if e1 <= e2 {
                    let mut tmp: Vec<T> = vec![e1.to_owned()];
                    tmp.append(&mut merge(tail1, l2).to_vec());
                    tmp.into_boxed_slice()
                } else {
                    let mut tmp: Vec<T> = vec![e2.to_owned()];
                    tmp.append(&mut merge(l1, tail2).to_vec());
                    tmp.into_boxed_slice()
                }
            }
        }
    }

    match a {
        [] | [_] => Box::from(a),
        other => merge(
            &merge_sort(&other[..other.len() / 2]),
            &merge_sort(&other[other.len() / 2..]),
        ),
    }
}

fn quick_sort<T: Clone + Copy + Ord>(a: Vec<T>) -> Vec<T> {
    match a.as_slice() {
        [] => a,
        [pivot, tail @ ..] => {
            let (l1, l2): (Vec<T>, Vec<T>) = tail.iter().partition(|&e| e <= pivot);
            let mut tmp = quick_sort(l1);
            tmp.push(*pivot);
            tmp.append(&mut quick_sort(l2));
            tmp
        }
    }
}

fn main() {
    const A: [u8; 10] = [3, 1, 9, 6, 7, 0, 2, 5, 4, 8];
    let res_insertion1: Box<[u8]> = insertion_sort(&A);
    println!("insertion sort: {:?}", res_insertion1);

    let res_merge1: Box<[u8]> = merge_sort(&A);
    println!("merge sort: {:?}", res_merge1);

    let res_quick1: Vec<u8> = quick_sort(A.to_vec());
    println!("quick sort: {:?}", res_quick1);

    const B: [u8; 1] = [42];

    let res_insertion2: Box<[u8]> = insertion_sort(&B);
    println!("insertion sort: {:?}", res_insertion2);

    let res_merge2: Box<[u8]> = merge_sort(&B);
    println!("merge sort: {:?}", res_merge2);

    let res_quick2: Vec<u8> = quick_sort(B.to_vec());
    println!("quick sort: {:?}", res_quick2);
}
