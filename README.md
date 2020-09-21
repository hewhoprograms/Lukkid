# A library for Arithmetic Sequences.

## Example 
- creating your sequence using Sequence()::new(?,?,?);
- sum all of list using zigma(<<put Sequence() Object here>> );
```
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
```
