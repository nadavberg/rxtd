use serde::Deserialize;
use std::fs;
use std::path::Path;

// Root Struct
#[derive(Debug, Deserialize)]
pub struct Preset {
    #[serde(rename = "@name")]
    pub name: String,
    
    #[serde(rename = "$value")]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Deserialize)]
pub enum Tag {
    #[serde(rename = "PARAM")]
    Param(Param),
    
    #[serde(rename = "SAMPLES")]
    Samples(Samples),
    
    #[serde(rename = "GUI")]
    Gui(Gui),
}

// Generic Param Tag:
#[derive(Debug, Deserialize)]
pub struct Param {
    #[serde(rename = "@id")]
    pub id: String,
    
    #[serde(rename = "@value")]
    pub value: Option<f64>, 
}

// Samples Container:
#[derive(Debug, Deserialize)]
pub struct Samples {
    #[serde(rename = "SAMPLE", default)]
    pub items: Vec<Sample>,
}

// Individual Sample:
#[derive(Debug, Deserialize)]
pub struct Sample {
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
    pub references: Option<References>,
}

// References Container:
#[derive(Debug, Deserialize)]
pub struct References {
    #[serde(rename = "REFERENCE")]
    pub reference: Option<Reference>,
}

// Individual Reference:
#[derive(Debug, Deserialize)]
pub struct Reference {
    #[serde(rename = "@type")]
    pub ref_type: String,
    
    #[serde(rename = "@value")]
    pub value: String,
}

// GUI Container:
#[derive(Debug, Deserialize)]
pub struct Gui {
    #[serde(rename = "PARAM", default)]
    pub params: Vec<Param>, 
}

impl Preset {
    pub fn load_from_file(path: &Path) -> anyhow::Result<Self> {
        let xml = fs::read_to_string(path)?;
        let preset = quick_xml::de::from_str(&xml)?;
        Ok(preset)
    }
}
