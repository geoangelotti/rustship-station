use rand::random;
use rand::thread_rng;
use rand::Rng;
use rand_derive2::RandGen;
use std::str::FromStr;
use strum_macros::Display;
use strum_macros::EnumString;

#[derive(Debug, RandGen, Display)]
enum Name {
    Akira,
    Californa,
    Daedalus,
    Eisenberg,
    Intrepid,
    Miranda,
    Nova,
    Reliant,
    Sagan,
}

#[derive(Debug, RandGen)]
pub struct Station {
    name: Name,
    version: u8,
    sections: Vec<Section>,
}

impl Station {
    pub fn new() -> Self {
        Station {
            name: random(),
            version: random(),
            sections: (0..10).map(|_| random()).collect(),
        }
    }
}

impl Station {
    pub fn days_left(&self) -> usize {
        self.sections.iter().filter(|m| m.active).count()
    }
}

#[derive(Debug, RandGen, Eq, PartialEq)]
struct Section {
    name: SectionName,
    active: bool,
}

#[derive(Debug, RandGen, Display, EnumString, Eq, PartialEq)]
enum SectionName {
    AstroScience,
    Solar,
    Antenna,
    RadiationMirrors,
    Sleeping,
    NuclearGenerator,
    Galley,
    Transponder,
    Tracking,
}

impl Station {
    pub fn working_sections(&self) -> Vec<String> {
        self.sections
            .iter()
            .filter(|m| m.active)
            .map(|m| m.name.to_string())
            .collect()
    }

    pub fn broken_sections(&self) -> Vec<String> {
        self.sections
            .iter()
            .filter(|m| !m.active)
            .map(|m| m.name.to_string())
            .collect()
    }
}

// fixes `broken_section` on a `station`
pub fn repair(broken_section: String, station: &mut Station) {
    let section = SectionName::from_str(broken_section.as_str()).unwrap();

    let broken_index = station
        .sections
        .iter()
        .position(|m| m.name == section)
        .expect("Section not found.");

    station.sections[broken_index].active = true;
}

pub fn science(working_section: String, station: &mut Station) {
    station.break_something();
}

impl Station {
    pub fn new_day(&mut self) {
        self.break_something();
    }

    fn break_something(&mut self) {
        let broken_index = thread_rng().gen_range(0..self.sections.len());
        let mut broken_section = &mut self.sections[broken_index];
        if broken_section.active {
            broken_section.active = false;
            println!("(Section-FAILURE {})", &broken_section.name);
        } else {
            println!("(sections OK)");
        }
    }

    pub fn status(&self) {
        dbg!(&self);
    }
}
