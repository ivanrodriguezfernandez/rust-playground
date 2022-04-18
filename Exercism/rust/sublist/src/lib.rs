#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.len() > _second_list.len() && is_sublist(_first_list, _second_list) {
        Comparison::Superlist
    } else if _second_list.len() > _first_list.len() && is_sublist(_second_list, _first_list) {
        Comparison::Sublist
    } else if _first_list == _second_list {
        Comparison::Equal
    } else {
        Comparison::Unequal
    }
}

pub fn is_sublist<T: PartialEq>(b_list: &[T], s_list: &[T]) -> bool {
    s_list.len() == 0
        || b_list
            .windows(s_list.len())
            .any(|subslice| subslice == s_list)
}
