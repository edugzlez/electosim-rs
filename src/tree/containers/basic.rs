#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Region {
    name: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Candidacy {
    name: String,
}

impl Region {
    pub fn new(name: &str) -> Region {
        Region {
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Candidacy {
    pub fn new(name: &str) -> Candidacy {
        Candidacy {
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
