#[derive(Debug)]
pub struct Duct {
    radius: f32,
}

impl Duct {
    pub fn new(radius: f32) -> Duct {
        Duct { radius }
    }
}

#[derive(Debug)]
pub struct Chamber {
    radius: f32,
    length: f32,
}

impl Chamber {
    pub fn new(radius: f32, length: f32) -> Chamber {
        Chamber { radius, length }
    }
}

#[derive(Debug)]
pub struct ExpansionChamber {
    duct: Duct,
    chamber: Chamber,
}

impl ExpansionChamber {
    pub fn new(duct_radius: f32, chamber_radius: f32, chamber_length: f32) -> ExpansionChamber {
        let duct = Duct::new(duct_radius);

        let chamber = Chamber::new(chamber_radius, chamber_length);

        ExpansionChamber { duct, chamber }
    }

    pub fn transmission_loss(&self, frequency: f32) -> f32 {
        let speed_of_sound = 331.5;
        let duct_area = self.duct.radius.powi(2) * std::f32::consts::PI;
        let chamber_area = self.chamber.radius.powi(2) * std::f32::consts::PI;
        let wave_number = 2.0 * std::f32::consts::PI * frequency / speed_of_sound;
        let h = duct_area / chamber_area;

        (1.0 + 0.24 * (h - 1.0 / h).powi(2) * ((wave_number * self.chamber.length).sin().powi(2)))
            .log(10.0)
            * 10.0
    }
}

#[test]
fn chamber_transmission_loss_0hz() {
    let expansion_chamber = ExpansionChamber::new(1.0, 2.0, 2.0);

    assert_eq!(expansion_chamber.transmission_loss(0.0), 0.0)
}
