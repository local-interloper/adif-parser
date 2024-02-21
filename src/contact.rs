use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Default, Clone)]
pub struct Contact(pub HashMap<String, String>);

impl Display for Contact {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (k, v) in self.0.iter() {
            f.write_fmt(format_args!("{}: {}\n", k, v))?;
        }

        Ok(())
    }
}

impl Contact {
    pub fn call(&self) -> Option<&String> { self.0.get("call") }
    pub fn time_on(&self) -> Option<&String> { self.0.get("time_on") }
    pub fn qso_date(&self) -> Option<&String> { self.0.get("qso_date") }
    pub fn band(&self) -> Option<&String> { self.0.get("band") }
    pub fn freq(&self) -> Option<&String> { self.0.get("freq") }
    pub fn mode(&self) -> Option<&String> { self.0.get("mode") }
    pub fn rst_sent(&self) -> Option<&String> { self.0.get("rst_sent") }
    pub fn rst_rcvd(&self) -> Option<&String> { self.0.get("rst_rcvd") }
}