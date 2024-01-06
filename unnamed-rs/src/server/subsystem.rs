use std::{future::Future, time::Duration};

use crate::{subsystem::Handle, subsystem_periodic};

pub struct ExampleSubsystem {
    cond: bool,
    string: String,
}

impl ExampleSubsystem {
    pub fn new() -> Handle<Self> {
        let (handle, inner) = Handle::new_and_inner(Self {
            cond: false,
            string: "hi".into(),
        });
        subsystem_periodic!(inner, periodic);
        handle
    }

    pub async fn do_the_thing(&mut self) {
        println!("YOOOO");
    }

    pub async fn example_condition(&self) -> bool {
        return self.cond;
    }

    pub async fn set_condition(&mut self, v: bool) {
        self.cond = v;
    }

    pub async fn periodic(&mut self) -> impl Future<Output = ()> + Send + 'static {
        println!("example perodic");
        tokio::time::sleep(Duration::from_millis(100))
    }
}

/*impl Subsystem for ExampleSubsystem {
    type PeriodicTimer = impl Future<Output = ()> + Send + 'static;

    async fn periodic(&mut self) -> Self::PeriodicTimer {
        println!("example perodic");
        async { }
        //return tokio::time::sleep(Duration::from_millis(100));
    }
}*/
