// Module Containing Behavior and Tests for Weather

#[derive(Debug, PartialEq, Clone)]
pub struct Weather {
    temperature: i32,
    conditions: String,
    wind_speed: i32,
}

impl Weather {
    pub fn new(temperature: i32, conditions: String, wind_speed: i32) -> Self {
        return Weather {
            temperature,
            conditions,
            wind_speed,
        };
    }

    pub fn _get_temperature(&self) -> i32 {
        return self.temperature;
    }

    pub fn _get_conditions(&self) -> String {
        return self.conditions.to_string();
    }

    pub fn _get_wind_speed(&self) -> i32 {
        return self.wind_speed;
    }

    pub fn _set_temperature(&mut self, temp: i32) -> () {
        let mut temp = temp;
        if temp < -70 || temp > 150 {
            temp = 70;
        }
        self.temperature = temp;
    }

    pub fn _set_conditions(&mut self, cond: String) -> () {
        let mut cond = cond;
        if cond.eq("") {
            cond = String::from("Cloudy");
        }
        self.conditions = cond;
    }

    pub fn _set_wind_speed(&mut self, wind: i32) -> () {
        let mut wind = wind;
        if wind < 0 {
            wind = 0;
        }
        self.wind_speed = wind;
    }

    pub fn _to_string(&self) -> String {
        return format!("{:?}", self);
    }

    pub fn _equals(&self, other: &Self) -> bool {
        return self == other;
    }
}

#[test]
fn set_temperature_upper() -> () {
    let mut wea = Weather::new(0, String::from(""), 0);
    let mut test: bool = true;
    wea._set_temperature(200);
    if wea.temperature > 150 {
        test = false;
    }
    assert!(test);
}
#[test]
fn set_temperature_lower() -> () {
    let mut wea = Weather::new(0, String::from(""), 0);
    let mut test: bool = true;
    wea._set_temperature(-100);
    if wea.temperature < -70 {
        test = false;
    }
    assert!(test);
}
#[test]
fn get_temperature() -> () {
    let wea = Weather::new(132, String::from(""), 0);
    let mut test: bool = true;
    if wea._get_temperature() != 132 {
        test = false;
    }
    assert!(test);
}

#[test]
fn set_conditions() -> () {
    let mut wea = Weather::new(0, String::from(""), 0);
    let mut test: bool = true;
    wea._set_conditions(String::from("Sunny"));
    if wea.conditions.ne("Sunny") {
        test = false;
    }
    assert!(test);
}

#[test]
fn get_conditions() -> () {
    let wea = Weather::new(0, String::from("Sunny"), 0);
    let mut test: bool = true;
    if wea._get_conditions() != String::from("Sunny") {
        test = false;
    }
    assert!(test);
}

#[test]
fn set_wind_speed() -> () {
    let mut wea = Weather::new(0, String::from(""), 0);
    let mut test: bool = true;
    wea._set_wind_speed(-100);
    if wea.wind_speed != 0 {
        test = false;
    }
    assert!(test);
}

#[test]
fn get_wind_speed() -> () {
    let wea = Weather::new(0, String::from(""), 132);
    let mut test = true;
    if wea._get_wind_speed() != 132 {
        test = false;
    }
    assert!(test);
}