import requests
import json

BASE_URL = "http://127.0.0.1:8080"


def pretty(response):
    try:
        print(json.dumps(response.json(), indent=2))
    except Exception:
        print(response.text)


def create_item(sku, quantity):
    print("\n▶ Creating item")
    payload = {
        "sku": sku,
        "total_quantity": quantity
    }
    r = requests.post(f"{BASE_URL}/items", json=payload)
    pretty(r)
    return r.json()["id"]


def list_items():
    print("\n▶ Listing items")
    r = requests.get(f"{BASE_URL}/items")
    pretty(r)


def reserve_item(item_id, qty):
    print(f"\n▶ Reserving {qty}")
    r = requests.post(f"{BASE_URL}/items/{item_id}/reserve/{qty}")
    print(r.text)


def release_item(item_id, qty):
    print(f"\n▶ Releasing {qty}")
    r = requests.post(f"{BASE_URL}/items/{item_id}/release/{qty}")
    print(r.text)


def commit_item(item_id, qty):
    print(f"\n▶ Committing {qty}")
    r = requests.post(f"{BASE_URL}/items/{item_id}/commit/{qty}")
    print(r.text)


if __name__ == "__main__":
    # Create inventory item
    item_id = create_item("SKU-ABC-123", 10)

    # Check inventory
    list_items()

    # Reserve stock
    reserve_item(item_id, 4)
    list_items()

    # Release some stock
    release_item(item_id, 2)
    list_items()

    # Commit remaining reserved stock
    commit_item(item_id, 2)
    list_items()
