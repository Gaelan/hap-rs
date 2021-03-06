// this file is auto-generated by hap-codegen

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    service::HapService,
    characteristic::{
        HapCharacteristic,
		active::ActiveCharacteristic,
		current_heater_cooler_state::CurrentHeaterCoolerStateCharacteristic,
		target_heater_cooler_state::TargetHeaterCoolerStateCharacteristic,
		current_temperature::CurrentTemperatureCharacteristic,
		lock_physical_controls::LockPhysicalControlsCharacteristic,
		name::NameCharacteristic,
		swing_mode::SwingModeCharacteristic,
		cooling_threshold_temperature::CoolingThresholdTemperatureCharacteristic,
		heating_threshold_temperature::HeatingThresholdTemperatureCharacteristic,
		temperature_display_units::TemperatureDisplayUnitsCharacteristic,
		rotation_speed::RotationSpeedCharacteristic,
	},
    HapType,
};

/// Heater Cooler Service.
#[derive(Debug, Default)]
pub struct HeaterCoolerService {
    /// ID of the Heater Cooler Service.
    id: u64,
    /// `HapType` of the Heater Cooler Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

	/// Active Characteristic (required).
	pub active: ActiveCharacteristic,
	/// Current Heater Cooler State Characteristic (required).
	pub current_heater_cooler_state: CurrentHeaterCoolerStateCharacteristic,
	/// Target Heater Cooler State Characteristic (required).
	pub target_heater_cooler_state: TargetHeaterCoolerStateCharacteristic,
	/// Current Temperature Characteristic (required).
	pub current_temperature: CurrentTemperatureCharacteristic,

	/// Lock Physical Controls Characteristic (optional).
	pub lock_physical_controls: Option<LockPhysicalControlsCharacteristic>,
	/// Name Characteristic (optional).
	pub name: Option<NameCharacteristic>,
	/// Swing Mode Characteristic (optional).
	pub swing_mode: Option<SwingModeCharacteristic>,
	/// Cooling Threshold Temperature Characteristic (optional).
	pub cooling_threshold_temperature: Option<CoolingThresholdTemperatureCharacteristic>,
	/// Heating Threshold Temperature Characteristic (optional).
	pub heating_threshold_temperature: Option<HeatingThresholdTemperatureCharacteristic>,
	/// Temperature Display Units Characteristic (optional).
	pub temperature_display_units: Option<TemperatureDisplayUnitsCharacteristic>,
	/// Rotation Speed Characteristic (optional).
	pub rotation_speed: Option<RotationSpeedCharacteristic>,
}

impl HeaterCoolerService {
    /// Creates a new Heater Cooler Service.
    pub fn new(id: u64, accessory_id: u64) -> Self {
        Self {
            id,
            hap_type: HapType::HeaterCooler,
			active: ActiveCharacteristic::new(id + 1 + 0, accessory_id),
			current_heater_cooler_state: CurrentHeaterCoolerStateCharacteristic::new(id + 1 + 1, accessory_id),
			target_heater_cooler_state: TargetHeaterCoolerStateCharacteristic::new(id + 1 + 2, accessory_id),
			current_temperature: CurrentTemperatureCharacteristic::new(id + 1 + 3, accessory_id),
			..Default::default()
        }
    }
}

impl HapService for HeaterCoolerService {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_type(&self) -> HapType {
        self.hap_type
    }

    fn get_hidden(&self) -> bool {
        self.hidden
    }

    fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }

    fn get_primary(&self) -> bool {
        self.primary
    }

    fn set_primary(&mut self, primary: bool) {
        self.primary = primary;
    }

    fn get_characteristic(&self, hap_type: HapType) -> Option<&dyn HapCharacteristic> {
        for characteristic in self.get_characteristics() {
            if characteristic.get_type() == hap_type {
                return Some(characteristic);
            }
        }
        None
    }

    fn get_mut_characteristic(&mut self, hap_type: HapType) -> Option<&mut dyn HapCharacteristic> {
        for characteristic in self.get_mut_characteristics() {
            if characteristic.get_type() == hap_type {
                return Some(characteristic);
            }
        }
        None
    }

    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> {
        let mut characteristics: Vec<&dyn HapCharacteristic> = vec![
			&self.active,
			&self.current_heater_cooler_state,
			&self.target_heater_cooler_state,
			&self.current_temperature,
		];
		if let Some(c) = &self.lock_physical_controls {
		    characteristics.push(c);
		}
		if let Some(c) = &self.name {
		    characteristics.push(c);
		}
		if let Some(c) = &self.swing_mode {
		    characteristics.push(c);
		}
		if let Some(c) = &self.cooling_threshold_temperature {
		    characteristics.push(c);
		}
		if let Some(c) = &self.heating_threshold_temperature {
		    characteristics.push(c);
		}
		if let Some(c) = &self.temperature_display_units {
		    characteristics.push(c);
		}
		if let Some(c) = &self.rotation_speed {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
			&mut self.active,
			&mut self.current_heater_cooler_state,
			&mut self.target_heater_cooler_state,
			&mut self.current_temperature,
		];
		if let Some(c) = &mut self.lock_physical_controls {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.name {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.swing_mode {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.cooling_threshold_temperature {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.heating_threshold_temperature {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.temperature_display_units {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.rotation_speed {
		    characteristics.push(c);
		}
		characteristics
    }
}

impl Serialize for HeaterCoolerService {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapService", 5)?;
        state.serialize_field("iid", &self.get_id())?;
        state.serialize_field("type", &self.get_type())?;
        state.serialize_field("hidden", &self.get_hidden())?;
        state.serialize_field("primary", &self.get_primary())?;
        state.serialize_field("characteristics", &self.get_characteristics())?;
        // linked services left out for now
        state.end()
    }
}
