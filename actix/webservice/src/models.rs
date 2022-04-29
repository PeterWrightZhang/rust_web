use actix_web::web;
use chrono::NativeDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Course {
    pub teacher_id: usize,
    pub id: Option<usize>,
    pub name: String,
    pub time: Option<NativeDateTime>,
}

impl From<web::Json<Course>> for Course {
    fn from(course: web::Json<Course>) -> Self {
        Self {
            teacher_id: course.teacher_id,
            id: course.id,
            name: course.name.clone(),
            time: course.time,
        }
    }
}