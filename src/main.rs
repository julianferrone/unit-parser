mod parser;

use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Sub},
};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct PhysicalQuantity {
    time: isize,
    length: isize,
    mass: isize,
    current: isize,
    temperature: isize,
    amount_of_substance: isize,
    luminous_intensity: isize,
}

impl PhysicalQuantity {
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

    fn dimensions(self) -> String {
        match (
            self.time,
            self.length,
            self.mass,
            self.current,
            self.temperature,
            self.amount_of_substance,
            self.luminous_intensity,
        ) {
            (0, 0, 0, 0, 0, 0, 0) => "Dimension(Dimensionless)".to_owned(),
            (1, 0, 0, 0, 0, 0, 0) => "Dimension(Time)".to_owned(), // Second (s)
            (0, 1, 0, 0, 0, 0, 0) => "Dimension(Length)".to_owned(), // Metre (m)
            (0, 0, 1, 0, 0, 0, 0) => "Dimension(Mass)".to_owned(), // Kilogram (kg)
            (0, 0, 0, 1, 0, 0, 0) => "Dimension(Current)".to_owned(), // Ampere (A)
            (0, 0, 0, 0, 1, 0, 0) => "Dimension(Temperature)".to_owned(), //  Kelvin (K)
            (0, 0, 0, 0, 0, 1, 0) => "Dimension(AmountOfSubstance)".to_owned(), // Mole (mol)
            (0, 0, 0, 0, 0, 0, 1) => "Dimension(LuminousIntensity)".to_owned(), // Candela (cd)
            (-1, 0, 0, 0, 0, 0, 0) => "Dimension(Frequency)".to_owned(), // Hertz (Hz) = second^-1
            (-2, 1, 1, 0, 0, 0, 0) => "Dimension(Force)".to_owned(), // Newton (N)= kg * m * s^-2
            (-2, -1, 1, 0, 0, 0, 0) => "Dimension(Pressure)".to_owned(), // Pascal (Pa)= N * m^-2 = kg * m^-1 * s^-2
            (-2, 2, 1, 0, 0, 0, 0) => "Dimension(Energy)".to_owned(), // Joule (J) = N * m = kg * m^2 * s^-2
            (-3, 2, 1, 0, 0, 0, 0) => "Dimension(Power)".to_owned(), // Power (W) = J * s^-1 = kg * m^2 * s^-3
            (1, 0, 0, 1, 0, 0, 0) => "Dimension(ElectricCharge)".to_owned(), // Coulomb (C) = A * s
            (-3, 2, 1, -1, 0, 0, 0) => "Dimension(ElectricPotential)".to_owned(), // Volt (V) = J * C^-1 = kg * m^2 * s^-3 * A^-1
            (-2, 2, 1, -1, 0, 0, 0) => "Dimension(MagneticFlux)".to_owned(), // Weber (Wb) = V * s = kg * m^2 * s^-2 * A^-1
            (-2, 0, 1, -1, 0, 0, 0) => "Dimension(MagneticFluxDensity)".to_owned(), // Tesla (T) = Wb * m^-2 = kg * s^-2 * A^-1
            (4, -2, -1, 2, 0, 0, 0) => "Dimension(ElectricalCapacitance)".to_owned(), // Farad (F) = kg^-1 * m^-2 * s^4 * A^2
            (-3, 2, 1, -2, 0, 0, 0) => "Dimension(ElectricalResistance)".to_owned(), // Ohm (Ω) = kg * m^2 * s^−3 * A^−2
            (3, -2, -1, 2, 0, 0, 0) => "Dimension(ElectricalConductance)".to_owned(), // Siemens (S) = kg^−1 * m^−2 * s^3 * A^2
            (-2, 2, 1, -2, 0, 0, 0) => "Dimension(ElectricalInductance)".to_owned(), // Henry (H) = kg * m^2 * s^−2 * A^−2
            (1, 0, 0, 0, 0, 1, 0) => "Dimension(CatalyticActivity)".to_owned(), // Katal (kat) = mol * s^-1
            (0, 2, 0, 0, 0, 0, 0) => "Dimension(Area)".to_owned(),
            (0, 3, 0, 0, 0, 0, 0) => "Dimension(Volume)".to_owned(),
            (-1, 1, 0, 0, 0, 0, 0) => "Dimension(Speed)".to_owned(),
            (-2, 1, 0, 0, 0, 0, 0) => "Dimension(Acceleration)".to_owned(),
            (-1, 3, 0, 0, 0, 0, 0) => "Dimension(VolumetricFlow)".to_owned(),
            (-1, 1, 1, 0, 0, 0, 0) => "Dimension(Momentum)".to_owned(),
            _ => {
                let mut units: Vec<(&str, isize)> = vec![];
                if self.time != 0 {
                    units.push(("Time", self.time));
                }
                if self.length != 0 {
                    units.push(("Length", self.length));
                }
                if self.mass != 0 {
                    units.push(("Mass", self.mass));
                }
                if self.current != 0 {
                    units.push(("Current", self.current));
                }
                if self.temperature != 0 {
                    units.push(("Temperature", self.temperature));
                }
                if self.amount_of_substance != 0 {
                    units.push(("AmountOfSubstance", self.amount_of_substance));
                }
                if self.luminous_intensity != 0 {
                    units.push(("LuminousIntensity", self.luminous_intensity));
                }
                units.sort_by(|a, b| a.0.cmp(b.0));
                let concatenated: Vec<String> = units
                    .into_iter()
                    .map(|(unit, exponent)| match exponent {
                        1 => format!("{unit}"),
                        _ => format!("{unit}^{exponent}"),
                    })
                    .collect();
                concatenated.join(" ")
            }
        }
    }
}

impl Display for PhysicalQuantity {
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
            (1, 0, 0, 0, 0, 0, 0) => write!(f, "s"),    // Second (s)
            (0, 1, 0, 0, 0, 0, 0) => write!(f, "m"),    // Metre (m)
            (0, 0, 1, 0, 0, 0, 0) => write!(f, "kg"),   // Kilogram (kg)
            (0, 0, 0, 1, 0, 0, 0) => write!(f, "A"),    // Ampere (A)
            (0, 0, 0, 0, 1, 0, 0) => write!(f, "K"),    //  Kelvin (K)
            (0, 0, 0, 0, 0, 1, 0) => write!(f, "mol"),  // Mole (mol)
            (0, 0, 0, 0, 0, 0, 1) => write!(f, "cd"),   // Candela (cd)
            (-1, 0, 0, 0, 0, 0, 0) => write!(f, "Hz"),  // Hertz (Hz) = second^-1
            (-2, 1, 1, 0, 0, 0, 0) => write!(f, "N"),   // Newton (N)= kg * m * s^-2
            (-2, -1, 1, 0, 0, 0, 0) => write!(f, "Pa"), // Pascal (Pa)= N * m^-2 = kg * m^-1 * s^-2
            (-2, 2, 1, 0, 0, 0, 0) => write!(f, "J"),   // Joule (J) = N * m = kg * m^2 * s^-2
            (-3, 2, 1, 0, 0, 0, 0) => write!(f, "W"),   // Power (W) = J * s^-1 = kg * m^2 * s^-3
            (1, 0, 0, 1, 0, 0, 0) => write!(f, "C"),    // Coulomb (C) = A * s
            (-3, 2, 1, -1, 0, 0, 0) => write!(f, "V"), // Volt (V) = J * C^-1 = kg * m^2 * s^-3 * A^-1
            (-2, 2, 1, -1, 0, 0, 0) => write!(f, "Wb"), // Weber (Wb) = V * s = kg * m^2 * s^-2 * A^-1
            (-2, 0, 1, -1, 0, 0, 0) => write!(f, "T"),  // Tesla (T) = Wb * m^-2 = kg * s^-2 * A^-1
            (4, -2, -1, 2, 0, 0, 0) => write!(f, "F"),  // Farad (F) = kg^-1 * m^-2 * s^4 * A^2
            (-3, 2, 1, -2, 0, 0, 0) => write!(f, "Ω"),  // Ohm (Ω) = kg * m^2 * s^−3 * A^−2
            (3, -2, -1, 2, 0, 0, 0) => write!(f, "S"),  // Siemens (S) = kg^−1 * m^−2 * s^3 * A^2
            (-2, 2, 1, -2, 0, 0, 0) => write!(f, "H"),  // Henry (H) = kg * m^2 * s^−2 * A^−2
            (1, 0, 0, 0, 0, 1, 0) => write!(f, "kat"),  // Katal (kat) = mol * s^-1
            (0, 0, 0, 0, 0, 0, 0) => write!(f, "dimensionless"), // dimension-less
            _ => {
                let mut units: Vec<(&str, isize)> = vec![];
                if self.time != 0 {
                    units.push(("s", self.time));
                }
                if self.length != 0 {
                    units.push(("m", self.length));
                }
                if self.mass != 0 {
                    units.push(("kg", self.mass));
                }
                if self.current != 0 {
                    units.push(("A", self.current));
                }
                if self.temperature != 0 {
                    units.push(("K", self.temperature));
                }
                if self.amount_of_substance != 0 {
                    units.push(("mol", self.amount_of_substance));
                }
                if self.luminous_intensity != 0 {
                    units.push(("cd", self.luminous_intensity));
                }
                units.sort_by(|a, b| a.0.cmp(b.0));
                let concatenated: Vec<String> = units
                    .into_iter()
                    .map(|(unit, exponent)| match exponent {
                        1 => format!("{unit}"),
                        _ => format!("{unit}^{exponent}"),
                    })
                    .collect();
                let compound_unit: String = concatenated.join(" ");
                write!(f, "{}", compound_unit)
            }
        }
    }
}

impl Mul for PhysicalQuantity {
    type Output = PhysicalQuantity;

    fn mul(self, rhs: Self) -> Self::Output {
        let product = PhysicalQuantity::new(
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

impl Div for PhysicalQuantity {
    type Output = PhysicalQuantity;

    fn div(self, rhs: Self) -> Self::Output {
        let quotient = PhysicalQuantity::new(
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

struct PhysicalQuantityBuilder {
    time: isize,
    length: isize,
    mass: isize,
    current: isize,
    temperature: isize,
    amount_of_substance: isize,
    luminous_intensity: isize,
}

impl PhysicalQuantityBuilder {
    fn new() -> Self {
        PhysicalQuantityBuilder {
            time: 0,
            length: 0,
            mass: 0,
            current: 0,
            temperature: 0,
            amount_of_substance: 0,
            luminous_intensity: 0,
        }
    }

    fn time(mut self, time: isize) -> Self {
        self.time = time;
        self
    }

    fn length(mut self, length: isize) -> Self {
        self.length = length;
        self
    }

    fn mass(mut self, mass: isize) -> Self {
        self.mass = mass;
        self
    }

    fn current(mut self, current: isize) -> Self {
        self.current = current;
        self
    }

    fn temperature(mut self, temperature: isize) -> Self {
        self.temperature = temperature;
        self
    }

    fn amount_of_substance(mut self, amount_of_substance: isize) -> Self {
        self.amount_of_substance = amount_of_substance;
        self
    }

    fn luminous_intensity(mut self, luminous_intensity: isize) -> Self {
        self.luminous_intensity = luminous_intensity;
        self
    }

    fn build(self) -> PhysicalQuantity {
        PhysicalQuantity {
            time: self.time,
            length: self.length,
            mass: self.mass,
            current: self.current,
            temperature: self.temperature,
            amount_of_substance: self.amount_of_substance,
            luminous_intensity: self.luminous_intensity,
        }
    }
}

impl Mul for PhysicalQuantityBuilder {
    type Output = PhysicalQuantityBuilder;

    fn mul(self, rhs: Self) -> Self::Output {
        let product: PhysicalQuantityBuilder = PhysicalQuantityBuilder::new();
        product
            .time(self.time + rhs.time)
            .length(self.length + rhs.length)
            .mass(self.mass + rhs.mass)
            .current(self.current + rhs.current)
            .temperature(self.temperature + rhs.temperature)
            .amount_of_substance(self.amount_of_substance + rhs.amount_of_substance)
            .luminous_intensity(self.luminous_intensity + rhs.luminous_intensity)
    }
}

impl Div for PhysicalQuantityBuilder {
    type Output = PhysicalQuantityBuilder;

    fn div(self, rhs: Self) -> Self::Output {
        let quotient: PhysicalQuantityBuilder = PhysicalQuantityBuilder::new();
        quotient
            .time(self.time + rhs.time)
            .length(self.length + rhs.length)
            .mass(self.mass + rhs.mass)
            .current(self.current + rhs.current)
            .temperature(self.temperature + rhs.temperature)
            .amount_of_substance(self.amount_of_substance + rhs.amount_of_substance)
            .luminous_intensity(self.luminous_intensity + rhs.luminous_intensity)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConcreteNumber {
    magnitude: f64,
    physical_quantity: PhysicalQuantity,
}

impl ConcreteNumber {
    fn new(quantity: f64, unit: PhysicalQuantity) -> Self {
        Self {
            magnitude: quantity,
            physical_quantity: unit,
        }
    }
}

impl Display for ConcreteNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.magnitude, self.physical_quantity)
    }
}

struct ConcreteNumberBuilder {
    magnitude: f64,
    physical_quantity: PhysicalQuantity,
}

impl ConcreteNumberBuilder {
    fn new() -> Self {
        Self {
            magnitude: 0f64,
            physical_quantity: PhysicalQuantityBuilder::new().build(),
        }
    }

    fn magnitude(mut self, magnitude: f64) -> Self {
        self.magnitude = magnitude;
        self
    }

    fn physical_quantity(mut self, physical_quantity: PhysicalQuantity) -> Self {
        self.physical_quantity = physical_quantity;
        self
    }

    fn build(self) -> ConcreteNumber {
        ConcreteNumber {
            magnitude: self.magnitude,
            physical_quantity: self.physical_quantity,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CustomError {
    AddingTwoDifferentUnits,
    SubtractingTwoDifferentUnits,
    SubExpressionError,
    ParseError(String),
}

impl Add for ConcreteNumber {
    type Output = Result<ConcreteNumber, crate::CustomError>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.physical_quantity != rhs.physical_quantity {
            Err(CustomError::AddingTwoDifferentUnits)
        } else {
            let quantity: f64 = self.magnitude + rhs.magnitude;
            let sum: ConcreteNumber = ConcreteNumber::new(quantity, self.physical_quantity);
            Ok(sum)
        }
    }
}

impl Sub for ConcreteNumber {
    type Output = Result<ConcreteNumber, crate::CustomError>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.physical_quantity != rhs.physical_quantity {
            Err(CustomError::SubtractingTwoDifferentUnits)
        } else {
            let quantity: f64 = self.magnitude - rhs.magnitude;
            let difference: ConcreteNumber = ConcreteNumber::new(quantity, self.physical_quantity);
            Ok(difference)
        }
    }
}

impl Mul for ConcreteNumber {
    type Output = ConcreteNumber;

    fn mul(self, rhs: Self) -> Self::Output {
        let quantity: f64 = self.magnitude * rhs.magnitude;
        let unit: PhysicalQuantity = self.physical_quantity * rhs.physical_quantity;
        let product: ConcreteNumber = ConcreteNumber::new(quantity, unit);
        product
    }
}

impl Div for ConcreteNumber {
    type Output = ConcreteNumber;

    fn div(self, rhs: Self) -> Self::Output {
        let quantity: f64 = self.magnitude / rhs.magnitude;
        let unit: PhysicalQuantity = self.physical_quantity / rhs.physical_quantity;
        let quotient: ConcreteNumber = ConcreteNumber::new(quantity, unit);
        quotient
    }
}

impl From<f64> for ConcreteNumber {
    fn from(value: f64) -> Self {
        ConcreteNumberBuilder::new().magnitude(value).build()
    }
}

fn main() {
    let input: String = std::env::args().nth(1).expect("No input provided");
    let result = parser::evaluate_physical_equation(input.as_str());
    if result.is_ok() {
        println!("Input: \"{}\" => result: \"{}\"", input, result.unwrap());
    } else {
        println!("ERROR: Input \"{}\" failed: {:?}", input, result.unwrap_err());
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser, ConcreteNumber, PhysicalQuantity, PhysicalQuantityBuilder};

    #[test]
    fn build_metre() {
        let length_unit: PhysicalQuantity = PhysicalQuantityBuilder::new().length(1).build();
        let length: ConcreteNumber = ConcreteNumber::new(13.0, length_unit);
        assert_eq!(format!("{}", length), "13 m")
    }

    #[test]
    fn build_time() {
        let time_unit: PhysicalQuantity = PhysicalQuantityBuilder::new().time(1).build();
        let time: ConcreteNumber = ConcreteNumber::new(2.0, time_unit);
        assert_eq!(format!("{}", time), "2 s");
    }

    #[test]
    fn build_acceleration() {
        let length_unit: PhysicalQuantity = PhysicalQuantityBuilder::new().length(1).build();
        let length: ConcreteNumber = ConcreteNumber::new(13.0, length_unit);
        let time_unit: PhysicalQuantity = PhysicalQuantityBuilder::new().time(1).build();
        let time: ConcreteNumber = ConcreteNumber::new(2.0, time_unit);
        let acceleration = length / (time * time.clone());
        assert_eq!(format!("{}", acceleration), "3.25 m s^-2");
    }

    #[test]
    fn parse_time_and_print() {
        let time: &str = "3 s";
        let concrete_time = parser::evaluate_physical_equation(time).unwrap();
        assert_eq!(format!("{}", concrete_time), time);
    }

    #[test]
    fn explicit_and_implicit_unit_multiplication_should_match() {
        let cn_1 = "3 W m^3";
        let cn_2 = "3 W * 1 m * 1 m^2";
        assert_eq!(
            parser::evaluate_physical_equation(cn_1),
            parser::evaluate_physical_equation(cn_2)
        );
    }
}
