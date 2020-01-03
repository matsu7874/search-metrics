/// Binary Preference-based measure
fn bpref(rel: &Vec<bool>) -> f64 {
    let count_relevant = rel.iter().filter(|x| **x).count();
    if count_relevant == 0 {
        return 0.0;
    }

    let mut i = rel.len() - 1;
    while !rel[i] {
        i -= 1;
    }
    let count_non_relevant = rel[0..i].iter().filter(|x| !**x).count();
    if count_non_relevant == 0 {
        return 1.0;
    }
    let count_relevant = count_relevant as f64;
    let mut n = 0.0;
    let mut ret = 0.0;
    for c in rel.iter() {
        if *c {
            eprintln!("{}/{}", n, count_relevant);
            ret += (1.0 - n / count_relevant) / count_relevant;
        } else {
            n += 1.0;
        }
    }
    ret
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bpref() {
        assert_eq!(bpref(&vec![true, true,]), 1.0);
        assert_eq!(bpref(&vec![false, false, false]), 0.0);
        assert_eq!(bpref(&vec![false, true, true, true, true]), 0.75);
        assert_eq!(
            bpref(&vec![false, true, true, false, true]),
            0.5555555555555556 // 1/3*((1-1/3)+(1-1/3)+(1-2/3))
        );
        assert_eq!(
            bpref(&vec![
                true, false, true, true, true, false, true, true, true, true, false, false, true,
                false, false, true, false, false, false, true, false, true
            ]),
            0.7222222222222221
        );
    }
}
