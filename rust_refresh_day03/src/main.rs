fn main() {
    let arr: [f64; 5] = [10.12341,20.1,30.41,40.7,12.1];
    println!("배열 전체: {:?}",arr);
    println!("배열 첫번쨰: {}",arr[0]);

    let mut vec = vec![1,2,3];
    vec.push(4);
    println!("{:?}",vec);
    
    for x in &vec{
        println!("vec {}",x);
    }

    let mut i = 0;
    while i < vec.len(){
        println!("while {}",vec[i]);
        i +=1;
    }

    let mut count = 0;
    loop{
        if count >= 99 {
            break;
        }
        println!("{}", count);
        count += 1;

    }
    // 6. 슬라이스 (&[T])
    let slice = &arr[1..3]; // arr의 1번부터 2번까지 (3은 포함X)
    println!("슬라이스: {:?}", slice);
}
