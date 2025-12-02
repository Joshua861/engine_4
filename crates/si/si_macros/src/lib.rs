use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Ident;

#[proc_macro]
pub fn gen_types(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let sizes = [
        ("exa", "E", 1_000_000_000_000_000_000.0f64),
        ("peta", "P", 1_000_000_000_000_000.0),
        ("tera", "T", 1_000_000_000_000.0),
        ("giga", "G", 1_000_000_000.0),
        ("mega", "M", 1_000_000.0),
        ("kilo", "K", 1000.0),
        ("hecto", "H", 100.0),
        ("deka", "D", 10.0),
        ("", "", 1.0),
        ("deci", "d", 0.1),
        ("centi", "c", 0.01),
        ("milli", "m", 0.001),
        ("micro", "u", 0.000_001),
        ("nano", "n", 0.000_000_001),
        ("pico", "p", 0.000_000_000_001),
        ("femto", "f", 0.000_000_000_000_001),
        ("atto", "a", 0.000_000_000_000_000_001),
    ];

    let units = [
        ("Length", "meters", "m"),
        ("Mass", "grams", "g"),
        ("Time", "seconds", "s"),
        ("Temperature", "kelvin", "K"),
        ("Current", "amperes", "A"),
        ("LuminousIntensity", "candelas", "cd"),
        ("AmountOfSubstance", "moles", "mol"),
        ("Area", "meters_squared", "m^2"),
        ("Volume", "meters_cubed", "m^3"),
        ("Speed", "meters_per_second", "m/s"),
        ("Acceleration", "meters_per_second_squared", "m/s^2"),
        ("Force", "newtons", "N"),
        ("Pressure", "pascals", "Pa"),
        ("Energy", "joules", "J"),
        ("Power", "watts", "W"),
        ("Momentum", "newton_seconds", "N·s"),
        ("Torque", "newton_meters", "N·m"),
        ("Impulse", "newton_seconds", "N·s"),
        ("Charge", "coulombs", "C"),
        ("Voltage", "volts", "V"),
        ("Resistance", "ohms", "Ω"),
        ("Capacitance", "farads", "F"),
        ("Inductance", "henries", "H"),
        ("MagneticFlux", "webers", "Wb"),
        ("MagneticFluxDensity", "teslas", "T"),
        ("Frequency", "hertz", "Hz"),
        ("Angle", "radians", "rad"),
        ("AngularVelocity", "radians_per_second", "rad/s"),
        (
            "AngularAcceleration",
            "radians_per_second_squared",
            "rad/s^2",
        ),
        ("Density", "kilograms_per_meter_cubed", "kg/m^3"),
        ("SpecificVolume", "meters_cubed_per_kilogram", "m^3/kg"),
    ];

    let physical_ops = [
        ("Length", "Length", "mul", "Area", 1.0),
        ("Area", "Length", "mul", "Volume", 1.0),
        ("Length", "Area", "mul", "Volume", 1.0),
        ("Length", "Time", "div", "Speed", 1.0),
        ("Speed", "Time", "div", "Acceleration", 1.0),
        ("Acceleration", "Time", "mul", "Speed", 1.0),
        ("Speed", "Time", "mul", "Length", 1.0),
        ("Angle", "Time", "div", "AngularVelocity", 1.0),
        ("AngularVelocity", "Time", "div", "AngularAcceleration", 1.0),
        ("AngularVelocity", "Time", "mul", "Angle", 1.0),
        ("Mass", "Acceleration", "mul", "Force", 0.001),
        ("Acceleration", "Mass", "mul", "Force", 0.001),
        ("Force", "Mass", "div", "Acceleration", 1000.0),
        ("Force", "Acceleration", "div", "Mass", 1000.0),
        ("Mass", "Speed", "mul", "Momentum", 0.001),
        ("Speed", "Mass", "mul", "Momentum", 0.001),
        ("Momentum", "Mass", "div", "Speed", 1000.0),
        ("Momentum", "Speed", "div", "Mass", 1000.0),
        ("Force", "Length", "mul", "Energy", 1.0),
        ("Length", "Force", "mul", "Torque", 1.0),
        ("Energy", "Time", "div", "Power", 1.0),
        ("Power", "Time", "mul", "Energy", 1.0),
        ("Force", "Speed", "mul", "Power", 1.0),
        ("Speed", "Force", "mul", "Power", 1.0),
        ("Power", "Speed", "div", "Force", 1.0),
        ("Power", "Force", "div", "Speed", 1.0),
        ("Force", "Time", "mul", "Impulse", 1.0),
        ("Time", "Force", "mul", "Impulse", 1.0),
        ("Force", "Area", "div", "Pressure", 1.0),
        ("Pressure", "Area", "mul", "Force", 1.0),
        ("Energy", "Volume", "div", "Pressure", 1.0),
        ("Mass", "Volume", "div", "Density", 0.001),
        ("Density", "Volume", "mul", "Mass", 1000.0),
        ("Volume", "Mass", "div", "SpecificVolume", 1000.0),
        ("Current", "Time", "mul", "Charge", 1.0),
        ("Charge", "Time", "div", "Current", 1.0),
        ("Energy", "Charge", "div", "Voltage", 1.0),
        ("Voltage", "Charge", "mul", "Energy", 1.0),
        ("Power", "Current", "div", "Voltage", 1.0),
        ("Voltage", "Current", "mul", "Power", 1.0),
        ("Current", "Voltage", "mul", "Power", 1.0),
        ("Voltage", "Current", "div", "Resistance", 1.0),
        ("Current", "Resistance", "mul", "Voltage", 1.0),
        ("Voltage", "Resistance", "div", "Current", 1.0),
        ("Charge", "Voltage", "div", "Capacitance", 1.0),
        ("Capacitance", "Voltage", "mul", "Charge", 1.0),
        ("MagneticFlux", "Current", "div", "Inductance", 1.0),
        ("MagneticFlux", "Area", "div", "MagneticFluxDensity", 1.0),
        ("MagneticFluxDensity", "Area", "mul", "MagneticFlux", 1.0),
    ];

    let mut output = quote! {};
    let mut cross_type_impls = quote! {};

    for (unit_type, unit_name, unit_shorthand) in units {
        let unit_type_ident = Ident::new(&unit_type, Span::call_site());

        let mut constructors = quote! {};
        let mut converters = quote! {};

        let uses_kilograms = unit_name.contains("kilogram");

        for (size_name, size_shorthand, scale_factor) in sizes.iter() {
            let constructor_name;
            let converter_name;
            let unit_full;
            let unit_symbol;

            if uses_kilograms {
                if size_name.is_empty() {
                    constructor_name = unit_name.to_string();
                    unit_full = unit_name.to_string();
                    unit_symbol = unit_shorthand.to_string();
                } else if *size_name == "kilo" {
                    continue;
                } else {
                    let base_with_grams = unit_name.replace("kilogram", "gram");
                    constructor_name = if size_name.is_empty() {
                        base_with_grams.clone()
                    } else {
                        format!("{}{}", size_name, base_with_grams)
                    };

                    unit_full = constructor_name.clone();

                    let adjusted_scale = scale_factor / 1000.0;
                    let adjusted_shorthand =
                        format!("{}{}", size_shorthand, unit_shorthand.replace("kg", "g"));
                    unit_symbol = adjusted_shorthand;

                    let constructor_ident = Ident::new(&constructor_name, Span::call_site());
                    let converter_name_str = format!("to_{}", constructor_name);
                    let converter_ident = Ident::new(&converter_name_str, Span::call_site());

                    constructors = quote! {
                        #constructors

                        #[doc = concat!("Create a new ", #unit_type, " from ", #unit_full, " (", #unit_symbol, ")")]
                        pub fn #constructor_ident(value: f64) -> Self {
                            Self(value * #adjusted_scale)
                        }
                    };

                    converters = quote! {
                        #converters

                        #[doc = concat!("Convert this ", #unit_type, " to ", #unit_full, " (", #unit_symbol, ")")]
                        pub fn #converter_ident(&self) -> f64 {
                            self.0 / #adjusted_scale
                        }
                    };
                    continue;
                }
            } else {
                constructor_name = if size_name.is_empty() {
                    format!("{}", unit_name)
                } else {
                    format!("{}{}", size_name, unit_name)
                };
                unit_full = constructor_name.clone();
                unit_symbol = format!("{}{}", size_shorthand, unit_shorthand);
            }

            let constructor_ident = Ident::new(&constructor_name, Span::call_site());
            converter_name = format!("to_{}", constructor_name);
            let converter_ident = Ident::new(&converter_name, Span::call_site());

            constructors = quote! {
                #constructors

                #[doc = concat!("Create a new ", #unit_type, " from ", #unit_full, " (", #unit_symbol, ")")]
                pub fn #constructor_ident(value: f64) -> Self {
                    Self(value * #scale_factor)
                }
            };

            converters = quote! {
                #converters

                #[doc = concat!("Convert this ", #unit_type, " to ", #unit_full, " (", #unit_symbol, ")")]
                pub fn #converter_ident(&self) -> f64 {
                    self.0 / #scale_factor
                }
            };
        }

        let mod_name = Ident::new(&unit_type.to_lowercase(), Span::call_site());

        output = quote! {
            #output

            pub use #mod_name::#unit_type_ident;

            mod #mod_name {
                #[doc = concat!(#unit_type, " quantity stored in base units")]
                #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
                pub struct #unit_type_ident(pub(crate) f64);

                impl #unit_type_ident {
                    #constructors
                    #converters

                    pub fn value(&self) -> f64 {
                        self.0
                    }
                }

                impl std::ops::Add for #unit_type_ident {
                    type Output = Self;

                    fn add(self, other: Self) -> Self::Output {
                        Self(self.0 + other.0)
                    }
                }

                impl std::ops::Sub for #unit_type_ident {
                    type Output = Self;

                    fn sub(self, other: Self) -> Self::Output {
                        Self(self.0 - other.0)
                    }
                }

                impl std::ops::Mul<f64> for #unit_type_ident {
                    type Output = Self;

                    fn mul(self, scalar: f64) -> Self::Output {
                        Self(self.0 * scalar)
                    }
                }

                impl std::ops::Div<f64> for #unit_type_ident {
                    type Output = Self;

                    fn div(self, scalar: f64) -> Self::Output {
                        Self(self.0 / scalar)
                    }
                }

                impl std::ops::AddAssign for #unit_type_ident {
                    fn add_assign(&mut self, other: Self) {
                        self.0 += other.0;
                    }
                }

                impl std::ops::SubAssign for #unit_type_ident {
                    fn sub_assign(&mut self, other: Self) {
                        self.0 -= other.0;
                    }
                }

                impl std::ops::MulAssign<f64> for #unit_type_ident {
                    fn mul_assign(&mut self, scalar: f64) {
                        self.0 *= scalar;
                    }
                }

                impl std::ops::DivAssign<f64> for #unit_type_ident {
                    fn div_assign(&mut self, scalar: f64) {
                        self.0 /= scalar;
                    }
                }


                use std::ops::Deref;
                impl Deref for #unit_type_ident {
                    type Target = f64;

                    fn deref(&self) -> &Self::Target {
                        &self.0
                    }
                }


                impl From<f64> for #unit_type_ident {
                    fn from(value: f64) -> Self {
                        Self(value)
                    }
                }

                impl Into<f64> for #unit_type_ident {
                    fn into(self) -> f64 {
                        self.0
                    }
                }

                use std::fmt::Display;
                impl Display for #unit_type_ident {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "{}{}", self.0, #unit_shorthand)
                    }
                }
            }
        }
    }

    for (type1, type2, operation, result_type, scale_adjustment) in physical_ops {
        let type1_ident = Ident::new(type1, Span::call_site());
        let type2_ident = Ident::new(type2, Span::call_site());
        let result_ident = Ident::new(result_type, Span::call_site());

        match operation {
            "mul" => {
                cross_type_impls = quote! {
                    #cross_type_impls

                    impl std::ops::Mul<#type2_ident> for #type1_ident {
                        type Output = #result_ident;

                        fn mul(self, other: #type2_ident) -> Self::Output {
                            #result_ident(self.0 * other.0 * #scale_adjustment)
                        }
                    }
                };
            }
            "div" => {
                cross_type_impls = quote! {
                    #cross_type_impls

                    impl std::ops::Div<#type2_ident> for #type1_ident {
                        type Output = #result_ident;

                        fn div(self, other: #type2_ident) -> Self::Output {
                            #result_ident(self.0 / other.0 * #scale_adjustment)
                        }
                    }
                };
            }
            _ => {}
        }
    }

    quote! {
        #output
        #cross_type_impls
    }
    .into()
}
