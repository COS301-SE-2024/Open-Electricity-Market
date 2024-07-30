#[macro_use]
extern crate rocket;

use crate::grid::circuit::Circuit;
use crate::grid::generator::Generator;
use crate::grid::load::Connection::{Parallel, Series};
use crate::grid::load::{Consumer, Load, LoadType};
use crate::grid::location::Location;
use crate::grid::{Grid, OscilloscopeDetail, Resistance, Voltage, VoltageWrapper};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Method, Status};
use rocket::response::content;
use rocket::serde::json::json;
use rocket::{Request, Response, State};
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub struct CORS;
pub mod grid;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if request.method() == Method::Options {
            response.set_status(Status::NoContent);
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, PATCH, GET, DELETE",
            ));
            response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        }

        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}








#[get("/set_generator")]
fn echo_channel(ws: ws::WebSocket, grid: &State<Arc<Mutex<Grid>>>) -> ws::Channel<'_> {
    use rocket::futures::{SinkExt, StreamExt};
    let a = grid;

    ws.channel(move |mut stream| {
        Box::pin(async move {
            while let Some(message) = stream.next().await {
                let _ = stream.send(ws::Message::Text("Recieved".to_string())).await;
                let mut b = a.lock().unwrap();
                b.set_generator(message.unwrap().to_string());
            }

            Ok(())
        })
    })
}

#[get("/")]
fn index() -> String {
    "Yay".to_string()
}


#[post("/stats")]
fn stats(grid : &State<Arc<Mutex<Grid>>>) -> content::RawJson<String> {
  let g = grid.lock().unwrap();
  let stats = g.get_grid_stats(); 
  let stats = serde_json::to_string(&stats).unwrap();
  content::RawJson(stats)
}



#[post("/info", format = "application/json")]
fn info(grid: &State<Arc<Mutex<Grid>>>) -> content::RawJson<String> {
    let g = grid.lock().unwrap();
    let info = serde_json::to_string(g.deref()).unwrap();
    content::RawJson(info)
}

#[post("/start", format = "application/json")]
fn start(grid: &State<Arc<Mutex<Grid>>>) -> String {
    let mut g = grid.lock().unwrap();
    if !g.started {
        g.started = true;
        let clone = grid.inner().clone();
        tokio::spawn(async move {
            let mut start = Instant::now();
            let mut elapsed_time = 0.0;
            loop {
                let duration = start.elapsed();
                elapsed_time += duration.as_secs_f32();
                start = Instant::now();
                let mut grid = clone.lock().unwrap();
                grid.update(elapsed_time)
            }
        });
        json!({
            "Message": "Started Grid"
        })
        .to_string()
    } else {
        json!({
            "Message": "Grid Already Running"
        })
        .to_string()
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .mount("/", routes![index, start, info, echo_channel,stats])
        .manage(Arc::new(Mutex::new(Grid {
            circuits: vec![Circuit {
                id: 0,
                loads: vec![
                    Load {
                        load_type: LoadType::Consumer(Consumer {
                            id: 0,
                            resistance: Resistance(10.0),
                            voltage: VoltageWrapper {
                                voltage: Voltage(0.0, 0.0, 0.0),
                                oscilloscope_detail: OscilloscopeDetail {
                                    frequency: 0.0,
                                    amplitude: 0.0,
                                    phase: 0.0,
                                },
                            },
                            location: Location {
                                latitude: -26.2044,
                                longitude: 28.0248,
                            },
                        }),
                        id: 0,
                    },
                    Load {
                        load_type: LoadType::Consumer(Consumer {
                            id: 1,
                            resistance: Resistance(15.0),
                            voltage: VoltageWrapper {
                                voltage: Voltage(0.0, 0.0, 0.0),
                                oscilloscope_detail: OscilloscopeDetail {
                                    frequency: 0.0,
                                    amplitude: 0.0,
                                    phase: 0.0,
                                },
                            },
                            location: Location {
                                latitude: -26.1735,
                                longitude: 27.9985,
                            },
                        }),
                        id: 1,
                    },
                    Load {
                        load_type: LoadType::Consumer(Consumer {
                            id: 2,
                            resistance: Resistance(30.0),
                            voltage: VoltageWrapper {
                                voltage: Voltage(0.0, 0.0, 0.0),
                                oscilloscope_detail: OscilloscopeDetail {
                                    frequency: 0.0,
                                    amplitude: 0.0,
                                    phase: 0.0,
                                },
                            },
                            location: Location {
                                latitude: -26.2015,
                                longitude: 28.0336,
                            },
                        }),
                        id: 2,
                    },
                    Load {
                        load_type: LoadType::Consumer(Consumer {
                            id: 3,
                            resistance: Resistance(10.0),
                            voltage: VoltageWrapper {
                                voltage: Voltage(0.0, 0.0, 0.0),
                                oscilloscope_detail: OscilloscopeDetail {
                                    frequency: 0.0,
                                    amplitude: 0.0,
                                    phase: 0.0,
                                },
                            },
                            location: Location {
                                latitude: -26.1886,
                                longitude: 28.0401,
                            },
                        }),
                        id: 3,
                    },
                    Load {
                        load_type: LoadType::Consumer(Consumer {
                            id: 4,
                            resistance: Resistance(18.0),
                            voltage: VoltageWrapper {
                                voltage: Voltage(0.0, 0.0, 0.0),
                                oscilloscope_detail: OscilloscopeDetail {
                                    frequency: 0.0,
                                    amplitude: 0.0,
                                    phase: 0.0,
                                },
                            },
                            location: Location {
                                latitude: -26.2026,
                                longitude: 28.0473,
                            },
                        }),
                        id: 4,
                    },
                    Load {
                        load_type: LoadType::Consumer(Consumer {
                            id: 5,
                            resistance: Resistance(15.0),
                            voltage: VoltageWrapper {
                                voltage: Voltage(0.0, 0.0, 0.0),
                                oscilloscope_detail: OscilloscopeDetail {
                                    frequency: 0.0,
                                    amplitude: 0.0,
                                    phase: 0.0,
                                },
                            },
                            location: Location {
                                latitude: -26.2348,
                                longitude: 28.0139,
                            },
                        }),
                        id: 5,
                    },
                    Load {
                        load_type: LoadType::Consumer(Consumer {
                            id: 6,
                            resistance: Resistance(17.0),
                            voltage: VoltageWrapper {
                                voltage: Voltage(0.0, 0.0, 0.0),
                                oscilloscope_detail: OscilloscopeDetail {
                                    frequency: 0.0,
                                    amplitude: 0.0,
                                    phase: 0.0,
                                },
                            },
                            location: Location {
                                latitude: -26.1980,
                                longitude: 28.0469,
                            },
                        }),
                        id: 6,
                    },
                    Load {
                        load_type: LoadType::Consumer(Consumer {
                            id: 7,
                            resistance: Resistance(25.0),
                            voltage: VoltageWrapper {
                                voltage: Voltage(0.0, 0.0, 0.0),
                                oscilloscope_detail: OscilloscopeDetail {
                                    frequency: 0.0,
                                    amplitude: 0.0,
                                    phase: 0.0,
                                },
                            },
                            location: Location {
                                latitude: -26.2068,
                                longitude: 28.0452,
                            },
                        }),
                        id: 7,
                    },
                    Load {
                        load_type: LoadType::Consumer(Consumer {
                            id: 8,
                            resistance: Resistance(10.0),
                            voltage: VoltageWrapper {
                                voltage: Voltage(0.0, 0.0, 0.0),
                                oscilloscope_detail: OscilloscopeDetail {
                                    frequency: 0.0,
                                    amplitude: 0.0,
                                    phase: 0.0,
                                },
                            },
                            location: Location {
                                latitude: -26.1593,
                                longitude: 28.0302,
                            },
                        }),
                        id: 8,
                    },
                    Load {
                        load_type: LoadType::Consumer(Consumer {
                            id: 9,
                            resistance: Resistance(11.0),
                            voltage: VoltageWrapper {
                                voltage: Voltage(0.0, 0.0, 0.0),
                                oscilloscope_detail: OscilloscopeDetail {
                                    frequency: 0.0,
                                    amplitude: 0.0,
                                    phase: 0.0,
                                },
                            },
                            location: Location {
                                latitude: -26.2369,
                                longitude: 28.0116,
                            },
                        }),
                        id: 9,
                    },
                ],
                connections: vec![
                    Parallel(0, 1),
                    Series(0, 2),
                    Series(0, 3),
                    Parallel(0, 4),
                    Series(4, 5),
                    Parallel(0, 7),
                    Series(7, 8),
                    Series(7, 9),
                ],
                generators: vec![Generator {
                    id: 0,
                    voltage: VoltageWrapper {
                        voltage: Voltage(0.0, 0.0, 0.0),
                        oscilloscope_detail: OscilloscopeDetail {
                            frequency: 0.0,
                            amplitude: 0.0,
                            phase: 0.0,
                        },
                    },
                    max_voltage: 240.0,
                    frequency: 50.0,
                    transmission_line: 0,
                    location: Location {
                        latitude: 0.0,
                        longitude: 0.0,
                    },
                }],
                transformers: vec![],
            }],
            frequency: 50.0,
            started: false,
        })))
}
