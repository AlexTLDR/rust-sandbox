#[derive(Debug)]
struct Product {
    id: u32,
    name: String,
    price: f32,
}
#[derive(Debug)]
enum OrderStatus {
    Pending,
    Processing,
    Shipped,
    Delivered,
    Canceled,
}
#[derive(Debug)]
struct Order {
    order_id: u32,
    product: Product,
    quantity: u32,
    status: OrderStatus,
}

impl Order {
    fn new(order_id: u32, product: Product, quantity: u32) -> Self {
        Self {
            order_id,
            product,
            quantity,
            status: OrderStatus::Pending,
        }
    }
    fn update_status(&mut self, new_status: OrderStatus) {
        self.status = new_status;
    }
}

fn main() {
    let product = Product {
        id: 99,
        name: String::from("Tuxedo Laptop"),
        price: 2578.0,
    };

    let mut order = Order::new(1, product, 10);
    println!(
        "Order id {} has the current status of {:?}. The ful details of the order are {:?}",
        order.order_id, order.status, order
    );
    order.update_status(OrderStatus::Delivered);
    println!(
        "Order id {} has the current status of {:?}. The ful details of the order are {:?}",
        order.order_id, order.status, order
    )
}
