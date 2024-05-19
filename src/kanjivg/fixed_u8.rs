#[derive(Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize)]
pub struct FixedU8(u8);

impl From<f32> for FixedU8 {
    fn from(value: f32) -> Self {
        // assert!(
        //     (-0.02..=1.02).contains(&value),
        //     "Value out of range for U8, must be between 0 and 1: {}",
        //     value
        // );
        FixedU8((value.clamp(0.0, 1.0) * 255.0).round() as u8)
    }
}

impl From<FixedU8> for f32 {
    fn from(value: FixedU8) -> Self { value.0 as f32 / 255.0 }
}
