fn dcg_jarvelin(rel: &Vec<usize>) -> f64 {
    let mut dcg = rel[0] as f64;
    for i in 1..rel.len() {
        dcg += rel[i] as f64 / f64::log2((i + 1) as f64);
    }
    dcg
}

fn dcg_burges(rel: &Vec<usize>) -> f64 {
    let mut dcg = 0.0 as f64;
    for i in 0..rel.len() {
        // TODO: 大きな数を入力しても`inf`, `NaN`を返さないようにする
        dcg += (f64::powi(2.0, rel[i] as i32) - 1.0) / f64::log2((i + 2) as f64);
    }
    dcg
}

/// Normalized Discounted Cumulative Gain
fn ndcg(rel: &Vec<usize>, use_burges: bool) -> f64 {
    let _n = rel.len();
    let mut ideal = rel.clone();
    ideal.sort();
    ideal.reverse();
    if use_burges {
        dcg_burges(&rel) / dcg_burges(&ideal)
    } else {
        dcg_jarvelin(&rel) / dcg_jarvelin(&ideal)
    }
}

pub fn ndcg_jarvelin(rel: &Vec<usize>) -> f64 {
    ndcg(rel, false)
}
pub fn ndcg_burges(rel: &Vec<usize>) -> f64 {
    ndcg(rel, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ndcg_jarvelin() {
        assert_eq!(ndcg_jarvelin(&vec![4, 3, 2, 1]), 1.0);
        assert_eq!(ndcg_jarvelin(&vec![4, 3, 1, 2]), 0.9850568531183683);
        assert_eq!(ndcg_jarvelin(&vec![4, 1, 2, 3]), 0.8858689757368545);
        assert_eq!(ndcg_jarvelin(&vec![100, 10, 1, 1]), 1.0);
        assert_eq!(ndcg_jarvelin(&vec![1, 10, 100, 1]), 0.6712170547169253);
        assert_eq!(ndcg_jarvelin(&vec![1, 4, 16, 1]), 0.7380118262192198);
        assert_eq!(
            ndcg_jarvelin(&vec![3, 3, 3, 3, 3, 0, 0, 0, 0, 5]),
            0.8804360184094201
        );
        assert_eq!(
            ndcg_jarvelin(&vec![5, 0, 0, 0, 0, 3, 3, 3, 3, 3]),
            0.7279443774455593
        );
    }
    #[test]
    fn test_ndcg_burges() {
        assert_eq!(ndcg_burges(&vec![4, 3, 2, 1]), 1.0);
        assert_eq!(ndcg_burges(&vec![4, 3, 1, 2]), 0.9935051443580323);
        assert_eq!(ndcg_burges(&vec![4, 1, 2, 3]), 0.9437153337836448);
        assert_eq!(ndcg_burges(&vec![100, 10, 1, 1]), 1.0);
        assert_eq!(ndcg_burges(&vec![1, 10, 100, 1]), 0.5);
        assert_eq!(ndcg_burges(&vec![1, 4, 16, 1]), 0.5000869216130837);
        assert_eq!(
            ndcg_burges(&vec![3, 3, 3, 3, 3, 0, 0, 0, 0, 5]),
            0.6280193149890032
        );
        assert_eq!(
            ndcg_burges(&vec![5, 0, 0, 0, 0, 3, 3, 3, 3, 3]),
            0.8946174017981632
        );
    }
}
