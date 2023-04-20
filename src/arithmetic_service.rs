use actix::prelude::*;
use actix::Actor;
use derive_new::new;
use std::thread;
use std::time::Duration;

pub(crate) struct ArithmeticService;

#[derive(Message, new)]
#[rtype(result = "i64")]
pub(crate) struct Square {
    pub input: i64,
}

impl Actor for ArithmeticService {
    type Context = SyncContext<Self>;

    fn started(&mut self, _context: &mut Self::Context) {
        println!("ArithmeticService is running");
    }

    fn stopped(&mut self, _context: &mut Self::Context) {
        println!("ArithmeticService stopped");
    }
}

impl Handler<Square> for ArithmeticService {
    type Result = i64;

    fn handle(&mut self, message: Square, _context: &mut Self::Context) -> Self::Result {
        println!(
            "started processing Square({}), on {:?}",
            message.input,
            thread::current().id(),
        );

        let delay_in_seconds = message.input.try_into().unwrap();
        thread::sleep(Duration::from_secs(delay_in_seconds));

        println!("finished processing Square({})", message.input);
        message.input * message.input
    }
}
