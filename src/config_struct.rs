pub struct GlobalConf{
    pub base_url:String
}

impl GlobalConf{
    pub fn new() -> GlobalConf{
        GlobalConf { 
            base_url:String::new(),
        }
    }
}