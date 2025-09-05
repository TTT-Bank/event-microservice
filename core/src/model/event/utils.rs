#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
        Approved,
        Rejected,
        OnReview
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Currency {
        Rubble
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cost {
        currency: Currency,
        amount: u16
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Place {
        city: String,
        street: String,
        house: u8,
        housing: Option<u8>,
        building: Option<u8>,
        metro: Option<String>
}