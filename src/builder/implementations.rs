use super::*;
use crate::objects::{Money, Order, OrderServiceCharge, SearchOrdersFilter, SearchOrdersQuery, SearchOrdersSort};
use crate::objects::enums::{OrderServiceChargeCalculationPhase, SearchOrdersSortField, SortOrder};

// -------------------------------------------------------------------------------------------------
// OrderServiceCharge builder implementation
// -------------------------------------------------------------------------------------------------
impl Validate for OrderServiceCharge {
    fn validate(self) -> Result<Self, ValidationError> {
        println!("{:?}", &self);
        if self.amount_money.is_some() &&
            self.name.is_some() &&
            self.calculation_phase.is_some() {
            println!("no error");
            Ok(self)
        } else {
            println!("error");
            Err(ValidationError)
        }
    }
}

impl<T: ParentBuilder> Builder<OrderServiceCharge, T> {
    pub fn amount_money(mut self, amount: Money) -> Self {
        self.body.amount_money = Some(amount);

        self
    }

    pub fn total_phase(mut self)
        -> Self {
        self.body.calculation_phase = Some(OrderServiceChargeCalculationPhase::TotalPhase);

        self
    }

    pub fn subtotal_phase(mut self)
        -> Self {
        self.body.calculation_phase = Some(OrderServiceChargeCalculationPhase::SubtotalPhase);

        self
    }

    pub fn not_taxable(mut self)-> Self {
        self.body.taxable = Some(false);

        self
    }

    pub fn taxable(mut self) -> Self {
        self.body.taxable = Some(true);

        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.body.name = Some(name);

        self
    }
}

// -------------------------------------------------------------------------------------------------
// SearchOrdersQuery builder implementation
// -------------------------------------------------------------------------------------------------
impl Validate for SearchOrdersQuery {
    fn validate(self) -> Result<Self, ValidationError> {
        Ok(self)
    }
}

impl<T: ParentBuilder> Builder<SearchOrdersQuery, T> {
    pub fn filter(mut self, filter: SearchOrdersFilter) -> Self {
        self.body.filter = Some(filter);

        self
    }

    pub fn sort_ascending(mut self) -> Self {
        match self.body.sort.as_mut() {
            Some(sort) => sort.sort_order= Some(SortOrder::Asc),
            None => self.body.sort = Some(SearchOrdersSort {
                sort_field: Some(SearchOrdersSortField::CreatedAt),
                sort_order: Some(SortOrder::Asc),
            })
        }

        self
    }

    pub fn sort_descending(mut self) -> Self {
        match self.body.sort.as_mut() {
            Some(sort) => sort.sort_order= Some(SortOrder::Desc),
            None => self.body.sort = Some(SearchOrdersSort {
                sort_field: Some(SearchOrdersSortField::CreatedAt),
                sort_order: Some(SortOrder::Desc),
            })
        }

        self
    }

    pub fn sort_field(mut self, sort_field: SearchOrdersSortField) -> Self {
        match self.body.sort.as_mut() {
            Some(sort) => sort.sort_field = Some(sort_field),
            None => self.body.sort = Some(SearchOrdersSort {
                sort_field: Some(sort_field),
                sort_order: None,
            })
        }

        self
    }
}
// -------------------------------------------------------------------------------------------------
// Order builder implementation
// -------------------------------------------------------------------------------------------------
impl Validate for Order {
    fn validate(self) -> Result<Self, ValidationError> where Self: Sized {
        if self.location_id.is_some() &&
            self.version.is_some() {
            Ok(self)
        } else {
            Err(ValidationError)
        }
    }
}

impl<T: ParentBuilder> Builder<Order, T> {
    pub fn location_id(mut self, location_id: String) -> Self {
        self.body.location_id = Some(location_id);

        self
    }

    pub fn version(mut self, version: i64) -> Self {
        self.body.version = Some(version);

        self
    }
}

impl AddField<OrderServiceCharge> for Order {
    fn add_field(&mut self, field: OrderServiceCharge) {
        if let Some(service_charges) = self.service_charges.as_mut() {
            service_charges.push(field);
        } else {
            self.service_charges = Some(vec![field]);
        }
    }
}