use super::*;
use crate::objects::{Money, OrderServiceCharge, SearchOrdersFilter, SearchOrdersQuery, SearchOrdersSort};
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
            Ok(self)
        } else {
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