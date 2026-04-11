# Inventory Web Service API Documentation

## Overview
The Inventory Web Service provides a set of APIs for managing inventory items.

## Authentication
All requests to the API must include an API key in the header:
```
Authorization: Bearer YOUR_API_KEY
```

## API Endpoints

### 1. Get All Items
- **Endpoint:** `/api/items`
- **Method:** `GET`
- **Description:** Retrieves a list of all inventory items.
- **Response:**
  - `200 OK`
  - Body: Array of items

### 2. Get Item By ID
- **Endpoint:** `/api/items/{id}`
- **Method:** `GET`
- **Description:** Retrieves a specific inventory item by ID.
- **Parameters:**
  - `id` (required): The ID of the item to retrieve.
- **Response:**
  - `200 OK`
  - Body: Item object

### 3. Create Item
- **Endpoint:** `/api/items`
- **Method:** `POST`
- **Description:** Creates a new inventory item.
- **Request Body:**
  ```json
  {
      "name": "Item Name",
      "description": "Item Description",
      "quantity": 10,
      "price": 5.99
  }
  ```
- **Response:**
  - `201 Created`
  - Body: Created item object

### 4. Update Item
- **Endpoint:** `/api/items/{id}`
- **Method:** `PUT`
- **Description:** Updates an existing inventory item.
- **Parameters:**
  - `id` (required): The ID of the item to update.
- **Request Body:** Same as the Create Item endpoint.
- **Response:**
  - `200 OK`
  - Body: Updated item object

### 5. Delete Item
- **Endpoint:** `/api/items/{id}`
- **Method:** `DELETE`
- **Description:** Deletes an inventory item by ID.
- **Parameters:**
  - `id` (required): The ID of the item to delete.
- **Response:**
  - `204 No Content`

## Error Handling
Common error responses include:
- `400 Bad Request`: Validation errors
- `401 Unauthorized`: Invalid API key
- `404 Not Found`: Item not found
- `500 Internal Server Error`: Unexpected error

## Conclusion
For further assistance, please contact the support team.