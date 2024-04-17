pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod solution {
    use std::cmp::Ordering;
    use std::collections::{LinkedList, VecDeque};
    use std::ops::Add;
    use std::ops::AddAssign;
    use std::hash::Hasher;
    use std::hash::Hash;

    #[derive(Copy, Clone, Debug)]
    pub struct ComplexNumber{
        real: f64,
        imag: f64,
    }

    impl ComplexNumber{
        pub fn new(r:f64,i:f64) -> ComplexNumber {
            ComplexNumber {
                real: r,
                imag: i,
            }
        }

        pub fn real(&self) -> f64 {
            self.real
        }

        pub fn imag(&self) -> f64 {
            self.imag
        }

        pub fn from_real(value: f64) -> ComplexNumber {
            ComplexNumber {
                real: value,
                imag: 0.0,
            }
        }

        pub fn to_tuple(&self) -> (f64,f64) {
            (self.real,self.imag)
        }
    }

    impl Add<ComplexNumber> for ComplexNumber{
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            ComplexNumber {
                real: (self.real + rhs.real),
                imag: (self.imag + rhs.imag),
            }
        }
    }

    impl Add<f64> for ComplexNumber{
        type Output = Self;

        fn add(self, rhs: f64) -> Self::Output {
            ComplexNumber{
                real:(self.real+rhs),
                imag: self.imag
            }
        }
    }

    impl Add<&ComplexNumber> for ComplexNumber{
        type Output = Self;

        fn add(self, rhs: &ComplexNumber) -> Self::Output {
            ComplexNumber{
                real:self.real+rhs.real,
                imag:self.imag+rhs.imag,
            }
        }
    }

    impl Add<&ComplexNumber> for &ComplexNumber{
        type Output = ComplexNumber;

        fn add(self, rhs: &ComplexNumber) -> Self::Output {
            ComplexNumber{
                real:self.real+rhs.real,
                imag:self.imag+rhs.imag,
            }
        }
    }

    impl AddAssign for ComplexNumber{
        fn add_assign(&mut self, rhs: Self) {
            self.real += rhs.real;
            self.imag += rhs.imag;
        }
    }

    impl Default for ComplexNumber{
        fn default() -> Self {
            ComplexNumber{
                real: 0.0,
                imag: 0.0,
            }
        }
    }

    impl PartialEq for ComplexNumber {
        fn eq(&self, other: &Self) -> bool {
            if (self.real == other.real && self.imag == other.imag) {
                true
            } else {
                false
            }
        }
    }
    /*
    impl From<f64> for ComplexNumber{
        fn from(value: f64) -> Self {
            ComplexNumber{
                real: value,
                imag: 0.0,
            }
        }
    }
    
     impl Into<f64> for ComplexNumber{
        fn into(self) -> f64 {
            if(self.imag==0.0) {
                self.real
            } else {
                panic!("Conversion to real is only possible when Im = 0.0!")
            }
        }
    } */
    
    impl TryInto<f64> for ComplexNumber{
        type Error = &'static str;
        fn try_into(self) -> Result<f64, Self::Error> {
            if(self.imag == 0.0) {
                Ok(self.real)
            } else {
                Err("Conversion to real is only possible when Im = 0.0!")
            }
        }
    }

    impl Into<ComplexNumber> for f64{
        fn into(self) -> ComplexNumber {
            ComplexNumber {
                real: self,
                imag: 0.0,
            }
        }
    }
    
    impl Ord for ComplexNumber{
        fn cmp(&self, other: &Self) -> Ordering {
            let selfModule = (self.real.powf(2.0)+self.imag.powf(2.0)).sqrt();
            let otherModule = (other.real.powf(2.0)+other.imag.powf(2.0)).sqrt();
            if(selfModule < otherModule){
                Ordering::Less
            } else if (selfModule == otherModule) {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        }
    }

    impl PartialOrd for ComplexNumber {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Eq for ComplexNumber {}
    
    impl AsRef<f64> for ComplexNumber {
        fn as_ref(&self) -> &f64 {
            &self.real
        }
    }
    
    impl AsMut<f64> for ComplexNumber {
        fn as_mut(&mut self) -> &mut f64 {
            &mut self.real
        }
    }

    impl Hash for ComplexNumber {
        fn hash<H: Hasher>(&self, state: &mut H) {
            state.write_u64(self.real.to_bits());
            state.write_u64(self.imag.to_bits());
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
