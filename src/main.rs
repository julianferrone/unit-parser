use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

#[derive(PartialEq, Eq)]
struct PhysicalUnit {
    time: isize,
    length: isize,
    mass: isize,
    current: isize,
    temperature: isize,
    amount_of_substance: isize,
    luminous_intensity: isize,
}

impl PhysicalUnit {
    fn new(
        time: isize,
        length: isize,
        mass: isize,
        current: isize,
        temperature: isize,
        amount_of_substance: isize,
        luminous_intensity: isize,
    ) -> Self {
        Self {
            time,
            length,
            mass,
            current,
            temperature,
            amount_of_substance,
            luminous_intensity,
        }
    }
}

impl Display for PhysicalUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (
            self.time,
            self.length,
            self.mass,
            self.current,
            self.temperature,
            self.amount_of_substance,
            self.luminous_intensity,
        ) {
            ( 1,  0,  0,  0,  0,  0,  0) => write!(f, "Unit(Time)"), // Second (s)
            ( 0,  1,  0,  0,  0,  0,  0) => write!(f, "Unit(Length)"), // Metre (m)
            ( 0,  0,  1,  0,  0,  0,  0) => write!(f, "Unit(Mass)"), // Kilogram (kg)
            ( 0,  0,  0,  1,  0,  0,  0) => write!(f, "Unit(Current)"), // Ampere (A)
            ( 0,  0,  0,  0,  1,  0,  0) => write!(f, "Unit(Temperature)"), //  Kelvin (K)
            ( 0,  0,  0,  0,  0,  1,  0) => write!(f, "Unit(AmountOfSubstance)"), // Mole (mol)
            ( 0,  0,  0,  0,  0,  0,  1) => write!(f, "Unit(LuminousIntensity)"), // Candela (cd)
            (-1,  0,  0,  0,  0,  0,  0) => write!(f, "Unit(Frequency)"), // Hertz (Hz) = second^-1
            (-2,  1,  1,  0,  0,  0,  0) => write!(f, "Unit(Force)"), // Newton (N)= kg * m * s^-2
            (-2, -1,  1,  0,  0,  0,  0) => write!(f, "Unit(Pressure)"), // Pascal (Pa)= N / m^-2 = kg * m^-1 * s^-2
            (-2,  2,  1,  0,  0,  0,  0) => write!(f, "Unit(Energy)"), // Joule (J) = N * m = kg * m^2 * s^-2
            (-3,  2,  1,  0,  0,  0,  0) => write!(f, "Unit(Power)"), // Power
            ( 1,  0,  0,  1,  0,  0,  0) => write!(f, "Unit(ElectricCharge)"), //
            (-2, -1,  1, -1,  0,  0,  0) => write!(f, "Unit(ElectricPotential)"), //
            (-1, -1,  1, -1,  0,  0,  0) => write!(f, "Unit(ElectricPotential)"), // Weber = volt * second
            _ => write!(
                f,
                "Unit(Time^{}, Length^{}, Mass^{}, Current^{}, Temperature^{}, AmountOfSubstance^{}, LuminousIntensity^{})",
                self.time,
                self.length,
                self.mass,
                self.current,
                self.temperature,
                self.amount_of_substance,
                self.luminous_intensity
            ),
        }
    }
}

impl Mul for PhysicalUnit {
    type Output = PhysicalUnit;

    fn mul(self, rhs: Self) -> Self::Output {
        let product = PhysicalUnit::new(
            self.time + rhs.time,
            self.length + rhs.length,
            self.mass + rhs.mass,
            self.current + rhs.current,
            self.temperature + rhs.temperature,
            self.amount_of_substance + rhs.amount_of_substance,
            self.luminous_intensity + rhs.luminous_intensity,
        );
        product
    }
}

impl Div for PhysicalUnit {
    type Output = PhysicalUnit;

    fn div(self, rhs: Self) -> Self::Output {
        let quotient = PhysicalUnit::new(
            self.time - rhs.time,
            self.length - rhs.length,
            self.mass - rhs.mass,
            self.current - rhs.current,
            self.temperature - rhs.temperature,
            self.amount_of_substance - rhs.amount_of_substance,
            self.luminous_intensity - rhs.luminous_intensity,
        );
        quotient
    }
}

struct ConcreteNumber {
    quantity: f64,
    unit: PhysicalUnit,
}

impl ConcreteNumber {
    fn new(quantity: f64, unit: PhysicalUnit) -> Self {
        Self { quantity, unit }
    }
}

enum CustomError {
    AddingTwoDifferentUnits,
}

impl Add for ConcreteNumber {
    type Output = Result<ConcreteNumber, crate::CustomError>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.unit != rhs.unit {
            Err(CustomError::AddingTwoDifferentUnits)
        } else {
            let quantity: f64 = self.quantity + rhs.quantity;
            let sum: ConcreteNumber = ConcreteNumber::new(quantity, self.unit);
            Ok(sum)
        }
    }
}

impl Sub for ConcreteNumber {
    type Output = Result<ConcreteNumber, crate::CustomError>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.unit != rhs.unit {
            Err(CustomError::AddingTwoDifferentUnits)
        } else {
            let quantity: f64 = self.quantity - rhs.quantity;
            let difference: ConcreteNumber = ConcreteNumber::new(quantity, self.unit);
            Ok(difference)
        }
    }
}

impl Mul for ConcreteNumber {
    type Output = ConcreteNumber;

    fn mul(self, rhs: Self) -> Self::Output {
        let quantity: f64 = self.quantity * rhs.quantity;
        let unit: PhysicalUnit = self.unit * rhs.unit;
        let product: ConcreteNumber = ConcreteNumber::new(quantity, unit);
        product
    }
}

impl Div for ConcreteNumber {
    type Output = ConcreteNumber;

    fn div(self, rhs: Self) -> Self::Output {
        let quantity: f64 = self.quantity / rhs.quantity;
        let unit: PhysicalUnit = self.unit / rhs.unit;
        let quotient: ConcreteNumber = ConcreteNumber::new(quantity, unit);
        quotient
    }
}

fn main() {
    println!("Hello, world!");
}
