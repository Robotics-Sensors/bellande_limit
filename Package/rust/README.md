# 📦 Bellande Limit

## 🧙 Organization Website
- [![Organization Website](https://img.shields.io/badge/Explore%20Our-Website-0099cc?style=for-the-badge)](https://robotics-sensors.github.io)

## 🧙 Organization Github
- [![Organization Github ](https://img.shields.io/badge/Explore%20Our-Github-0099cc?style=for-the-badge)](https://github.com/Robotics-Sensors)

# Author, Creator and Maintainer
- **Ronaldson Bellande**

## Bellande Limit Executables & Models
- [![Bellande Limit Models & Executables ](https://img.shields.io/badge/Bellande%20Step-Models/Executables-0099cc?style=for-the-badge)](https://github.com/Artificial-Intelligence-Computer-Vision/bellande_limit_models_executables)

# API HTTP Usability (BELLANDE FORMAT)
```
# Copyright (C) 2024 Bellande Robotics Sensors Research Innovation Center, Ronaldson Bellande
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
# 
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.

# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.
# GNU General Public License v3.0 or later

url: https://bellande-robotics-sensors-research-innovation-center.org

endpoint_path:
    bellande_limit: /api/Bellande_Limit/bellande_limit

Bellande_Framework_Access_Key: bellande_web_api_opensource
```

# API HTTP Usability (JSON FORMAT)
```
{
  "license": [
    "Copyright (C) 2024 Bellande Robotics Sensors Research Innovation Center, Ronaldson Bellande",
    "This program is free software: you can redistribute it and/or modify",
    "it under the terms of the GNU General Public License as published by",
    "the Free Software Foundation, either version 3 of the License, or",
    "(at your option) any later version.",
    "",
    "This program is distributed in the hope that it will be useful,",
    "but WITHOUT ANY WARRANTY; without even the implied warranty of",
    "MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the",
    "GNU General Public License for more details.",
    "",
    "You should have received a copy of the GNU General Public License",
    "along with this program.  If not, see <https://www.gnu.org/licenses/>.",
    "GNU General Public License v3.0 or later"
  ],
  "url": "https://bellande-robotics-sensors-research-innovation-center.org",
  "endpoint_path": {
    "bellande_limit": "/api/Bellande_Limit/bellande_limit"
  },
  "Bellande_Framework_Access_Key": "bellande_web_api_opensource"
}
```

# API Payload Example
```
{
    "node0": [0, 0, 0],
    "node1": [100, 100, 100],
    "environment": [1000, 1000, 1000],
    "size": [10, 10, 10],
    "goal": [200, 200, 200],
    "obstacles": [
      {
        "position": [50, 50, 50],
        "dimensions": [20, 20, 20]
      }
    ],
    "search_radius": 50,
    "sample_points": 20,
    "auth": {
      "authorization_key": "bellande_web_api_opensource"
    }
}
```

# 🧙 Website Bellande API Testing 
- [![Website API Testing](https://img.shields.io/badge/Bellande%20API-Testing-0099cc?style=for-the-badge)](https://bellande-robotics-sensors-research-innovation-center.org/api/bellande_limit_experiment)
  
# Quick Bellande API Testing
```
curl -X 'POST' \
  'https://bellande-robotics-sensors-research-innovation-center.org/api/Bellande_Limit/bellande_limit' \
  -H 'accept: application/json' \
  -H 'Content-Type: application/json' \
  -d '{
    "node0": [0, 0, 0],
    "node1": [100, 100, 100],
    "environment": [1000, 1000, 1000],
    "size": [10, 10, 10],
    "goal": [100, 100, 100],
        "obstacles": [
      {
        "position": [50, 50, 50],
        "dimensions": [20, 20, 20]
      }
    ],
    "search_radius": 50,
    "sample_points": 20,
    "auth": {
      "authorization_key": "bellande_web_api_opensource"
    }
  }'
```

# Bellande Limit Usage

## Website Crates
- https://crates.io/crates/bellande_limit

### Installation
- `cargo add bellande_limit`

## Website PYPI
- https://pypi.org/project/bellande_limit

### Installation
- `$ pip install bellande_limit`

### Usage 
```
bellande_limit_api \
  --node0 "[0,0,0]" \
  --node1 "[100,100,100]" \
  --environment "[1000,1000,1000]" \
  --size "[10,10,10]" \
  --goal "[200,200,200]" \
  --obstacles '[{"position":[50,50,50],"dimensions":[20,20,20]}]' \
  --search-radius 50 \
  --sample-points 20
```

### Upgrade (if not upgraded)
- `$ pip install --upgrade bellande_limit`

```
Name: bellande_limit
Summary: Computes the next step towards a target node
Home-page: github.com/RonaldsonBellande/bellande_limit
Author: Ronaldson Bellande
Author-email: ronaldsonbellande@gmail.com
License: GNU General Public License v3.0
```

## Published Paper
```
Coming Soon
```

## Preprint
- [![Preprint](https://img.shields.io/badge/Preprint-Bellande%20Step-0099cc?style=for-the-badge)](https://dapp.orvium.io/deposits/6650ccb8afb407dc8beb0ff2/view)


## License
This Algorithm or Models is distributed under the [Creative Commons Attribution-ShareAlike 4.0 International License](http://creativecommons.org/licenses/by-sa/4.0/), see [LICENSE](https://github.com/RonaldsonBellande/bellande_step/blob/main/LICENSE) and [NOTICE](https://github.com/RonaldsonBellande/bellande_step/blob/main/LICENSE) for more information.
