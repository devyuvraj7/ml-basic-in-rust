use csv::Reader;
use linfa::prelude::*;
use linfa::Dataset;
use linfa_trees::DecisionTree;
use ndarray::{Array, Array1, Array2};
use std::fs::File;

fn decision_tree() {
    let (train, test) = linfa_datasets::iris().split_with_ratio(0.9);

    let model = DecisionTree::params().fit(&train).unwrap();

    let predictions = model.predict(&test);

    println!("{:?}", predictions);
    println!("{:?}", test.targets);
}

fn get_dataset() -> Dataset<f32, i32, ndarray::Dim<[usize; 1]>> {
    let mut reader = Reader::from_path("./src/heart.csv").unwrap();

    let headers = get_headers(&mut reader);
    println!("header {:?}", headers);
    let data = get_data(&mut reader);
    let target_index = headers.len() - 1;

    let features = headers[0..target_index].to_vec();
    let records = get_records(&data, target_index);
    println!("record {:?}", records);
    let targets = get_targets(&data, target_index);
    println!("target {:?}", targets);

    return Dataset::new(records, targets).with_feature_names(features);
}

fn get_headers(reader: &mut Reader<File>) -> Vec<String> {
    return reader
        .headers()
        .unwrap()
        .iter()
        .map(|r| r.to_owned())
        .collect();
}

fn get_records(data: &Vec<Vec<f32>>, target_index: usize) -> Array2<f32> {
    let mut records: Vec<f32> = vec![];
    for record in data.iter() {
        records.extend_from_slice(&record[0..target_index]);
    }
    return Array::from(records).into_shape((303, 13)).unwrap();
}

fn get_targets(data: &Vec<Vec<f32>>, target_index: usize) -> Array1<i32> {
    let targets = data
        .iter()
        .map(|record| record[target_index] as i32)
        .collect::<Vec<i32>>();
    return Array::from(targets);
}

fn get_data(reader: &mut Reader<File>) -> Vec<Vec<f32>> {
    return reader
        .records()
        .map(|r| {
            r.unwrap()
                .iter()
                .map(|field| field.parse::<f32>().unwrap())
                .collect::<Vec<f32>>()
        })
        .collect::<Vec<Vec<f32>>>();
}

fn main() {
    let dataset = get_dataset();
    println!("{:?}", dataset);
    decision_tree();
}
