// //Exercitiul 1
// fn prime(n: i32) -> bool {
//     if n < 2 {
//         false
//     } else {
//         let mut d = 2;
//         while d * d <= n {
//             if n % d == 0 {
//                 return false;
//             }
//             d += 1;
//         }
//         true
//     }
// }
// fn main() {
//     let mut i = 0;
//     while i <= 100 {
//         if prime(i) {
//             println!("{i}");
//         }
//         i += 1
//     }
// }
// Exercitiul 2
// fn coprime(a: i32, b : i32) -> bool{
//     if a<1 || b<1{
//         return false;
//     }
//     let mut i = 2;
//     while i<=a && i<=b{
//         if a%i==0 && b%i==0
//         {return false}
//         i+=1;
//     }
//     true
// }
// fn main(){
//     let mut a = 1;
//     while a<=100{
//         let mut b = 1;
//         while b<=100{
//             if coprime(a,b){
//                 println!{"{a}, {b}"}
//             }
//             b+=1;
//         }
//         a+=1;
//     }
// }

// Exercitiul 3
fn sing(i:i32) {
        if i == 1{
            println!("1 bottle of beer on the wall,
1 bottle of beer.
Take one down, pass it around,
No bottles of beer on the wall.

No bottles of beer on the wall,
No bottles of beer.
Go to the store, buy some more,
99 bottles of beer on the wall.");
            return;
        }
        println!("{i} bottles of beer on the wall,
{i} bottles of beer.
Take one down, pass it around,
{} bottles of beer on the wall.
 ", i-1);
    }
fn main(){
    let mut i = 99;
    loop {
        sing(i);
        i-=1;
        if i==0{
            break;
        }
    }
}
