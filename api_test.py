import json
import os
import uuid

import requests

BASE_URL = os.getenv("BASE_URL", "http://127.0.0.1:8081")
AUTH_TOKEN = os.getenv("AUTH_TOKEN")


def _headers():
    if not AUTH_TOKEN:
        return {}
    return {"Authorization": f"Bearer {AUTH_TOKEN}"}


def pretty(response):
    print(f"Status: {response.status_code}")
    try:
        print(json.dumps(response.json(), indent=2))
    except Exception:
        print(response.text)


def create_item(item_id, sku, quantity):
    print("\n▶ Creating item")
    payload = {"id": item_id, "sku": sku, "quantity": quantity}
    r = requests.post(f"{BASE_URL}/items", json=payload, headers=_headers())
    pretty(r)
    return r


def list_items():
    print("\n▶ Listing items")
    r = requests.get(f"{BASE_URL}/items", headers=_headers())
    pretty(r)
    return r


def reserve_item(item_id, qty):
    print(f"\n▶ Reserving {qty}")
    r = requests.post(
        f"{BASE_URL}/items/{item_id}/reserve/{qty}",
        headers=_headers(),
    )
    pretty(r)
    return r


def release_item(item_id, qty):
    print(f"\n▶ Releasing {qty}")
    r = requests.post(
        f"{BASE_URL}/items/{item_id}/release/{qty}",
        headers=_headers(),
    )
    pretty(r)
    return r


def commit_item(item_id, qty):
    print(f"\n▶ Committing {qty}")
    r = requests.post(
        f"{BASE_URL}/items/{item_id}/commit/{qty}",
        headers=_headers(),
    )
    pretty(r)
    return r


if __name__ == "__main__":
    item_id = f"item-{uuid.uuid4().hex[:8]}"

    create_item(item_id, "SKU-ABC-123", 10)
    list_items()

    reserve_item(item_id, 4)
    list_items()

    release_item(item_id, 2)
    list_items()

    commit_item(item_id, 2)
    list_items()
