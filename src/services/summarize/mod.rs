use std::{collections::{BTreeMap, BTreeSet}, result};

use chrono::{NaiveDate, Datelike};

use crate::models;

fn get_target_dates(data: &Vec<models::Item>) -> BTreeSet<NaiveDate> {
    let target_dates: BTreeSet<_> = data.iter().map(|item| {
        item.get_first_day()
    }).collect();
    target_dates
}

fn get_filtered_data(data: &Vec<models::Item>, filter_data: NaiveDate) -> Vec<&models::Item> {
    let filtered_data: Vec<&models::Item> = data.iter().filter(|item| {
        (item.get_year() == filter_data.year()) && (item.get_month() == filter_data.month())
    }).collect();
    filtered_data
}

fn summarize_data(data: &Vec<&models::Item>) -> i32 {
    let mut sum = 0;
    for item in data {
        sum += item.get_price_for_summary();
    }
    sum
}

fn format_date(date: NaiveDate) -> String {
    format!("{}/{}", date.year(), date.month())
}

fn format_price(price: i32) -> String {
    if price > 0 {
        format!("+{}", price)
    } else {
        format!("{}", price)
    }
}

fn print_table(resule_table: BTreeMap<NaiveDate, i32>) {
    for result in resule_table {
        let date = format_date(result.0);
        let price = format_price(result.1);
        println!("{}の収支は{}円でした", date, price);
    }
}