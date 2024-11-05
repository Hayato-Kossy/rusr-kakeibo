use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum IncomCategory {
  Salary,
  Bonus,
  Other,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ExpenceCategory {
    Food,
    Hoby,
    Other,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Category {
  Income(IncomCategory),
  Expence(ExpenceCategory),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Item {
  name: String,
  category: Category,
  price: u32,
  date: NaiveDate,
}

impl Item {
  pub fn new(  
    name: String,
    category: Category,
    price: u32,
    date: NaiveDate,
   ) -> Self {
    Item {
      name,
      category,
      price,
      date
    }
   }
}