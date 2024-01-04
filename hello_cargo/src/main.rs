#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut sort_operations = vec![];
    let value = 1;

    list.sort_by_key(|r| {
        sort_operations.push(value);
        r.width
    });
    println!("{:#?}", list);
}

fn iterator_sum() {
   let v1 = vec![1, 2, 3];

   let v1_iter = v1.iter();

   let total: i32 = v1_iter.sum();
   dbg!(v1.iter());
   let v1: Vec<i32> = vec![1, 2, 3];

   let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

   assert_eq!(v2, vec![2, 3, 4]);

}