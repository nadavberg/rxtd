#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn pad_id_to_index_test() {
        let pad_ids = [
            "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "b1", "b2", "b3", "b4", "b5", "b6",
            "b7", "b8", "c1", "c2", "c3", "c4", "c5", "c6", "c7", "c8", "d1", "d2", "d3", "d4",
            "d5", "d6", "d7", "d8",
        ];
        for (index, &pad_id) in pad_ids.iter().enumerate() {
            assert_eq!(intermediate::pad_id_to_index(pad_id), index);
            assert_eq!(intermediate::pad_id_to_indexx(pad_id), Some(index));
        }
    }
}
