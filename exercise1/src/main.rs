fn main() {
    let buffer1 = Buffer {data: vec![1, 2, 3, 4, 5]};
    let buffer2 = Buffer {data: vec![1.1, 2.5, 3.7, 4.5, 5.6]};
    let _sum1 = buffer1.sum();
    let _sum2 = buffer2.sum();
    println!("{:?}", _sum1);
    println!("{:?}", _sum2);
}

struct Buffer<T> {
    data: Vec<T>,
}

impl<T> Buffer<T>
where
    T: std::ops::Add + std::ops::AddAssign + Copy,
{
    fn sum(&self) ->Option<T>{
        let mut sum = self.data[0];
        for i in 1..self.data.len() {
            sum += self.data[i];
        }
        Some(sum)
    }
}
