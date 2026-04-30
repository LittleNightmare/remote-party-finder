## Overview

## Base URL

```
http://<my-server>/api
```

## Endpoints

### 1. Get Listings

- **Endpoint:** `/listings`
- **Method:** `GET`
- **Query Parameters:**
  - `page` (optional): The page number to retrieve (default is 1).
  - `per_page` (optional): The number of listings per page (default is 20, maximum is 100).
  - `category` (optional): Filter listings by [category](https://github.com/LittleNightmare/remote-party-finder/blob/main/server/src/ffxiv/duties.rs#L14).
  - `world` (optional): Filter listings by world. e.g. `水晶塔`, `紫水栈桥`, `潮风亭`, `拉诺西亚`...
  - `search` (optional): Search for listings by name or description.
  - `datacenter` (optional): Filter listings by datacenter. Supports multiple values separated by commas. e.g. `豆豆柴,猫小胖`
  - `jobs` (optional): Filter listings by job IDs, supports multiple IDs separated by commas (e.g., `1,2,8`).
  - `duty` (optional): Filter listings by duty IDs, supports multiple IDs separated by commas (e.g., `1,2,8`).

- **Response:**
  - **Status Code:** `200 OK`
  - **Content:**
```json
{
   "data":[
      {
         "id": 1234567890,
         "name":"玩家名字",
         "description":"",
         "created_world":"潮风亭",
         "created_world_id":1167,
         "home_world":"潮风亭",
         "home_world_id":1167,
         "category":"None",
         "category_id":1,
         "duty":"无",
         "min_item_level":0,
         "slots_filled":1,
         "slots_available":8,
         "time_left":17.86,
         "updated_at":"2025-03-02T16:51:07.599+00:00",
         "is_cross_world":true,
         "datacenter":"莫古力"
      }
   ],
   "pagination":{
      "total":1786,
      "page":1,
      "per_page":20,
      "total_pages":90
   }
}
```

### 2. Get Listing Details

- **Endpoint:** `/listing/{id}`
- **Method:** `GET`
- **Path Parameters:**
  - `id`: The ID of the listing to retrieve.

- **Response:**
  - **Status Code:** `200 OK`
  - **Content:**
```json
{
  "id": 123456890,
  "name": "玩家名字",
  "description": "",
  "created_world": "红茶川",
  "home_world": "红茶川",
  "category": "None",
  "duty": "无",
  "min_item_level": 0,
  "slots_filled": 1,
  "slots_available": 8,
  "time_left": 8.55,
  "updated_at": "2025-03-02T16:59:15.667+00:00",
  "is_cross_world": true,
  "beginners_welcome": false,
  "duty_type": "Other",
  "objective": "NONE",
  "conditions": "NONE",
  "loot_rules": "NONE",
  "slots": [
    {
      "filled": true,
      "role": "DPS",
      "role_id": 3,
      "job": "BLM",
      "job_id": [25]
    },
    {
      "filled": false,
      "role": null,
      "role_id": 0,
      "job": "GLA PGL MRD LNC ARC CNJ THM PLD MNK WAR DRG BRD WHM BLM ACN SMN SCH ROG NIN MCH DRK AST SAM RDM BLU GNB DNC RPR SGE VPR PCT",
      "job_id": [1, 2, 3]
    },
    {
      "filled": false,
      "role": null,
      "job": "GLA PGL MRD LNC ARC CNJ THM PLD MNK WAR DRG BRD WHM BLM ACN SMN SCH ROG NIN MCH DRK AST SAM RDM BLU GNB DNC RPR SGE VPR PCT"
    },
    {
      "filled": false,
      "role": null,
      "job": "GLA PGL MRD LNC ARC CNJ THM PLD MNK WAR DRG BRD WHM BLM ACN SMN SCH ROG NIN MCH DRK AST SAM RDM BLU GNB DNC RPR SGE VPR PCT"
    },
    {
      "filled": false,
      "role": null,
      "job": "GLA PGL MRD LNC ARC CNJ THM PLD MNK WAR DRG BRD WHM BLM ACN SMN SCH ROG NIN MCH DRK AST SAM RDM BLU GNB DNC RPR SGE VPR PCT"
    },
    {
      "filled": false,
      "role": null,
      "job": "GLA PGL MRD LNC ARC CNJ THM PLD MNK WAR DRG BRD WHM BLM ACN SMN SCH ROG NIN MCH DRK AST SAM RDM BLU GNB DNC RPR SGE VPR PCT"
    },
    {
      "filled": false,
      "role": null,
      "job": "GLA PGL MRD LNC ARC CNJ THM PLD MNK WAR DRG BRD WHM BLM ACN SMN SCH ROG NIN MCH DRK AST SAM RDM BLU GNB DNC RPR SGE VPR PCT"
    },
    {
      "filled": false,
      "role": null,
      "job": "GLA PGL MRD LNC ARC CNJ THM PLD MNK WAR DRG BRD WHM BLM ACN SMN SCH ROG NIN MCH DRK AST SAM RDM BLU GNB DNC RPR SGE VPR PCT"
    }
  ],
  "datacenter": "豆豆柴"
}
```

- **Error Responses:**
  - **Status Code:** `404 Not Found`
    - **Content:**
      ```json
      {
        "error": "未找到招募信息"
      }
      ```
  - **Status Code:** `500 Internal Server Error`
    - **Content:**
      ```json
      {
        "error": "数据库查询错误"
      }
      ```
    - **Alternative content:**
      ```json
      {
        "error": "无法解析招募信息"
      }
      ```

## Examples

### Filter by multiple datacenters
```
/api/listings?datacenter=猫小胖,豆豆柴
```

### Filter by duty and jobs
```
/api/listings?duty=1006,1007,1017&jobs=8,10
```

### Filter by datacenter, category and jobs
```
/api/listings?datacenter=猫小胖&category=HighEndDuty&jobs=10,21
```

## Notes

- The API uses MongoDB for data storage and retrieval.
- The `time_left` field represents the remaining time for the listing in seconds.
- The `pagination` object provides information about the total number of listings, the current page, the number of listings per page, and the total number of pages available.
- Duty IDs refer to specific dungeons, trials, raids, etc. in the game. They can be found in the `duties.rs` file.
- Job IDs can be found in the `jobs.rs` file.
- For better cache performance, parameter order doesn't matter (e.g. `jobs=1,2,3` and `jobs=3,2,1` will yield the same results).

This documentation provides a comprehensive overview of the API's functionality, allowing developers to integrate and utilize the endpoints effectively.
