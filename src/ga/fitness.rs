use crate::variables::Item;
use crate::ga::chromosome::Chromosome;

pub fn fitness(chromosome: &Chromosome, items: &[Item], max_weight: u32) -> i32 {
    let (total_value, total_weight) = chromosome.iter().enumerate().fold((0u64, 0u64), |(value_acc, weight_acc), (i, &include_item)| {
        if include_item {
            let added_value = items[i].value as u64 + value_acc;
            let added_weight = items[i].weight as u64 + weight_acc;
            // Check for overflow
            if added_value > u64::MAX - items[i].value as u64 || added_weight > u64::MAX - items[i].weight as u64 {
                return (u64::MAX, u64::MAX); // Indicate overflow by setting to max
            }
            (added_value, added_weight)
        } else {
            (value_acc, weight_acc)
        }
    });

    // Check for overflow or overweight
    if total_weight > max_weight as u64 || total_weight == u64::MAX {
        return 0;
    }

    // Assuming total_value fits into i32
    total_value as i32
}
