/*struct Temperature {
    degrees_f: f64,
}

impl Temperature {
    fn show_temp(temp: &Temperature) {
        println!("{:?} degrees F", temp.degrees_f);
    }
}

fn main() {
    let hot = Temperature {
        degrees_f : 99.9
    };

    Temperature::show_temp(&hot);
}
*/

struct Temperature {
    degrees_f: f64,
}

impl Temperature {
    fn freezing() -> Self {
        Self {
            degrees_f : 32.0,
        }
    }

    fn boiling() -> Self {
        Self {
            degrees_f : 212.0,
        }
    }

    fn show_temp(&self) {
        println!("{:?} degrees F", self.degrees_f);
    }
}

fn main() {
    let hot = Temperature {
        degrees_f : 99.9
    };
    let cold = Temperature::freezing();
    let boil = Temperature::boiling();
    hot.show_temp();
    cold.show_temp();
    boil.show_temp();
}
