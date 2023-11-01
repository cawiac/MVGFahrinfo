use ratatui::widgets::{ListState, ScrollbarState};

use crate::api;

#[derive(PartialEq)] // need this to do binary comparison
pub enum AppTabs {
    HomeTab,
    StationTab,
}
pub struct App {
    pub selected_tab: AppTabs,
    // pub counter: i64,
    pub should_quit: bool,
    pub stations: Vec<api::Station>,
    pub selected_station: Option<api::Station>,
    pub departures: Vec<api::DepartureInfo>,
    pub should_redraw: bool,
    // auto_refresh: bool,
    pub status: String,
    //scroll related
    // pub scroll_position: usize,
    pub scroll_state: ListState,
}

impl App {
    pub async fn new() -> Self {
        Self {
            selected_tab: AppTabs::HomeTab,
            // counter: 0,
            should_quit: false,
            stations: api::get_stations().await.unwrap_or_else(|_| vec![]),
            selected_station: None,
            departures: vec![],
            should_redraw: true,
            // auto_refresh: false,
            status: "Loading stations...".to_string(),
            // scroll_position: 0,
            scroll_state: ListState::default(),
        }
    }
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
    pub fn increment_station(&mut self) {
        // self.counter += 1;

        let i = match self.scroll_state.selected() {
            Some(i) => {
                if i >= self.stations.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.scroll_state.select(Some(i));
    }

    pub fn decrement_station(&mut self) {
        // if self.counter > 0 {
        //     self.counter -= 1;
        // }

        let i = match self.scroll_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.stations.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.scroll_state.select(Some(i));
    }
    pub fn toggle_tabs(&mut self) {
        match self.selected_tab {
            AppTabs::HomeTab => self.selected_tab = AppTabs::StationTab,
            AppTabs::StationTab => self.selected_tab = AppTabs::HomeTab,
        }
    }

    pub async fn update_departures(&mut self) {
        if let Some(station) = &self.selected_station {
            self.departures = match api::get_departures(&station.id).await {
                Ok(departures) => departures,
                Err(e) => {
                    println!("Error fetching departures {}", e);
                    vec![]
                }
            }
        }
    }

    pub async fn select_station(&mut self) {
        // self.selected_station = Some(self.stations[self.counter as usize].clone());
        self.selected_station = match self.scroll_state.selected() {
            Some(i) => Some(self.stations[i].clone()),
            None => None,
        };
        self.status = format!("Fetching departures");
        self.update_departures().await;
        self.selected_tab = AppTabs::HomeTab; // switch to home tab immidiatelyq
        self.should_redraw = true;
        // self.auto_refresh = true;
        // self.keep_refreshing_departures().await;
    }
}
