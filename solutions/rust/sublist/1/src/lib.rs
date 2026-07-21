#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list{ return Comparison::Equal }
    else if first_list.len() == 0 && second_list.len() != 0 {return Comparison::Sublist}
    else if first_list.len() != 0 && second_list.len() == 0 {return Comparison::Superlist}  
    else if second_list.windows(first_list.len()).any(|window| window == first_list){return Comparison::Sublist}
    else if first_list.windows(second_list.len()).any(|window| window == second_list){return Comparison::Superlist}
    else {
        Comparison::Unequal
    }
}
