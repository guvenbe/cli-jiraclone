use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum Action{
    NavigateToEpicDetail {epic_id: u32},
    NavigateToStoryDetail {epic_id: u32, story_id: u32},
    NavigateToPreviousPage,
    CreateEpic,
    UpdateEpicStatus {epic_id: u32},
    DeleteEpic {epic_id: u32},
    CreateStory {epic_id: u32},
    UpdateStoryStatus {story_id: u32},
    DeleteStory {epic_id: u32, story_id: u32},
    Exit,
}
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<u32>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::Open,
            stories: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Story {
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub(crate) fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::Open,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct DBState {
    pub last_item_id: u32,
    pub epics: HashMap<u32, Epic>,
    pub stories: HashMap<u32, Story>,
}
