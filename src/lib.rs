use thiserror::Error;
use std::error;
use std::fmt;
use std::ops;

#[derive(Error, Debug)]
pub enum FiniteFieldError {
    #[error("The number {0} is not in the finite field")]
    NotInFiniteField(u32),
}

#[derive(Debug, PartialEq, Eq)]
pub struct FieldElement {
    pub(crate) num: u32,
    pub(crate) prime: u32,
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FieldElement_{}({})", self.num, self.prime)
    }
}

impl ops::Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot add two elements of different fields");
        }
        Self {
            num: (self.num + other.num).rem_euclid(self.prime),
            prime: self.prime,
        }
    }
}

impl FieldElement {
    pub fn new(num: u32, prime: u32) -> Result<Self, impl error::Error> {
        if num >= prime {
            Err(FiniteFieldError::NotInFiniteField(num))
        } else {
            Ok(FieldElement { num, prime })
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_field_element() {
        let FieldElement {num, prime} = FieldElement::new(1, 13).unwrap();
        assert_eq!(num, 1);
        assert_eq!(prime, 13);
        match FieldElement::new(13, 13) {
            Ok(_) => panic!("Should not be able to create a field element with a number greater than the prime"),
            Err(_) => { assert!(true); }
        }
    }

    #[test]
    fn field_element_equality() {
        let e1 = FieldElement::new(1, 13).unwrap();
        let e2 = FieldElement::new(1, 13).unwrap();
        assert!(e1 == e2);
        let e3 = FieldElement::new(1, 27).unwrap();
        assert!(e3 != e2);
    }

    #[test]
    fn field_element_addition() {
        let p = 13;
        let e1 = FieldElement::new(1, p).unwrap();
        let e2 = FieldElement::new(5, p).unwrap();
        let expect = FieldElement::new((1+5 as u32).rem_euclid(p), p).unwrap();
        assert_eq!(e1 + e2, expect);
    }

    #[test]
    #[should_panic]
    fn field_element_addition_of_different_order() {
        let e1 = FieldElement::new(1, 13).unwrap();
        let e2 = FieldElement::new(5, 27).unwrap();
        let _ = e1 + e2;
    }
}
