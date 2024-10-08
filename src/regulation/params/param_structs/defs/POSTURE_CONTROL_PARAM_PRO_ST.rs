use deku::prelude::*;
use deku::ctx::Endian;
use deku::{ DekuRead, DekuWrite};
#[derive(PartialEq, Debug, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian, version: u32")]
pub struct POSTURE_CONTROL_PARAM_PRO_ST {
	pub a000_rightArmIO: i16,
	pub a000_rightArmFB: i16,
	pub a000_leftArmIO: i16,
	pub a000_leftArmFB: i16,
	pub a002_rightArmIO: i16,
	pub a002_rightArmFB: i16,
	pub a002_leftArmIO: i16,
	pub a002_leftArmFB: i16,
	pub a003_rightArmIO: i16,
	pub a003_rightArmFB: i16,
	pub a003_leftArmIO: i16,
	pub a003_leftArmFB: i16,
	pub a010_rightArmIO: i16,
	pub a010_rightArmFB: i16,
	pub a010_leftArmIO: i16,
	pub a010_leftArmFB: i16,
	pub a012_rightArmIO: i16,
	pub a012_rightArmFB: i16,
	pub a012_leftArmIO: i16,
	pub a012_leftArmFB: i16,
	pub a013_rightArmIO: i16,
	pub a013_rightArmFB: i16,
	pub a013_leftArmIO: i16,
	pub a013_leftArmFB: i16,
	pub a014_rightArmIO: i16,
	pub a014_rightArmFB: i16,
	pub a014_leftArmIO: i16,
	pub a014_leftArmFB: i16,
	pub a015_rightArmIO: i16,
	pub a015_rightArmFB: i16,
	pub a015_leftArmIO: i16,
	pub a015_leftArmFB: i16,
	pub a016_rightArmIO: i16,
	pub a016_rightArmFB: i16,
	pub a016_leftArmIO: i16,
	pub a016_leftArmFB: i16,
	pub pad: [u8;8],
}
