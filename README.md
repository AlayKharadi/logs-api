## Setup

- Current setup will create a logs folder and create a new `parquet` file with sorted logs within a minute folder. This can be modified to consider hour as base folder.


- Start the service using the below command:

```bash
cargo run
```

## Test (Initial setup)

- You can use the `setup-and-test.sh` bash script to populate the logs folder initially and test things out.

- There are two routes to test here, both of which can be tested using the below curl commands:

```bash
curl -X POST http://localhost:8080/ingest \
 -H "Content-Type: application/json" \
 -d '[
   {"time": 1685426705, "log": "System startup sequence initiated."},
   {"time": 1685426708, "log": "Auth service connection established."},
   {"time": 1685426715, "log": "User admin logged in successfully."},
   {"time": 1685426720, "log": "Configuration loaded from /etc/app/config.yaml"}
 ]'
```

```bash
curl "http://localhost:8080/query?start=1685426715&end=1685426715&text=admin"
```