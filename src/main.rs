use std::io;
fn main() {
    println!("Convert Temperature from temperature to another temperature");
    let mut base_temp = String::new();
    let mut temperature_value = String::new();
    println!("Enter the base temperature type (c/celsius),(f/fahrenheit) or (k/kelvin) ");
    match io::stdin().read_line(&mut base_temp) {
        Ok(_) => {
            println!("Enter Temperature: ");
            match io::stdin().read_line(&mut temperature_value) {
                Ok(_) => {
                    let temperature_value: f32 = temperature_value
                        .trim()
                        .parse()
                        .expect("Please type a number!");
                    // transform base_temp to lowercase
                    let base_temp = base_temp.to_string().to_lowercase();

                    match base_temp.trim() {
                        "c" | "celsius" => convert_temp("celsius", temperature_value),
                        "f" | "fahrenheit" => convert_temp("fahrenheit", temperature_value),
                        "k" | "kelvin" => convert_temp("kelvin", temperature_value),
                        _ => println!("Please enter a valid temperature type!"),
                    }
                }
                Err(e) => println!("Error: {}", e),
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}

fn convert_temp(base_temp: &str, temperature_value: f32) {
    match base_temp.trim() {
        "celsius" => {
            let celsius = temperature_value;
            let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
            let kelvin = celsius + 273.15;

            println!("{}°C = {}°F", celsius, fahrenheit);
            println!("{}°C = {}°K", celsius, kelvin);
        }
        "fahrenheit" => {
            let fahrenheit = temperature_value;
            let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
            let kelvin = (fahrenheit + 459.67) * 5.0 / 9.0;
            println!("{}°F = {}°C", fahrenheit, celsius);
            println!("{}°F = {}°K", fahrenheit, kelvin);
        }
        "kelvin" => {
            let kelvin = temperature_value;
            let celsius = kelvin - 273.15;
            let fahrenheit = (kelvin * 9.0 / 5.0) - 459.67;
            println!("{}°K = {}°C", kelvin, celsius);
            println!("{}°K = {}°F", kelvin, fahrenheit);
        }
        _ => println!("Please enter a valid temperature type!"),
    }
}
