// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: i32,
    unit_price: i32,
}

impl Order {
    pub fn new(product_name: String, quantity: i32, unit_price: i32) -> Order {
        let mut result = Order {
            product_name: "".to_string(),
            quantity: 0,
            unit_price: 0,
        };
        result.set_product_name(product_name);
        result.set_quantity(quantity);
        result.set_unit_price(unit_price);
        return result;
    }

    pub fn product_name(&self) -> &String {
        return &self.product_name;
    }

    pub fn quantity(&self) -> &i32 {
        return &self.quantity;
    }

    pub fn unit_price(&self) -> &i32 {
        return &self.unit_price;
    }
    
    pub fn total(&self) -> i32 {
        return self.unit_price * self.quantity;
    }
    
    pub fn set_product_name(&mut self, product_name: String) {
        match product_name.as_str().len() {
            d if d < 1 => panic!("product_name is required"),
            d if d > 300 => panic!("product_name may not be longer than 300 characters"),
            _ => self.product_name = product_name
        }
    }
    
    pub fn set_quantity(&mut self, quantity: i32) {
        match quantity {
            0 => panic!("quantity must be > 0"),
            _ => self.quantity = quantity
        }
    }
    
    pub fn set_unit_price(&mut self, unit_price: i32) {
        match unit_price {
            0 => panic!("unit_price must be > 0"),
            _ => self.unit_price = unit_price
        }
        
    }
}

