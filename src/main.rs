// Prompt
/*

Write a Java class encapsulating the concept of weather conditions. The class should include temperature, conditions
(sunny, cloudy, snow, rain), and wind speed. Include constructor, accessors(getters), mutators(setters), toString and
equals. Temperature, in Fahrenheit, should be between -70 and 150, with a default of 70. Wind speed should be
between 0 and 150 mph, with a default of zero. Default condition is cloudy. Also, include a method which checks
whether the condition and temperature are consistent (if temperature is above 40, it cannot be snowing; if the
temperature is below 30 it cannot be raining).
Write a client main method which tests all the methods of the weather class, and shows all the restrictions on the values
are followed.
Submit printed copies of all formatted code, printed copies of the test results including documentation of what are
shown by the test cases.
*/
//

// External Includes
use text_io::read;
//

// Local Modules
mod weather_module {
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
}
use weather_module::Weather;
//

// Functions
fn main() -> () {
    let temp: i32 = read!();
    let cond: String = read!();
    let wind: i32 = read!();

    let mut wea = Weather::new(0, String::from(""), 0);

    wea._set_temperature(temp);
    wea._set_conditions(cond);
    wea._set_wind_speed(wind);

    print!("{:?}", wea);
}
//