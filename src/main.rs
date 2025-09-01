use clap::{Arg, Command};
use sensors;

enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}
struct AppArgs {
    unit: TemperatureUnit,
    poll_interval: std::time::Duration,
    names: Option<Vec<String>>,
}

fn main() {
    let args = match AppArgs::new() {
        Some(args) => args,
        None => {
            eprintln!("Error: Invalid arguments");
            std::process::exit(1);
        }
    };

    loop {
        let temperature: Option<f64> = match get_average_temperature(&args.names) {
            Some(temperature) => match args.unit {
                TemperatureUnit::Celsius => Some(temperature),
                TemperatureUnit::Fahrenheit => Some(celsius_to_fahrenheit(temperature)),
                TemperatureUnit::Kelvin => Some(celsius_to_kelvin(temperature)),
            },
            None => None,
        };

        match temperature {
            Some(temperature) => print_average_temperature(temperature),
            None => eprintln!("Failed to read temperature"),
        };

        std::thread::sleep(args.poll_interval);
    }
}

impl AppArgs {
    fn new() -> Option<Self> {
        let matches = Command::new("yambar-temperature")
            .version("1.0.0")
            .about("Temperature module for Yambar")
            .arg(
                Arg::new("unit")
                    .long("unit")
                    .value_name("UNIT")
                    .help("Unit of temperature value to display")
                    .value_parser(["celsius", "fahrenheit", "kelvin"])
                    .default_value("celsius"),
            )
            .arg(
                Arg::new("poll-interval")
                    .long("poll-interval")
                    .value_name("POLL_INTERVAL")
                    .help("Interval between updates in milliseconds")
                    .value_parser(clap::value_parser!(u64))
                    .default_value("1000"),
            )
            .arg(
                Arg::new("names")
                    .long("names")
                    .value_name("NAMES")
                    .help(
                        "Names of sensors included in temperature calculation. If not specified, all sensors will be used.",
                    )
                    .num_args(1..),
            )
            .get_matches();

        let unit = match matches.get_one::<String>("unit")?.to_string().as_str() {
            "celsius" => TemperatureUnit::Celsius,
            "fahrenheit" => TemperatureUnit::Fahrenheit,
            "kelvin" => TemperatureUnit::Kelvin,
            _ => unreachable!(),
        };

        let poll_interval = matches.get_one::<u64>("poll-interval")?;

        let names = match matches.get_many::<String>("names") {
            Some(names) => Some(names.map(|s| s.to_string()).collect()),
            None => None,
        };

        Some(AppArgs {
            unit: unit,
            poll_interval: std::time::Duration::from_millis(*poll_interval),
            names: names,
        })
    }
}

fn filter_chips(chip: &sensors::Chip, names: &Option<Vec<String>>) -> bool {
    let name = chip.get_name().ok();
    match name {
        Some(name) => match names {
            Some(names) => names.contains(&name),
            None => true,
        },
        None => false,
    }
}

fn filter_features(feature: &sensors::Feature) -> bool {
    *feature.feature_type() == sensors::FeatureType::SENSORS_FEATURE_TEMP
}

fn filter_subfeatures(subfeature: &sensors::Subfeature) -> bool {
    *subfeature.subfeature_type() == sensors::SubfeatureType::SENSORS_SUBFEATURE_TEMP_INPUT
}

fn get_average_temperature(filter_names: &Option<Vec<String>>) -> Option<f64> {
   let (sum, count) = sensors::Sensors::new()
        .into_iter()
        .filter(|chip| filter_chips(chip, filter_names))
        .flat_map(|chip| chip.into_iter())
        .filter(filter_features)
        .flat_map(|feature| feature.into_iter())
        .filter(filter_subfeatures)
        .filter_map(|subfeature| subfeature.get_value().ok())
        .fold((0.0, 0usize), |(sum, count), v| (sum + v, count + 1));

    if count == 0 {
        None
    } else {
        Some(sum / count as f64)
    }
}

fn print_average_temperature(temperature: f64) {
    println!("temperature|float|{}", temperature);
    println!();
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn celsius_to_kelvin(celsius: f64) -> f64 {
    celsius + 273.15
}
