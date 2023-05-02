use std::collections::HashSet;
pub struct Solution {}
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut ns: Vec<HashSet<Vec<u8>>> = vec![HashSet::new(); (n + 1) as usize];

        ns[0].insert(vec![1]);

        ns[1].insert(vec![1, 1]);
        ns[1].insert(vec![2]);

        for i in 2..ns.len() {
            // add 1 step to all elements in i - 1, one as prefix and one as suffix
            for step in ns[i - 1].clone() {
                let mut new_step = step.clone();
                new_step.push(1);
                ns[i].insert(new_step);

                let mut new_step = vec![1];
                new_step.extend(step);
                ns[i].insert(new_step);
            }
            // add 2 step to all elements in i - 2
            for step in ns[i - 2].clone() {
                let mut new_step = step.clone();
                new_step.push(2);
                ns[i].insert(new_step);

                let mut new_step = vec![2];
                new_step.extend(step);
                ns[i].insert(new_step);
            }
        }

        ns[n as usize - 1].len() as i32
    }
}
