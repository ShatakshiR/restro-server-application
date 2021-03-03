# restro-server-application
Restaurant Management Server side application

**Introduction**
This application exposes some endpoints for various function of a restaurant. The REST api's are readily available for client apps to:
1. Create Menu Items
2. Update Menu Items
3. Fetch Menu Items
4. Delete Menu Items
5. Create Order - embedding the id's of one or more menu items
6. This request returns a Complete order placed by the customer alongwith Billing amount, average waiting time and order id. 

**Installation**

You can either clone this repo or download the zip file.
To execute this you need to have the Rust environment setup on your system. 
It runs fine on the stable version of Rust, it also worked fine with the nightly version.
To Run the application to can just tap into the app directory and run the Command 'cargo run'
The default setup runs on http://localhost:8000/
The application is equipped with mongodb cluster for the data connectivity.


**End Points and their use**
**Menu**
1. Fetch all menu items
   http://localhost:8000/menu
   
   This end point returns all the menu items in the collection
  <code> Sample response: [
    {
        "_id": {
            "$oid": "603876d72c19a8cde68a6a13"
        },
        "title": "PIZZA",
        "itemType": "Vegetarian",
        "timeToPrepare": 15,
        "price": 1600,
        "name": "Five Pepper",
        "description": "Mozarella Cheese, Tomato Sauce, Red Paprika, Capsicum, Bell Pepper"
    },
    {
        "_id": {
            "$oid": "60387e952c19a8cde68a6a14"
        },
        "title": "PIZZA",
        "itemType": "Vegetarian",
        "timeToPrepare": 20,
        "price": 1200,
        "name": "Spinach & Corn",
        "description": "Sauted Spinach & Sweet Corn"
    }] </code>
   
2. Fetch menu item by id
Endpoint: http://localhost:8000/menu/<item_id>
Request: http://localhost:8000/menu/603876d72c19a8cde68a6a13
Response: {
    "_id": {
        "$oid": "603876d72c19a8cde68a6a13"
    },
    "title": "PIZZA",
    "itemType": "Vegetarian",
    "timeToPrepare": 15,
    "price": 1200,
    "name": "MARGHERITA",
    "description": "Mozarella Cheese, Tomato Sauce"
}

3. Create menu item:
Endpoint: http://localhost:8000/menu/
Request body: {"title": "Desserts", "name": "Chocolate Mud Pie", "timeToPrepare": 10, "price": 830, "itemType":"Vegetarian", "description": "A delicious dessert from Cafe Monza"}
Response: Created: 201

4. Update menu Item: 

Endpoint: http://localhost:8000/menu/<item_id>
Request: http://localhost:8000/menu/603876d72c19a8cde68a6a13
Response: {
    "_id": {
        "$oid": "603876d72c19a8cde68a6a13"
    },
    "title": "PIZZA",
    "itemType": "Vegetarian",
    "timeToPrepare": 15,
    "price": 600,
    "name": "Five Pepper",
    "description": "Mozarella Cheese, Tomato Sauce, Red Paprika, Capsicum, Bell Pepper"
}

5. Create Order
Method: POST
Endpoint: http://localhost:8000/order/
Request: {"table_no":24,
"order_status": "Order Placed",
"ordered_items":[{
    "item_id":"6038821a2c19a8cde68a6a1c",
    "quantity": 1,
    "price": 380 
}, 
{
    "item_id":"603881c62c19a8cde68a6a1b",
    "quantity": 1,
    "price": 480
}, 
{
    "item_id":"603db0e200b2d4a400c41e6e",
    "quantity": 1,
    "price":920 
}] }

Response: {
    "_id": {
        "$oid": "603fd1ee00e5ac1800d840b1"
    },
    "order_id": 2,
    "ordered_items": [
        {
            "_id": {
                "$oid": "603fd1ee00e5ac1800d840b2"
            },
            "order_id": "603fd1ee00e5ac1800d840b1",
            "item_id": "6038821a2c19a8cde68a6a1c",
            "quantity": 1,
            "price": 380
        },
        {
            "_id": {
                "$oid": "603fd1ee00e5ac1800d840b3"
            },
            "order_id": "603fd1ee00e5ac1800d840b1",
            "item_id": "603881c62c19a8cde68a6a1b",
            "quantity": 1,
            "price": 480
        },
        {
            "_id": {
                "$oid": "603fd1ee00e5ac1800d840b4"
            },
            "order_id": "603fd1ee00e5ac1800d840b1",
            "item_id": "603db0e200b2d4a400c41e6e",
            "quantity": 1,
            "price": 920
        }
    ],
    "table_no": 24,
    "order_status": "Order Placed",
    "total_amount": 1869.0,
    "waiting_time": 10
}
