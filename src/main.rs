extern crate actix;
extern crate actix_web;

use actix::prelude::*;
use actix_web::{App, HttpRequest, server};

#[derive(Message)]
struct BackgroundTask;

#[derive(Default)]
struct BackgroundTaskActor;

impl Actor for BackgroundTaskActor {
    type Context = Context<Self>;
}

impl actix::Supervised for BackgroundTaskActor {}

impl SystemService for BackgroundTaskActor {
    fn service_started(&mut self, _: &mut Context<Self>) {
        println!("Background task actor started up")
    }
}

impl Handler<BackgroundTask> for BackgroundTaskActor {
    type Result = ();

    fn handle(&mut self, _: BackgroundTask, _: &mut Context<Self>) {
        println!("Ran background task")
    }
}

fn index(_req: HttpRequest) -> &'static str {
    let background_task = System::current().registry().get::<BackgroundTaskActor>();
    background_task.do_send(BackgroundTask {});

    "Minimal response"
}

fn main() {
    let sys = actix::System::new("background-worker-example");

    BackgroundTaskActor.start();

    let srv = server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind("127.0.0.1:8000")
        .unwrap();

    // Normally I had this chained off the previous call, pulled it off to
    // isolate this exception:
    //
    //  thread 'main' panicked at 'System is not running'
    srv.start();

    sys.run();
}
