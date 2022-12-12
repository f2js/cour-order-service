# cour-order-service

## Status
[![CircleCI](https://dl.circleci.com/status-badge/img/gh/f2js/cour-order-service/tree/main.svg?style=svg&circle-token=a8e72aac924cefac309e010a2f8544fece7401cb)](https://dl.circleci.com/status-badge/redirect/gh/f2js/cour-order-service/tree/main)

[![CircleCI](https://dl.circleci.com/insights-snapshot/gh/f2js/cour-order-service/main/test_and_build/badge.svg?window=30d&circle-token=cdd48442d6194c13be97cb1f978dc6664525b07a)](https://app.circleci.com/insights/github/f2js/cour-order-service/workflows/test_and_build/overview?branch=main&reporting-window=last-30-days&insights-snapshot=true)

## REST API
### GET /cust/{id}
Gets all orders for a given customer. Does not fetch orderlines.

#### Response
- 200 OK: The orders were successfully found. The response body contains a list of the orders for the given customer.
- 404 Not Found: There was no orders found for the customer.
- 500 Internal Server Error: An error occurred on the server side.

## Database 
The service uses HBase as the database. Below is a sketch of the datamodel.

<table>
  <tr>
    <td><i>Column Family</i></td>
    <td rowspan="2"><b>rowkey</b></td>
    <td colspan="3"><b>info</b></td>
    <td colspan="2"><b>ids</b></td>
    <td colspan="2"><b>addr</b></td>
    <td colspan="6"><b>ol</b></td>
  </tr>
  <tr>
    <td><i>Column</i></td>
    <td><i><b>o_time</b></i></td>
    <td><i><b>state</b></i></td>
    <td><i><b>c_id</b></i></td>
    <td><i><b>r_id</b></i></td>
    <td><i><b>c_addr</b></i></td>
    <td><i><b>r_addr</b></i></td>
    <td><i><b>1</b></i></td>
    <td><i><b>2</b></i></td>
    <td><i><b>3</b></i></td>
    <td colspan="3"><i><b>...</b></i></td>
  </tr>
  <tr>
    <td><i>Content</i></td>
    <td>*</td>
    <td>DateTime of order creation</td>
    <td>Processing, Pending, Rejected, Accepted, ReadyForPickup, OutForDelivery, Delivered</td>
    <td>Mongo ObjectId</td>
    <td>Mongo ObjectId</td>
    <td>Customer address</td>
    <td>Restaurant address</td>
    <td>menuid:price**</td>
    <td>-||-</td>
    <td>-||-</td>
    <td>-||-</td>
    <td>-||-</td>
    <td>-||-</td>
  </tr>
  <tr>
    <td><i>Examples</i></td>
    <td></td>
    <td>2022-25-08 13:48:25</td>
    <td>Pending</td>
    <td>"507f1f77bcf86cd799439011"</td>
    <td>"507f191e810c19729de860ea"</td>
    <td>Lyngvej 2, 2800 Lyngby</td>
    <td>Lyngvej 2, 2800 Lyngby</td>
    <td>25:70</td>
    <td>12:60</td>
    <td>12:60</td>
    <td>5:52</td>
    <td>3:10</td>
    <td>1:15</td>
  </tr>
</table>
* sha256 of c_id, r_id, ordertime and all orderlines with random salt using r_id as seed appended to front, to make searching easier for restaurants

** price in cents/ører

## Kafka Events
### Consumed
#### OrderOutForDelivery
Updates the state of the given order to OutForDelivery in the database. 
##### Expected Body
- orderId (String): The ID of the order in the order-database. 
- courierId (String): The ID of the courier who will deliver the order. 

#### OrderDelivered
Updates the state of the given order to Delivered in the database. 
##### Expected Body
- orderId (String): The ID of the order in the order-database. 
- courierId (String): The ID of the courier who will deliver the order.