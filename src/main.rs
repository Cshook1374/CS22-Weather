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
    mod weather_module;
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

    print!("{:?}", wea);}
//