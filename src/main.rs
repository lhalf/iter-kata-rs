fn main() {
    todo!()
}

fn sum_of(_values: &[i32]) -> i32 {
    todo!()
}

fn all_values_greater_than_10(_values: &[i32]) -> bool {
    todo!()
}

fn sum_of_squares(_values: &[i32]) -> i32 {
    todo!()
}

#[allow(dead_code)]
struct Person {
    eyes: u8,
    height: u8,
    name: &'static str,
    shops_at_tesco: bool
}

fn who_has_two_eyes(_peoples: &[Person]) -> Vec<&'static str> {
    todo!()
}

fn how_tall_is_that_first_billy(_peoples: &[Person]) -> Option<u8> {
    todo!()
}

#[allow(unused_imports)]
use itertools::Itertools;
fn how_tall_is_the_third_tallest(_peoples: &[Person]) -> Option<u8> {
    todo!()
}

#[allow(dead_code)]
struct Hat {
    name: &'static str,
    height: u8
}

fn how_tall_is_the_tallest_when_assigned_hats(_peoples: &[Person], _hats: &[Hat]) -> Option<u8> {
    _peoples.iter()
        .sorted_by_key(|person| person.height)
        .zip(_hats.iter().sorted_by_key(|hat| hat.height).rev())
        .map(|(person, hat)| person.height + hat.height)
        .sorted()
        .rev()
        .nth(0)
}

fn three_adjacent_people_height_ordered_with_most_eyes(_peoples: &[Person]) -> Option<(&str, &str, &str)> {
    _peoples.iter()
        .sorted_by_key(|person| person.height)
        .tuple_windows()
        .max_by_key(|(first, second, third)| first.eyes + second.eyes + third.eyes)
        .map(|(first, second, third)| (first.name, second.name, third.name))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of() {
        assert_eq!(32, sum_of(&[5, 10, 5, 12]))
    }

    #[test]
    fn test_all_values_greater_than_10() {
        assert_eq!(false, all_values_greater_than_10(&[14, 54, 2, 27]))
    }

    #[test]
    fn map_to_sum_of_squares() {
        assert_eq!(38, sum_of_squares(&[2, 3, 5]))
    }

    const PEOPLES: [Person; 7] = [
        Person{eyes: 2, height: 179, name: "Billy", shops_at_tesco: true},
        Person{eyes: 1, height: 200, name: "Cyclops", shops_at_tesco: true},
        Person{eyes: 2, height: 189, name: "Alice", shops_at_tesco: false},
        Person{eyes: 0, height: 240, name: "Entity of the void", shops_at_tesco: true},
        Person{eyes: 2, height: 231, name: "Billy", shops_at_tesco: false},
        Person{eyes: 8, height: 10, name: "Tarantula", shops_at_tesco: false},
        Person{eyes: 3, height: 5, name: "Blinky", shops_at_tesco: true},
    ];

    #[test]
    fn collect_names_of_people_with_two_eyes() {
        assert_eq!(vec!["Billy", "Alice", "Billy"], who_has_two_eyes(&PEOPLES))
    }

    #[test]
    fn find_height_of_first_billy() {
        assert_eq!(Some(179), how_tall_is_that_first_billy(&PEOPLES))
    }

    #[test]
    fn find_height_of_third_tallest_person() {
        assert_eq!(Some(200), how_tall_is_the_third_tallest(&PEOPLES))
    }

    const HATS: [Hat; 8] = [
        Hat{name: "Sorting hat", height: 30},
        Hat{name: "Top hat", height: 60},
        Hat{name: "A rubber duck", height: 5},
        Hat{name: "An actual living duck", height: 25},
        Hat{name: "Bowler hat", height: 22},
        Hat{name: "Papier-mâché volcano", height: 20},
        Hat{name: "Bucket", height: 30},
        Hat{name: "Flat cap", height: 1},
    ];

    #[test]
    fn if_tallest_people_assigned_shortest_hats_find_height_of_tallest_person_wearing_hat() {
        assert_eq!(Some(251), how_tall_is_the_tallest_when_assigned_hats(&PEOPLES, &HATS))
    }

    #[test]
    fn if_people_sat_down_in_height_order_which_3_adjacent_people_have_the_most_total_eyes() {
        assert_eq!(Some(("Blinky", "Tarantula", "Billy")), three_adjacent_people_height_ordered_with_most_eyes(&PEOPLES))
    }
}
