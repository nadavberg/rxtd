use serde::Deserialize;
use std::fs;
use std::path::Path;

// Root Struct
#[derive(Debug, Deserialize)]
pub struct RxPreset {
    #[serde(rename = "@name")]
    pub name: String,
    
    #[serde(rename = "$value")]
    pub tags: Vec<RxTag>,
}

#[derive(Debug, Deserialize)]
pub enum RxTag {
    #[serde(rename = "PARAM")]
    Param(RxParam),
    
    #[serde(rename = "SAMPLES")]
    Samples(RxSamples),
    
    #[serde(rename = "GUI")]
    Gui(RxGui),
}

// Generic Param Tag:
#[derive(Debug, Deserialize)]
pub struct RxParam {
    #[serde(rename = "@id")]
    pub id: String,
    
    #[serde(rename = "@value")]
    pub value: Option<f64>, 
}

// Samples Container:
#[derive(Debug, Deserialize)]
pub struct RxSamples {
    #[serde(rename = "SAMPLE", default)]
    pub items: Vec<RxSample>,
}

// Individual Sample:
#[derive(Debug, Deserialize)]
pub struct RxSample {
    #[serde(rename = "@id")]
    pub id: String,
    
    #[serde(rename = "@reversed")]
    pub reversed: bool,
    
    #[serde(rename = "@gain")]
    pub gain: f64,
    
    #[serde(rename = "@start")]
    pub start: u32,
    
    #[serde(rename = "@end")]
    pub end: u32,
    
    #[serde(rename = "REFERENCES")]
    pub references: Option<RxReferences>,
}

// References Container:
#[derive(Debug, Deserialize)]
pub struct RxReferences {
    #[serde(rename = "REFERENCE")]
    pub reference: Option<RxReference>,
}

// Individual Reference:
#[derive(Debug, Deserialize)]
pub struct RxReference {
    #[serde(rename = "@type")]
    pub ref_type: String,
    
    #[serde(rename = "@value")]
    pub value: String,
}

// GUI Container:
#[derive(Debug, Deserialize)]
pub struct RxGui {
    #[serde(rename = "PARAM", default)]
    pub params: Vec<RxParam>, 
}

impl RxPreset {
    pub fn load_from_file(path: &Path) -> anyhow::Result<Self> {
        let xml = fs::read_to_string(path)?;
        let preset = quick_xml::de::from_str(&xml)?;
        Ok(preset)
    }
}
