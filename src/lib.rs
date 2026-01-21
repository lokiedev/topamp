use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
struct PowerampPreset {
    name: String,
    preamp: f32,
    parametric: bool,
    bands: Vec<PowerampBand>,
}

#[derive(Serialize, Deserialize)]
struct PowerampBand {
    r#type: u32, // Filter type
    channels: u32,
    frequency: u32,
    q: f32,
    gain: f32,
    color: i32,
}

enum FilterType {
    /* Available at squig.link */
    Pk,  // Peak Band (code 3)
    Lsq, // Low Shelf (code 4)
    Hsq, // High Shelf (code 5)

    /* Only available at Poweramp (I named these myself) */
    Lp, // Low pass (code 0)
    Hp, // High pass (code 1)
    Bp, // Band pass (code 2)
}

impl FilterType {
    pub fn from_squig(filter: String) -> Result<Self, String> {
        match filter.to_ascii_lowercase().as_str() {
            "pk" => Ok(FilterType::Pk),
            "lsq" => Ok(FilterType::Lsq),
            "hsq" => Ok(FilterType::Hsq),
            _ => Err(format!(
                "Type \"{}\" is not a valid squig.link filter type",
                filter
            )),
        }
    }

    pub fn to_poweramp(&self) -> u32 {
        match self {
            FilterType::Pk => 3,
            FilterType::Lsq => 4,
            FilterType::Hsq => 5,

            FilterType::Lp => 0,
            FilterType::Hp => 1,
            FilterType::Bp => 2,
        }
    }
}

/*
 * Without making the WASM function in a seperate
 * function as a wrapper, the unit test won't work
 *
 * Note: The Err(JsValue) will be used to show error in the front-end
 */
#[wasm_bindgen]
pub fn convert_to_poweramp(name: String, squig_link_peq: &str) -> Result<String, JsValue> {
    core_convert_to_poweramp(name, squig_link_peq).map_err(|e| JsValue::from_str(&e))
}

fn core_convert_to_poweramp(name: String, squig_link_peq: &str) -> Result<String, String> {
    let mut lines = squig_link_peq.lines();
    let preamp = if let Some(line) = lines.next() {
        convert_preamp_line_to_poweramp(line.to_string())
    } else {
        return Err("Failed to parse the first line".to_string());
    }?;

    let mut bands: Vec<PowerampBand> = vec![];
    bands.extend(initial_poweramp_preset_band());

    for band in lines {
        bands.push(convert_band_line_to_poweramp(band.to_string())?)
    }

    let preset = PowerampPreset {
        name,
        preamp,
        bands,
        parametric: true,
    };

    Ok(serde_json::to_string_pretty(&preset)
        .map_err(|_| "Failed to convert data structure to JSON string".to_string())?)
}

/*
 * Example of correct input format:
 * 1. "Filter 1: ON PK Fc 29 Hz Gain -5.8 dB Q 0.500"
 * 2. "Filter 1: OFF PK Fc 29 Hz Gain 5.0 dB Q 0.500"
 */
fn convert_band_line_to_poweramp(line: String) -> Result<PowerampBand, String> {
    let tokens: Vec<&str> = line.split_whitespace().collect();

    let get_token = |i: usize, name: &str| {
        tokens
            .get(i)
            .copied()
            .ok_or_else(|| format!("Failed to parse band line caused by a missing {}", name))
    };

    Ok(PowerampBand {
        r#type: FilterType::from_squig(get_token(3, "filter_type")?.to_string())?.to_poweramp(),
        channels: 0,
        frequency: get_token(5, "frequency")?
            .parse()
            .map_err(|_| "Invalid frequency value".to_string())?,
        q: get_token(11, "query")?
            .parse()
            .map_err(|_| "Invalid query value".to_string())?,
        gain: get_token(8, "gain")?
            .parse()
            .map_err(|_| "Invalid gain value".to_string())?,
        color: 0,
    })
}

fn convert_preamp_line_to_poweramp(line: String) -> Result<f32, String> {
    if let Some(gain) = line.split_whitespace().nth(1) {
        return gain
            .parse()
            .map_err(|_| "Preamp gain value is invalid".to_string());
    } else {
        return Err("Preamp gain is missing".to_string());
    };
}

fn initial_poweramp_preset_band() -> Vec<PowerampBand> {
    vec![
        PowerampBand {
            r#type: FilterType::Lp.to_poweramp(),
            channels: 0,
            frequency: 90,
            q: 0.8,
            gain: 0.0,
            color: 0,
        },
        PowerampBand {
            r#type: FilterType::Hp.to_poweramp(),
            channels: 0,
            frequency: 10000,
            q: 0.6,
            gain: 0.0,
            color: 0,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_squig_to_poweramp_conversion() {
        let input = "Preamp: -3.5 dB\n\
                     Filter 1: ON PK Fc 29 Hz Gain -5.8 dB Q 0.500\n\
                     Filter 2: ON LSQ Fc 105 Hz Gain 3.2 dB Q 0.700";

        let result = core_convert_to_poweramp("My Preset".to_string(), input);

        assert!(result.is_ok(), "Conversion failed: {:?}", result.err());

        let json_output = result.unwrap();

        let preset: PowerampPreset = serde_json::from_str(&json_output).unwrap();

        assert_eq!(preset.name, "My Preset");
        assert_eq!(preset.preamp, -3.5);
        assert!(preset.parametric);

        // 4 bands: 2 initial poweramp bands + 2 from input
        assert_eq!(preset.bands.len(), 4);

        // Test Filter 1
        let filter_1 = &preset.bands[2];
        assert_eq!(filter_1.r#type, 3);
        assert_eq!(filter_1.frequency, 29);
        assert_eq!(filter_1.gain, -5.8);
        assert_eq!(filter_1.q, 0.5);

        // Test Filter 2
        let filter_2 = &preset.bands[3];
        assert_eq!(filter_2.r#type, 4);
        assert_eq!(filter_2.frequency, 105);

        println!("{}", serde_json::to_string_pretty(&preset).unwrap());
    }

    #[test]
    fn test_invalid_input() {
        let input = "Preamp: invalid\nFilter 1: ON PK Fc 29 Hz Gain -5.8 dB Q 0.500";
        let result = core_convert_to_poweramp("Error Test".to_string(), input);
        assert!(result.is_err());
    }
}
