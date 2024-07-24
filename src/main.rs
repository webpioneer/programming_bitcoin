use std::fmt;
use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct FieldElement {
    num: i64,
    prime: i64,
}

#[derive(Debug)]
enum FieldElementError {
    DifferentFields,
    InvalidElement,
}

impl fmt::Display for FieldElementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FieldElementError::DifferentFields => write!(f, "Cannot operate on elements from different fields"),
            FieldElementError::InvalidElement => write!(f, "Element is not in valid field range"),
        }
    }
}

impl FieldElement {
    fn new(num: i64, prime: i64) -> Result<Self, FieldElementError> {
        if num >= prime || num < 0 {
            Err(FieldElementError::InvalidElement)
        } else {
            Ok(FieldElement { num, prime })
        }
    }

    fn pow(self, exponent: i64) -> FieldElement {
        let mut exp = exponent;
        let mut base = self.num;
        let mut result = 1;
        while exp > 0 {
            if exp % 2 == 1 {
                result = (result * base) % self.prime;
            }
            base = (base * base) % self.prime;
            exp /= 2;
        }
        FieldElement { num: result, prime: self.prime }
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FieldElement_{}({})", self.prime, self.num)
    }
}

impl Add for FieldElement {
    type Output = Result<FieldElement, FieldElementError>;

    fn add(self, other: FieldElement) -> Result<FieldElement, FieldElementError> {
        if self.prime != other.prime {
            Err(FieldElementError::DifferentFields)
        } else {
            let num = (self.num + other.num) % self.prime;
            Ok(FieldElement { num, prime: self.prime })
        }
    }
}

impl Sub for FieldElement {
    type Output = Result<FieldElement, FieldElementError>;

    fn sub(self, other: FieldElement) -> Result<FieldElement, FieldElementError> {
        if self.prime != other.prime {
            Err(FieldElementError::DifferentFields)
        } else {
            let num = (self.num - other.num) % self.prime;
            Ok(FieldElement { num: (num + self.prime) % self.prime, prime: self.prime })  // Ensuring positive result
        }
    }
}

impl Mul for FieldElement {
    type Output = Result<FieldElement, FieldElementError>;

    fn mul(self, other: FieldElement) -> Result<FieldElement, FieldElementError> {
        if self.prime != other.prime {
            Err(FieldElementError::DifferentFields)
        } else {
            let num = (self.num * other.num) % self.prime;
            Ok(FieldElement { num, prime: self.prime })
        }
    }
}

impl Div for FieldElement {
    type Output = Result<FieldElement, FieldElementError>;

    fn div(self, other: FieldElement) -> Result<FieldElement, FieldElementError> {
        if self.prime != other.prime {
            Err(FieldElementError::DifferentFields)
        } else {
            // Use Fermat's Little Theorem to find the multiplicative inverse:
            // a^(p-1) ≡ 1 (mod p) -> a^(p-2) ≡ a^(-1) (mod p)
            let num = (self.num * other.pow(self.prime - 2).num) % self.prime;
            Ok(FieldElement { num, prime: self.prime })
        }
    }
}

fn main() {
    let a = FieldElement::new(2, 19).unwrap();
    let b = FieldElement::new(7, 19).unwrap();

    // Addition
    println!("{}", (a + b).unwrap());  // Should print FieldElement_19(9)

    // Subtraction
    println!("{}", (a - b).unwrap());  // Should print FieldElement_19(14) because 2 - 7 ≡ -5 ≡ 14 (mod 19)

    // Multiplication
    println!("{}", (a * b).unwrap());  // Should print FieldElement_19(14) because 2 * 7 ≡ 14 (mod 19)

    // Division
    println!("{}", (a / b).unwrap());  // Should print FieldElement_19(3) because 2 / 7 ≡ 2 * 7^(-1) ≡ 2 * 11 ≡ 22 ≡ 3 (mod 19)

    // Exponentiation
    println!("{}", a.pow(3));  // Should print FieldElement_19(8) because 2^3 ≡ 8 (mod 19)
}
