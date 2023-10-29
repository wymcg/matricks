#[derive(Clone)]
pub struct PathMap {
    pub from: String,
    pub to: String,
}

impl PathMap {
    pub fn from_string(map_string: String) -> Result<Self, ()> {
        let split_map_string = map_string.split_once('>');
        let (from, to) = match split_map_string {
            None => return Err(()),
            Some(res) => res,
        };
        let (from, to) = (from.to_string(), to.to_string());

        Ok(Self { from, to })
    }
}
