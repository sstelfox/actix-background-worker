extern crate actix;
extern crate actix_web;
extern crate env_logger;

#[macro_use]
extern crate log;

use actix::prelude::*;
use actix_web::{http, middleware, server, App, HttpRequest};
use std::{thread, time};

#[derive(Message)]
struct BackgroundTask;

#[derive(Default)]
struct BackgroundTaskActor;

impl Actor for BackgroundTaskActor {
    type Context = SyncContext<Self>;

    fn started(&mut self, _: &mut SyncContext<Self>) {
        info!("Background task actor started up")
    }
}

impl Handler<BackgroundTask> for BackgroundTaskActor {
    type Result = ();

    fn handle(&mut self, _: BackgroundTask, _: &mut SyncContext<Self>) {
        info!("Starting background task");
        thread::sleep(time::Duration::new(4,0));
        info!("Finished background task");
    }
}

fn index(req: HttpRequest<AppState>) -> &'static str {
    req.state().background_actor.do_send(BackgroundTask {});

    "Background task started\n"
}

struct AppState {
    background_actor: Addr<Syn, BackgroundTaskActor>,
}

fn main() {
    ::std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let sys = actix::System::new("background-worker-example");
    let bt_actor = SyncArbiter::start(1, move || BackgroundTaskActor::default());

    server::new(move ||
            App::with_state(AppState { background_actor: bt_actor.clone() })
                .middleware(middleware::Logger::default())
                .resource("/", |r| r.method(http::Method::GET).with(index))
        )
        .bind("127.0.0.1:8000")
        .unwrap()
        .start();

    info!("Starting up server on 127.0.0.1:8000");
    sys.run();
}
