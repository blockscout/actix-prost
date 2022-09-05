use prost_build::{Service, ServiceGenerator};

pub struct GeneratorList {
    generators: Vec<Box<dyn ServiceGenerator>>,
}

impl GeneratorList {
    pub fn new(generators: Vec<Box<dyn ServiceGenerator>>) -> GeneratorList {
        GeneratorList { generators }
    }
}

impl ServiceGenerator for GeneratorList {
    fn generate(&mut self, service: Service, buf: &mut String) {
        for gen in self.generators.iter_mut() {
            gen.generate(service.clone(), buf);
        }
    }

    fn finalize(&mut self, buf: &mut String) {
        for gen in self.generators.iter_mut() {
            gen.finalize(buf);
        }
    }

    fn finalize_package(&mut self, package: &str, buf: &mut String) {
        for gen in self.generators.iter_mut() {
            gen.finalize_package(package, buf);
        }
    }
}
