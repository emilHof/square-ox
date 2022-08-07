use super::*;
use crate::objects::{TimeRange, DeviceCheckoutOptions, Money, Order, OrderLineItem, OrderServiceCharge, SearchOrdersFilter, SearchOrdersQuery, SearchOrdersSort, TerminalCheckoutQuery, TerminalCheckoutQueryFilter, TerminalCheckoutQuerySort, TerminalRefundQuery, TerminalRefundQueryFilter, TipSettings, InventoryChange, InventoryPhysicalCount, InventoryAdjustment, InventoryTransfer};
use crate::objects::enums::{InventoryChangeType, OrderServiceChargeCalculationPhase, SearchOrdersSortField, SortOrder, TerminalCheckoutStatus};

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
        if self.location_id.is_some(){
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

    pub fn add_service_charge(mut self, service_charge: OrderServiceCharge) -> Self {
        if let Some(service_charges) = self.body.service_charges.as_mut() {
            service_charges.push(service_charge);
        } else {
            self.body.service_charges = Some(vec![service_charge])
        };

        self
    }

    pub fn add_order_item(mut self, order_item: OrderLineItem) -> Self {
        if let Some(line_items) = self.body.line_items.as_mut() {
            line_items.push(order_item);
        } else {
            self.body.line_items = Some(vec![order_item])
        };

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

impl AddField<OrderLineItem> for Order {
    fn add_field(&mut self, field: OrderLineItem) {
        if let Some(line_items) = self.line_items.as_mut() {
            line_items.push(field);
        } else {
            self.line_items = Some(vec![field]);
        }
    }
}

// -------------------------------------------------------------------------------------------------
// DeviceCheckoutOptions builder implementation
// -------------------------------------------------------------------------------------------------
impl Validate for DeviceCheckoutOptions {
    fn validate(self) -> Result<Self, ValidationError> where Self: Sized {
        if self.device_id.is_some() {
            Ok(self)
        } else {
            Err(ValidationError)
        }
    }
}

impl<T: ParentBuilder> Builder<DeviceCheckoutOptions, T> {
    pub fn device_id(mut self, device_id: String) -> Self {
        self.body.device_id = Some(device_id);

        self
    }

    pub fn collect_signature(mut self) -> Self {
        self.body.collect_signature = Some(true);

        self
    }

    pub fn show_itemized_cart(mut self) -> Self {
        self.body.show_itemized_cart = Some(true);

        self
    }

    pub fn skip_receipt_screen(mut self) -> Self {
        self.body.skip_receipt_screen = Some(true);

        self
    }

    pub fn tip_settings(mut self, tip_settings: TipSettings) -> Self {
        self.body.tip_settings = Some(tip_settings);

        self
    }
}

// -------------------------------------------------------------------------------------------------
// TerminalCheckoutQuery builder implementation
// -------------------------------------------------------------------------------------------------
impl Validate for TerminalCheckoutQuery {
    fn validate(self) -> Result<Self, ValidationError> where Self: Sized {
        Ok(self)
    }
}

impl<T: ParentBuilder> Builder<TerminalCheckoutQuery, T> {
    pub fn sort_ascending(mut self) -> Self {
        self.body.sort = Some(TerminalCheckoutQuerySort { sort_order: Some(SortOrder::Asc) });

        self
    }

    pub fn sort_descending(mut self) -> Self {
        self.body.sort = Some(TerminalCheckoutQuerySort { sort_order: Some(SortOrder::Desc) });

        self
    }

    pub fn created_at(mut self, created_at: TimeRange) -> Self {
        if let Some(filter) = self.body.filter.as_mut() {
            filter.created_at = Some(created_at);
        } else {
            self.body.filter = Some(TerminalCheckoutQueryFilter {
                created_at: Some(created_at),
                device_id: None,
                status: None
            })
        };

        self
    }

    pub fn device_id(mut self, device_id: String) -> Self {
        if let Some(filter) = self.body.filter.as_mut() {
            filter.device_id = Some(device_id);
        } else {
            self.body.filter = Some(TerminalCheckoutQueryFilter {
                created_at: None,
                device_id: Some(device_id),
                status: None
            })
        };

        self
    }

    pub fn status(mut self, status: TerminalCheckoutStatus) -> Self {
        if let Some(filter) = self.body.filter.as_mut() {
            filter.status = Some(status);
        } else {
            self.body.filter = Some(TerminalCheckoutQueryFilter {
                created_at: None,
                device_id: None,
                status: Some(status)
            })
        };

        self
    }
}

// -------------------------------------------------------------------------------------------------
// SearchOrdersQuery builder implementation
// -------------------------------------------------------------------------------------------------
impl Validate for TerminalRefundQuery {
    fn validate(self) -> Result<Self, ValidationError> where Self: Sized {
        Ok(self)
    }
}

impl<T: ParentBuilder> Builder<TerminalRefundQuery, T> {
    pub fn created_at(mut self, created_at: TimeRange) -> Self {
        if let Some(filter) = self.body.filter.as_mut() {
            filter.created_at = Some(created_at)
        } else {
            self.body.filter = Some(TerminalRefundQueryFilter {
                created_at: Some(created_at),
                device_id: None,
                status: None
            })
        }

        self
    }

    pub fn device_id(mut self, device_id: String) -> Self {
        if let Some(filter) = self.body.filter.as_mut() {
            filter.device_id = Some(device_id)
        } else {
            self.body.filter = Some(TerminalRefundQueryFilter {
                created_at: None,
                device_id: Some(device_id),
                status: None
            })
        }

        self
    }

    pub fn pending(mut self) -> Self {
        if let Some(filter) = self.body.filter.as_mut() {
            filter.status = Some(TerminalCheckoutStatus::Pending)
        } else {
            self.body.filter = Some(TerminalRefundQueryFilter {
                created_at: None,
                device_id: None,
                status: Some(TerminalCheckoutStatus::Pending)
            })
        }

        self
    }

    pub fn in_progress(mut self) -> Self {
        if let Some(filter) = self.body.filter.as_mut() {
            filter.status = Some(TerminalCheckoutStatus::InProgress)
        } else {
            self.body.filter = Some(TerminalRefundQueryFilter {
                created_at: None,
                device_id: None,
                status: Some(TerminalCheckoutStatus::InProgress)
            })
        }

        self
    }

    pub fn cancel_requested(mut self) -> Self {
        if let Some(filter) = self.body.filter.as_mut() {
            filter.status = Some(TerminalCheckoutStatus::CancelRequested)
        } else {
            self.body.filter = Some(TerminalRefundQueryFilter {
                created_at: None,
                device_id: None,
                status: Some(TerminalCheckoutStatus::CancelRequested)
            })
        }

        self
    }

    pub fn canceled(mut self) -> Self {
        if let Some(filter) = self.body.filter.as_mut() {
            filter.status = Some(TerminalCheckoutStatus::Canceled)
        } else {
            self.body.filter = Some(TerminalRefundQueryFilter {
                created_at: None,
                device_id: None,
                status: Some(TerminalCheckoutStatus::Canceled)
            })
        }

        self
    }

    pub fn completed(mut self) -> Self {
        if let Some(filter) = self.body.filter.as_mut() {
            filter.status = Some(TerminalCheckoutStatus::Completed)
        } else {
            self.body.filter = Some(TerminalRefundQueryFilter {
                created_at: None,
                device_id: None,
                status: Some(TerminalCheckoutStatus::Completed)
            })
        }

        self
    }

    pub fn sort_ascending(mut self) -> Self {
        self.body.sort = Some(TerminalCheckoutQuerySort{ sort_order: Some(SortOrder::Asc) });

        self
    }

    pub fn sort_descending(mut self) -> Self {
        self.body.sort = Some(TerminalCheckoutQuerySort{ sort_order: Some(SortOrder::Desc) });

        self
    }
}

// -------------------------------------------------------------------------------------------------
// SearchOrdersQuery builder implementation
// -------------------------------------------------------------------------------------------------
impl Validate for InventoryChange {
    fn validate(self) -> Result<Self, ValidationError> where Self: Sized {
        Ok(self)
    }
}

impl<T: ParentBuilder> Builder<InventoryChange, T> {
    pub fn change_type(mut self, change_type: InventoryChangeType) -> Self {
        self.body.inventory_change_type = change_type;

        self
    }

    pub fn physical_count(mut self, physical_count: InventoryPhysicalCount) -> Self {
        self.body.physical_count = Some(physical_count);

        self
    }

    pub fn adjustment(mut self, adjustment: InventoryAdjustment) -> Self {
        self.body.adjustment = Some(adjustment);

        self
    }

    pub fn transfer(mut self, transfer: InventoryTransfer) -> Self {
        self.body.transfer = Some(transfer);

        self
    }
}