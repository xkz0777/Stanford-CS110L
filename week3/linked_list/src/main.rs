use linked_list::ComputeNorm;
use linked_list::LinkedList;
pub mod linked_list;

fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);

    for i in 1..12 {
        list.push_front(i);
    }

    println!("List contents:{}", list);
    println!("List size: {}", list.get_size());
    println!("Top element: {}", list.pop_front().unwrap());
    println!("List contents:{}", list);
    println!("List size: {}", list.get_size());
    println!("Test `to_string`:");
    println!("{}", list.to_string()); // ToString impl for anything impl Display
    let copy = list.clone();
    println!("Copy of the list:{}", copy);

    println!("Iterating over &list:");
    for val in &list {
        println!("{}", val);
    }

    println!("Iterating over list:");
    for val in list {
        println!("{}", val);
    }

    // list can't be used anymore
    // println!("{}", list);

    let mut list = LinkedList::new();
    list.push_front(3.0);
    list.push_front(4.0);
    println!("Norm of 3 and 4: {}", list.compute_norm());
}
