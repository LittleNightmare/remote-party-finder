# API v2, phase 1 contract

`/api/v2` is a new read surface that runs alongside the existing v1 API.

## Scope

Phase 1 exposes only these routes:

- `GET /api/v2/listings`
- `GET /api/v2/listings/{id}`

Phase 1 does not expose lookup routes. `/api/v2/lookups/*` does not exist. World names, duty names, category labels, job labels, and other lookup-backed text must be resolved outside this API.

## Contract rules

- v1 stays available. Do not treat v1 as deprecated or removed.
- v2 is IDs-only for lookup-backed references.
- `player_name` and `description` stay inline text.
- Unknown but well-formed filter ids return `200` with an empty `data` array.
- Unsupported legacy label filters return `400 invalid_query`.
- `/api/v2/listings/{id}` resolves the current active visible PF listing by listing id. It is an active-detail lookup alias, not a durable storage identity.
- Missing, expired, or non-visible ids return `404 not_found`.

## `GET /api/v2/listings`

Supported query parameters in phase 1:

- `page`
- `per_page`
- `search`
- `created_world_id`
- `home_world_id`
- `category_id`
- `duty_id`
- `job_ids`

Summary item shape:

```json
{
  "id": 900001,
  "player_name": "Alice",
  "description": "Need clear",
  "created_world_id": 1167,
  "home_world_id": 1167,
  "category_id": 64,
  "duty_id": 1234,
  "duty_type_id": 0,
  "min_item_level": 710,
  "slots_filled": 6,
  "slots_available": 8,
  "time_left_seconds": 1200,
  "updated_at": "2026-04-23T12:34:56Z",
  "is_cross_world": true,
  "beginners_welcome": false
}
```

Collection response example:

```json
{
  "data": [
    {
      "id": 900001,
      "player_name": "Alice",
      "description": "Need clear",
      "created_world_id": 1167,
      "home_world_id": 1167,
      "category_id": 64,
      "duty_id": 1234,
      "duty_type_id": 0,
      "min_item_level": 710,
      "slots_filled": 6,
      "slots_available": 8,
      "time_left_seconds": 1200,
      "updated_at": "2026-04-23T12:34:56Z",
      "is_cross_world": true,
      "beginners_welcome": false
    }
  ],
  "pagination": {
    "total": 1,
    "page": 1,
    "per_page": 20,
    "total_pages": 1
  }
}
```

Example requests:

- `GET /api/v2/listings`
- `GET /api/v2/listings?search=clear&job_ids=24,28`
- `GET /api/v2/listings?created_world_id=1167&category_id=64`

Avoid old label-based queries such as `world=` or `category=`. Use numeric ids instead.

## `GET /api/v2/listings/{id}`

Detail response example:

```json
{
  "data": {
    "id": 900001,
    "player_name": "Alice",
    "description": "Need clear",
    "created_world_id": 1167,
    "home_world_id": 1167,
    "category_id": 64,
    "duty_id": 1234,
    "duty_type_id": 0,
    "min_item_level": 710,
    "slots_filled": 6,
    "slots_available": 8,
    "time_left_seconds": 1200,
    "updated_at": "2026-04-23T12:34:56Z",
    "is_cross_world": true,
    "beginners_welcome": false,
    "objective_ids": [1, 4],
    "condition_ids": [2],
    "loot_rule_id": 3,
    "slots": [
      {
        "filled": true,
        "role_id": 3,
        "filled_job_id": 19,
        "accepted_job_ids": []
      },
      {
        "filled": false,
        "role_id": 2,
        "filled_job_id": null,
        "accepted_job_ids": [24, 28]
      }
    ]
  }
}
```

Use this route as a lookup for the current active PF listing id only. If a listing expires, becomes non-visible, or is replaced by a newer active row for the same PF id, this route follows the current active row.

## Migration guidance for external clients

If you already consume v1:

1. Keep existing v1 integrations running while you add v2 support.
2. Switch list and detail reads to `/api/v2/listings` and `/api/v2/listings/{id}`.
3. Replace label-based parsing with id-based parsing for worlds, duties, categories, jobs, objectives, conditions, loot rules, and slot roles.
4. Move label resolution into your own lookup tables, frontend assets, or another metadata source.
5. Treat `/api/v2/listings/{id}` as active listing lookup semantics, not as a stable historical key.

If you are starting fresh:

- Prefer v2 for new read clients that already have an external metadata source.
- Use v1 only if you still depend on inline labels from the old response shape.
