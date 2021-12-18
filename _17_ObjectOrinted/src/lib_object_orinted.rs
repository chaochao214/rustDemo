pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}


impl AveragedCollection {
    pub fn add(&mut self, value: i32)  {
        self.list.push(value);
        self.update_average();
    }

    pub fn average(&self)->f64{
        self.average
    }

    fn update_average(&mut self){m
        let total: i32= self.list.iter().su();
        self.average = total as  f64/ self.list.len() as f64;

    }


}

