use anyhow::{Ok, Result, bail};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Default)]
pub struct HistoryList {
    pub histories: Vec<String>,
}

impl HistoryList {
    const FILE_PATH: &'static str = "conversion.json";

    pub fn load() -> Result<Self> {
        if !Path::new(Self::FILE_PATH).exists() {
            return Ok(Self::default());
        }

        let data = fs::read_to_string(Self::FILE_PATH)?;
        let list = serde_json::from_str(&data)?;
        Ok(list)
    }

    pub fn add(&mut self, desc_param: String) {
        self.histories.push(desc_param);
    }

    pub fn save(&self) -> Result<()> {
        let data = serde_json::to_string_pretty(&self)?;
        fs::write(Self::FILE_PATH, data)?;
        Ok(())
    }

    pub fn list(&self) {
        println!("Satuan yang di dukung:");
        println!("1. [suhu] celcius");
        println!("2. [suhu] fahrenheit");
        println!("3. [suhu] kelvin");
        println!("4. [panjang] cm");
        println!("5. [panjang] inch");
        println!("6. [panjang] km");
        println!("7. [panjang] miles");
    }

    pub fn print(&self) {
        if self.histories.is_empty() {
            println!("Belum ada history");
            return;
        }

        println!("Riwayat Konversi:");
        for (i, history) in self.histories.iter().enumerate() {
            println!("{}. {}", i + 1, history);
        }
    }

    pub fn calculate(&self, from: String, to: String, value: f32) -> Result<String> {
        let temp_and_panjang_list = vec![
            "celcius",
            "fahrenheit",
            "kelvin",
            "cm",
            "inch",
            "km",
            "miles",
        ];

        if from == "celcius" && to == "fahrenheit" {
            // [°F] = [°C] × 9⁄5 + 32 => 9⁄5 convert to decimal become 1.8
            let fahrenheit: f32 = value * 1.8 + 32.0;
            let result = format!("{:.2} °C = {} °F", value, fahrenheit);
            println!("{}", result);
            Ok(result)
        } else if from == "celcius" && to == "kelvin" {
            // [K] = [°C] + 273,15
            let kelvin: f32 = value + 273.15;
            let result = format!("{:.2} °C = {} °K", value, kelvin);
            println!("{}", result);
            Ok(result)
        } else if from == "fahrenheit" && to == "celcius" {
            // [°C] = ([°F] − 32) × 5⁄9 => 5⁄9  convert to decimal become 0.5
            let celcius: f32 = (value - 32.0) * 0.5;
            let result = format!("{:.2} °F = {} °C", value, celcius);
            println!("{}", result);
            Ok(result)
        } else if from == "fahrenheit" && to == "kelvin" {
            // [K] = ([°F] + 459,67) × 5⁄9
            let kelvin: f32 = (value + 459.67) * 0.5;
            let result = format!("{:.2} °F = {} °K", value, kelvin);
            println!("{}", result);
            Ok(result)
        } else if from == "kelvin" && to == "celcius" {
            // [°C] = [K] − 273,15
            let celcius: f32 = value - 273.15;
            let result = format!("{:.2} °K = {} °C", value, celcius);
            println!("{}", result);
            Ok(result)
        } else if from == "cm" && to == "inch" {
            // inch = cm / 2,54
            let inch: f32 = value / 2.54;
            let result = format!("{} cm = {} inch", value, inch);
            println!("{}", result);
            Ok(result)
        } else if from == "cm" && to == "km" {
            // km = cm / 100,000
            let km: f32 = value / 100_000_f32;
            let result = format!("{} cm = {} km", value, km);
            println!("{}", result);
            Ok(result)
        } else if from == "cm" && to == "miles" {
            // miles = cm / 160,935
            let miles: f32 = value / 160.935;
            let result = format!("{} cm = {} miles", value, miles);
            println!("{}", result);
            Ok(result)
        } else if from == "inch" && to == "cm" {
            // cm = inch * 2,54
            let cm: f32 = value * 2.54;
            let result = format!("{} inch = {} cm", value, cm);
            println!("{}", result);
            Ok(result)
        } else if from == "inch" && to == "km" {
            // km = inch * 0.0000254
            let km: f32 = value * 0.0000254;
            let result = format!("{} inch = {} km", value, km);
            println!("{}", result);
            Ok(result)
        } else if from == "inch" && to == "miles" {
            // miles = inch / 63,360
            let miles: f32 = value / 63.360;
            let result = format!("{} inch = {} miles", value, miles);
            println!("{}", result);
            Ok(result)
        } else if from == "km" && to == "cm" {
            // cm = km * 100,000
            let cm: f32 = value * 100_000_f32;
            let result = format!("{} km = {} cm", value, cm);
            println!("{}", result);
            Ok(result)
        } else if from == "km" && to == "inch" {
            // inch = km * 39,37
            let inch: f32 = value * 39.37;
            let result = format!("{} km = {} inch", value, inch);
            println!("{}", result);
            Ok(result)
        } else if from == "km" && to == "miles" {
            // miles = km * 0.621371
            let miles: f32 = value * 0.621371;
            let result = format!("{} km = {} miles", value, miles);
            println!("{}", result);
            Ok(result)
        } else if from == "miles" && to == "cm" {
            // cm = miles * 160,934
            let cm: f32 = value * 160.934;
            let result = format!("{} km = {} cm", value, cm);
            println!("{}", result);
            Ok(result)
        } else if from == "miles" && to == "inch" {
            // inch = miles * 63,360
            let inch: f32 = value * 63.360;
            let result = format!("{} km = {} inch", value, inch);
            println!("{}", result);
            Ok(result)
        } else if from == "miles" && to == "km" {
            // km = miles * 1,609344
            let km: f32 = value * 1.609344;
            let result = format!("{} km = {} km", value, km);
            println!("{}", result);
            Ok(result)
        } else {
            if !temp_and_panjang_list.contains(&from.as_str()) {
                bail!("[Error] Satuan asal '{}' tidak dikenai.", from)
            } else if !temp_and_panjang_list.contains(&to.as_str()) {
                bail!("[Error] Satuan tujuan '{}' tidak dikenai.", to)
            } else if from == to {
                let result = format!("{} {} = {} {}", value, from, value, to);
                println!("{}", result);
                Ok(result)
            } else {
                bail!(
                    "[ERROR] Tidak dapat mengonversi satuan yang berbeda kategori: [panjang] {} → [suhu] {}",
                    from,
                    to
                )
            }
        }
    }
}
