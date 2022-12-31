// https://leetcode.com/problems/design-underground-system/

use std::collections::HashMap;

#[derive(Default)]
struct UndergroundSystem {
    start_time: HashMap<i32, (String, i32)>,
    avg_time: HashMap<(String, String), (f64, usize)>,
}

impl UndergroundSystem {

    fn new() -> Self {
        Default::default()
    }
    
    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.start_time.insert(id, (station_name, t));
    }
    
    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        let (from_station, from_time) = self.start_time.remove(&id).unwrap();
        let entry = self.avg_time.entry((from_station, station_name)).or_default();
        let (time, count) = *entry;
        let (diff, amount) = ((t - from_time) as f64, count as f64);
        
        *entry = ((time * amount + diff) / (amount + 1.), count + 1);
    }
    
    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        self.avg_time[&(start_station, end_station)].0
    }
}

#[cfg(test)]
mod tests {
    use super::UndergroundSystem;

    #[test]
    fn test() {
        let mut obj = UndergroundSystem::new();
        obj.check_in(45, "Leyton".into(), 3);
        obj.check_in(32, "Paradise".into(), 8);
        obj.check_in(27, "Leyton".into(), 10);
        
        obj.check_out(45, "Waterloo".into(), 15);
        obj.check_out(27, "Waterloo".into(), 20);
        obj.check_out(32, "Cambridge".into(), 22);
        
        let avg_time = obj.get_average_time("Paradise".into(), "Cambridge".into());
        assert_eq!(avg_time, 14.);
        
        let avg_time = obj.get_average_time("Leyton".into(), "Waterloo".into());
        assert_eq!(avg_time, 11.);
        
        obj.check_in(10, "Leyton".into(), 24);
        let avg_time = obj.get_average_time("Leyton".into(), "Waterloo".into());
        assert_eq!(avg_time, 11.);
        
        obj.check_out(10, "Waterloo".into(), 38);
        
        let avg_time = obj.get_average_time("Leyton".into(), "Waterloo".into());
        assert_eq!(avg_time, 12.);
    }
}
