use chrono::{Datelike, NaiveDate, NaiveTime, Weekday};
use notify_rust::Notification;
use serde::Deserialize;

#[derive(Debug)]
enum WeekType {
    A,
    B,
    C,
}

#[derive(Debug)]
struct Bell {
    bell: String,
    time: NaiveTime,
}

#[derive(Debug)]
struct Bells {
    status: String,
    bells_altered: bool,
    bells_altered_reason: String,
    bells: Vec<Bell>,
    date: NaiveDate,
    day: Weekday,
    term: u8,
    week: u8,
    week_type: WeekType,
}

impl From<BellsResponse> for Bells {
    fn from(response: BellsResponse) -> Self {
        let bells_list: Vec<_> = response
            .bells
            .into_iter()
            .map(|response_bell| Bell {
                bell: match &response_bell.bell[..] {
                    "1" => String::from("Period 1"),
                    "2" => String::from("Period 2"),
                    "3" => String::from("Period 3"),
                    "4" => String::from("Period 4"),
                    "5" => String::from("Period 5"),
                    _ => response_bell.bell,
                },
                time: NaiveTime::parse_from_str(&response_bell.time[..], "%H:%M").unwrap(),
            })
            .collect();
        Bells {
            status: response.status,
            bells_altered: response.bells_altered,
            bells_altered_reason: response.bells_altered_reason,
            bells: bells_list,
            date: NaiveDate::parse_from_str(&response.date[..], "%Y-%m-%d").unwrap(),
            day: match &response.day[..] {
                "Monday" => Weekday::Mon,
                "Tuesday" => Weekday::Tue,
                "Wednesday" => Weekday::Wed,
                "Thursday" => Weekday::Thu,
                "Friday" => Weekday::Fri,
                _ => panic!("The response weekday doesn't match any of the weekday patterns!"),
            },
            term: response.term.parse().unwrap(),
            week: response.week.parse().unwrap(),
            week_type: match &response.week_type[..] {
                "A" => WeekType::A,
                "B" => WeekType::B,
                "C" => WeekType::C,
                _ => panic!("The week type doesn't match any of the week type patterns!"),
            },
        }
    }
}

#[derive(Deserialize)]
struct ResponseBell {
    bell: String,
    time: String,
}

#[derive(Deserialize)]
struct BellsResponse {
    status: String,
    #[serde(rename = "bellsAltered")]
    bells_altered: bool,
    #[serde(rename = "bellsAlteredReason")]
    bells_altered_reason: String,
    bells: Vec<ResponseBell>,
    date: String,
    day: String,
    term: String,
    week: String,
    #[serde(rename = "weekType")]
    week_type: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Notification::new()
        .summary("RunToClass | Active")
        .body("RunToClass is active and will notify you of school belltimes.")
        .show()?;

    loop {
        let response = fetch_bells();
        let bells: Bells;
        match response {
            Ok(response) => bells = Bells::from(response),
            Err(_) => {
                Notification::new()
                    .summary("RunToClass | Offline Mode")
                    .body("RunToClass cannot connect to the SBHS Student Portal API and will use offline mode. Keep in mind that you will not be notified of changed bell times this way.")
                    .show()?;
                bells = get_offline_bells();
            }
        }
        println!("{:#?}", bells);

        if bells.bells_altered {
            Notification::new()
                .summary("RunToClass | Bells Changed")
                .body(
                    &(String::from("The bell times have been changed for today. Reason: ")
                        + &bells.bells_altered_reason[..])[..],
                )
                .show()?;
        }
        for bell in bells.bells {
            if chrono::Local::now().naive_local().time() > bell.time {
                continue;
            }
            loop {
                let now_time = chrono::Utc::now().naive_local().time();
                if !(now_time <= bell.time) {
                    continue;
                }
                Notification::new()
                    .summary(&bell.bell[..])
                    .body("This notification was sent by RunToClass.")
                    .show()?;
                break;
            }
        }
        loop {
            if chrono::Local::today().naive_local() <= bells.date {
                continue;
            }
            break;
        }
    }
}

fn fetch_bells() -> Result<BellsResponse, reqwest::Error> {
    let response: BellsResponse =
        reqwest::blocking::get("https://student.sbhs.net.au/api/timetable/bells.json")?.json()?;
    Ok(response)
}

fn get_offline_bells() -> Bells {
    let today = chrono::Local::today().naive_local();
    let bells = match today.weekday() {
        Weekday::Mon | Weekday::Tue => vec![
            Bell {
                bell: String::from("Warning Bell"),
                time: NaiveTime::from_hms(9, 0, 0),
            },
            Bell {
                bell: String::from("Period 1"),
                time: NaiveTime::from_hms(9, 5, 0),
            },
            Bell {
                bell: String::from("Transition"),
                time: NaiveTime::from_hms(10, 5, 0),
            },
            Bell {
                bell: String::from("Period 2"),
                time: NaiveTime::from_hms(10, 10, 0),
            },
            Bell {
                bell: String::from("Lunch 1"),
                time: NaiveTime::from_hms(11, 10, 0),
            },
            Bell {
                bell: String::from("Lunch 2"),
                time: NaiveTime::from_hms(11, 30, 0),
            },
            Bell {
                bell: String::from("Period 3"),
                time: NaiveTime::from_hms(11, 50, 0),
            },
            Bell {
                bell: String::from("Transition"),
                time: NaiveTime::from_hms(12, 50, 0),
            },
            Bell {
                bell: String::from("Period 4"),
                time: NaiveTime::from_hms(12, 55, 0),
            },
            Bell {
                bell: String::from("Recess"),
                time: NaiveTime::from_hms(13, 55, 0),
            },
            Bell {
                bell: String::from("Period 5"),
                time: NaiveTime::from_hms(14, 15, 0),
            },
            Bell {
                bell: String::from("End of Day"),
                time: NaiveTime::from_hms(15, 15, 0),
            },
        ],
        Weekday::Wed | Weekday::Thu => vec![
            Bell {
                bell: String::from("Warning Bell"),
                time: NaiveTime::from_hms(9, 0, 0),
            },
            Bell {
                bell: String::from("Period 1"),
                time: NaiveTime::from_hms(9, 5, 0),
            },
            Bell {
                bell: String::from("Transition"),
                time: NaiveTime::from_hms(10, 5, 0),
            },
            Bell {
                bell: String::from("Period 2"),
                time: NaiveTime::from_hms(10, 10, 0),
            },
            Bell {
                bell: String::from("Recess"),
                time: NaiveTime::from_hms(11, 10, 0),
            },
            Bell {
                bell: String::from("Period 3"),
                time: NaiveTime::from_hms(11, 30, 0),
            },
            Bell {
                bell: String::from("Lunch 1"),
                time: NaiveTime::from_hms(12, 30, 0),
            },
            Bell {
                bell: String::from("Lunch 2"),
                time: NaiveTime::from_hms(12, 50, 0),
            },
            Bell {
                bell: String::from("Period 4"),
                time: NaiveTime::from_hms(13, 10, 0),
            },
            Bell {
                bell: String::from("Transition"),
                time: NaiveTime::from_hms(14, 10, 0),
            },
            Bell {
                bell: String::from("Period 5"),
                time: NaiveTime::from_hms(14, 15, 0),
            },
            Bell {
                bell: String::from("End of Day"),
                time: NaiveTime::from_hms(15, 15, 0),
            },
        ],
        Weekday::Fri => vec![
            Bell {
                bell: String::from("Scripture"),
                time: NaiveTime::from_hms(8, 50, 0),
            },
            Bell {
                bell: String::from("Warning Bell"),
                time: NaiveTime::from_hms(9, 25, 0),
            },
            Bell {
                bell: String::from("Period 1"),
                time: NaiveTime::from_hms(9, 30, 0),
            },
            Bell {
                bell: String::from("Transition"),
                time: NaiveTime::from_hms(10, 25, 0),
            },
            Bell {
                bell: String::from("Period 2"),
                time: NaiveTime::from_hms(10, 30, 0),
            },
            Bell {
                bell: String::from("Recess"),
                time: NaiveTime::from_hms(11, 25, 0),
            },
            Bell {
                bell: String::from("Period 3"),
                time: NaiveTime::from_hms(11, 45, 0),
            },
            Bell {
                bell: String::from("Lunch 1"),
                time: NaiveTime::from_hms(12, 40, 0),
            },
            Bell {
                bell: String::from("Lunch 2"),
                time: NaiveTime::from_hms(13, 0, 0),
            },
            Bell {
                bell: String::from("Period 4"),
                time: NaiveTime::from_hms(13, 20, 0),
            },
            Bell {
                bell: String::from("Transition"),
                time: NaiveTime::from_hms(14, 15, 0),
            },
            Bell {
                bell: String::from("Period 5"),
                time: NaiveTime::from_hms(14, 20, 0),
            },
            Bell {
                bell: String::from("End of Day"),
                time: NaiveTime::from_hms(15, 15, 0),
            },
        ],
        _ => vec![],
    };
    Bells {
        status: String::from("OK"),
        bells_altered: false,
        bells_altered_reason: String::from(""),
        bells,
        date: today,
        day: today.weekday(),
        term: 1,
        week: 1,
        week_type: WeekType::A,
    }
}
