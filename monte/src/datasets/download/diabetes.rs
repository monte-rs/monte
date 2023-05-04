// use polars::{frame::DataFrame, series::Series};
use polars::prelude::*;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct DiabetesRecord {
    id: i32,
    preg: Option<i32>,
    plas: Option<i32>,
    pres: Option<i32>,
    skin: Option<i32>,
    insu: Option<i32>,
    mass: Option<f32>,
    pedi: Option<f32>,
    age: Option<i32>,
    class: Option<String>,
}

pub fn get_as_dataframe() -> Result<DataFrame, Box<dyn Error>> {
    let url =
        "https://raw.githubusercontent.com/monte-rs/monte-datasets/main/diabetes/diabetes.json"
            .to_owned();

    let response = reqwest::blocking::get(url)?.text()?;

    let record_data: Vec<DiabetesRecord> = serde_json::from_str(&response)?;

    let mut id = Vec::<i32>::new();
    let mut preg = Vec::<Option<i32>>::new();
    let mut plas = Vec::<Option<i32>>::new();
    let mut pres = Vec::<Option<i32>>::new();
    let mut skin = Vec::<Option<i32>>::new();
    let mut insu = Vec::<Option<i32>>::new();
    let mut mass = Vec::<Option<f32>>::new();
    let mut pedi = Vec::<Option<f32>>::new();
    let mut age = Vec::<Option<i32>>::new();
    let mut class = Vec::<Option<String>>::new();

    for record in record_data.into_iter() {
        id.push(record.id);
        preg.push(record.preg);
        plas.push(record.plas);
        pres.push(record.pres);
        skin.push(record.skin);
        insu.push(record.insu);
        mass.push(record.mass);
        pedi.push(record.pedi);
        age.push(record.age);
        class.push(record.class);
    }

    let id = Series::new("id", id);
    let preg = Series::new("preg", preg);
    let plas = Series::new("plas", plas);
    let pres = Series::new("pres", pres);
    let skin = Series::new("skin", skin);
    let insu = Series::new("insu", insu);
    let mass = Series::new("mass", mass);
    let pedi = Series::new("pedi", pedi);
    let age = Series::new("age", age);
    let class = Series::new("class", class);

    let df = DataFrame::new(vec![
        id, preg, plas, pres, skin, insu, mass, pedi, age, class,
    ])?;

    Ok(df)
}

// pub fn get() -> Result<DataFrame, Box<dyn Error>> {
//     let url =
//         "https://raw.githubusercontent.com/monte-rs/monte-datasets/main/diabetes/diabetes.csv"
//             .to_owned();

//     let record_data = reqwest::blocking::get(url)?.text()?;

//     let mut reader = csv::Reader::from_reader(record_data.as_bytes());

//     let mut id = Vec::<i32>::new();
//     let mut preg = Vec::<Option<i32>>::new();
//     let mut plas = Vec::<Option<i32>>::new();
//     let mut pres = Vec::<Option<i32>>::new();
//     let mut skin = Vec::<Option<i32>>::new();
//     let mut insu = Vec::<Option<i32>>::new();
//     let mut mass = Vec::<Option<f32>>::new();
//     let mut pedi = Vec::<Option<f32>>::new();
//     let mut age = Vec::<Option<i32>>::new();
//     let mut class = Vec::<Option<String>>::new();

//     for record in reader.deserialize() {
//         let record: DiabetesRecord = record?;

//         id.push(record.id);
//         preg.push(record.preg);
//         plas.push(record.plas);
//         pres.push(record.pres);
//         skin.push(record.skin);
//         insu.push(record.insu);
//         mass.push(record.mass);
//         pedi.push(record.pedi);
//         age.push(record.age);
//         class.push(record.class);
//     }

//     let df = df!(
//         "id" => id,
//         "preg" => preg,
//         "plas" => plas,
//         "pres" => pres,
//         "skin" => skin,
//         "insu" => insu,
//         "mass" => mass,
//         "pedi" => pedi,
//         "age" => age,
//         "class" => class,
//     )?;

//     Ok(df)
// }
