pub mod world {
    use super::objects::Object;

    #[derive(Debug)]
    pub struct Space {
        time: usize,
        objects: Vec<Object>,
    }

    impl Space {
        pub fn time(&self) -> usize {
            self.time
        }
    }

    impl std::default::Default for Space {
        fn default() -> Self {
            Self {
                time: 0,
                objects: Vec::new(),
            }
        }
    }
}

pub mod objects {
    #[derive(Debug)]
    pub enum Object {
        Ball,
    }

    #[derive(Debug)]
    pub struct Ball {
        width: f64,
        height: f64,
        coordinates: (f64, f64),
        mass: f64,
    }

    trait ObjectMethods {
        fn get_type_of_object(&self) -> &str {
            match self {
                Ball => "Ball",
                _ => "null",
            }
        }
    }

    impl Ball {
        pub(crate) fn coordinates(&self) -> (f64, f64) {
            self.coordinates
        }

        pub(crate) fn new() -> Ball {
            Self {
                width: 1.0,
                height: 1.0,
                coordinates: (0.0, 0.0),
                mass: 1.0,
            }
        }
    }

    impl ObjectMethods for Ball {}
}
