use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    let mut a = Vec::new();

    for _ in 0..n {
        input! {
            element: i64,
        }
        a.push(element);
    }

    println!("original: {:?}", a);

    let r = a.len();

    let mut c = vec![0; r];

    merge_sort(&mut a, &mut c, 0, r);

    println!("answer: {:?}", a);
}

fn merge_sort(a: &mut Vec<i64>, c: &mut Vec<i64>, l: usize, r: usize) {
    if r - l != 1 {
        let m = (l + r) / 2;
        merge_sort(a, c, l, m);
        merge_sort(a, c, m, r);

        let mut first_count = l;
        let mut second_count = m;
        let mut count = 0;
        while first_count != m || second_count != r {
            if first_count == m {
                c[count] = a[second_count];
                second_count += 1;
            } else if second_count == r || a[first_count] < a[second_count] {
                c[count] = a[first_count];
                first_count += 1;
            } else {
                c[count] = a[second_count];
                second_count += 1;
            }
            count += 1;
        }

        a[l..(count + l)].copy_from_slice(&c[..count]);
        // for (i, e) in c.iter().enumerate() {
        //     a[i] = *e;
        // }
    }
}
