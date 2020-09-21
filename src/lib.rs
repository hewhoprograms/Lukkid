//! # Lukkid
//!
//! A library for Arithmetic Sequences.
//! 
use std::ops::AddAssign;
use std::cmp::PartialOrd;

/// Creating sequences
pub struct Sequence<T>{
    curr: T,
    step: T,
    stop: T,
}

/// Incarnation of Sequence Struct
pub struct Anukrom<T>{
    curr: T,
    step: T,
    stop: T,
}

/// As following this formular  
/// a1 + (a1 + d) + (a1 + 2d) + … + (a1 + (n – 1)d)
///
/// The First argument is the number for initail start.\n
/// The Second argument is the end of number or the last number of sequence.\n
/// The Third is common difference.\n
/// Anukrom::new(a1, (a1 + (n – 1)d, d= an+1 – an )
/// 
/// # Examples
///
/// ```
///let mut c = 0;
///for n in Anukrom::new(2,10,2){
///    println!("It is: {}", n);
///    c += n;
///}
/// ```
/// 
impl<T> Anukrom<T>{
    pub fn new(start:T, stop:T, step:T)->Self{
        Anukrom{
            curr: start,
            step: step,
            stop: stop,
        }
    }
}

/// As following this formular  
/// a1 + (a1 + d) + (a1 + 2d) + … + (a1 + (n – 1)d)
///
/// The First argument is the number for initail start.
/// 
/// The Second argument is the end of number or the last number of sequence.
/// 
/// The Third is common difference.
/// 
/// Sequence::new(a1, (a1 + (n – 1)d, d= an+1 – an )
/// 
/// # Examples
///
/// ```
///let mut c = 0;
///for n in Sequence::new(2,10,2){
///    println!("It is: {}", n);
///    c += n;
///}
/// ```
/// 
impl<T> Sequence<T>{
    pub fn new(start:T, stop:T, step:T)->Self{
        Sequence{
            curr: start,
            step: step,
            stop: stop,
        }
    }
}

impl<T> Iterator for Sequence<T>
    where T:AddAssign + Copy + PartialOrd
{
    type Item=T;
    fn next(&mut self)->Option<T>{
        if self.curr >= self.stop{
            return None;
        }else{
            let res = self.curr; 
            self.curr += self.step; 
            Some(res)
        }
    }
}

/// Sum all list in Sequence or Anukrom Struct
///
/// # Examples
///
/// ```
///let sl = zigma(Sequence::new(2,10,2), 0);
///assert_eq!(sl, 20);
/// ```
/// 
/// # Alternative 
/// We can use outsite method of this library as you need.
/// 
/// ```
///let fl = Sequence::new(2, 10, 2).fold(0,|acc,x|acc+x);
///assert_eq!(fl, 20);
/// ```
/// 
pub fn zigma<I,S>(l:I,mut s:S)->S
    where I: Iterator<Item=S>,
          S: AddAssign,
{
    let mut it = l.into_iter();
    while let Some(n) = it.next(){
        s += n
    }
    s

    /*
    let mut c = s;
    for n in l {
        c += n;
    }
    c*/
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut c = 0;
        for n in Sequence::new(2,10,2){
            println!("It is: {}", n);
            c += n;
        }
        assert_eq!(c, 20);
        let sl = zigma(Sequence::new(2,10,2), 0);
        assert_eq!(sl, 20);

        let fl = Sequence::new(2, 10, 2).fold(0,|acc,x|acc+x);
        assert_eq!(fl, 20);
    }
}
