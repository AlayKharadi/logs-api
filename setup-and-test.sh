#!/bin/bash

# IMPORTANT_NOTE: generated through LLM, use it for local setup

# --- Configuration ---
BASE_URL="http://127.0.0.1:8080" # API's url

# --- Helper Functions ---

# Function to send an ingest POST request
# Usage: send_ingest "Description" 'JSON Payload'
send_ingest() {
  local description="$1"
  local json_payload="$2"
  echo "--- Sending Ingest: $description ---"
  curl -s -X POST "$BASE_URL/ingest" \
    -H "Content-Type: application/json" \
    -d "$json_payload"
  # -s silences curl progress meter
  echo # Add a newline for better readability
  echo # Add another newline
  sleep 0.2 # Small delay between requests (optional)
}

# Function to run a query GET request
# Usage: run_query "Description" "query_parameters"
run_query() {
  local description="$1"
  local query_params="$2"
  echo "--- Running Query: $description ---"
  echo "Query Params: $query_params"
  curl -s "$BASE_URL/query?$query_params"
  echo # Add a newline
  echo # Add another newline
  sleep 0.5 # Slightly longer delay for queries (optional)
}


# --- Main Script ---

echo "Starting Log Ingester Test Script..."
echo "Targeting Server: $BASE_URL"
echo

# =========================================
# == INGEST REQUESTS
# =========================================

send_ingest "Batch 1 (06:05 UTC)" '[
   {"time": 1685426705, "log": "System startup sequence initiated."},
   {"time": 1685426708, "log": "Auth service connection established."},
   {"time": 1685426715, "log": "User admin logged in successfully."},
   {"time": 1685426720, "log": "Configuration loaded from /etc/app/config.yaml"}
 ]'

send_ingest "Batch 2 (06:10 UTC)" '[
   {"time": 1685427010, "log": "WARN: Disk space running low on /data volume."},
   {"time": 1685427012, "log": "Processing job #1234 started."},
   {"time": 1685427018, "log": "User guest attempted login, failed due to invalid credentials."},
   {"time": 1685427025, "log": "Processing job #1234 completed successfully."}
 ]'

send_ingest "Batch 3 (Cross Hour 07:00 UTC)" '[
   {"time": 1685429995, "log": "INFO: Preparing hourly maintenance task."},
   {"time": 1685429998, "log": "Auth service reports high latency (avg 550ms)."},
   {"time": 1685430002, "log": "Hourly maintenance task started."},
   {"time": 1685430005, "log": "ERROR: Failed to connect to database replica db-slave-1. Timeout exceeded."},
   {"time": 1685430008, "log": "Retrying database connection to db-slave-1..."}
 ]'

send_ingest "Batch 4 (07:15 UTC)" '[
   {"time": 1685430901, "log": "Database connection to db-slave-1 successful after retry."},
   {"time": 1685430905, "log": "Hourly maintenance task completed."},
   {"time": 1685430910, "log": "WARN: User session timeout for 'admin' due to inactivity."},
   {"time": 1685430915, "log": "DEBUG: Cleaning up temporary files in /tmp."}
 ]'

send_ingest "Batch 5 (Single Log 06:30 UTC)" '[
   {"time": 1685428200, "log": "DEBUG: Cache invalidated for key user:admin"}
 ]'

send_ingest "Batch 6 (Empty)" '[]'

# Optional: Uncomment to test invalid timestamp (if server handles it)
send_ingest "Batch 7 (Invalid Timestamp)" '[ {"time": -100, "log": "This timestamp is invalid"} ]'


echo "--- Ingest phase complete. Pausing briefly before queries... ---"
sleep 2 # Wait 2 seconds

# =========================================
# == QUERY REQUESTS
# =========================================

run_query "Query 1: Specific log by exact time and text" \
  "start=1685426715&end=1685426715&text=admin"

run_query "Query 2: Range within first hour, common word 'service'" \
  "start=1685426700&end=1685429999&text=service"

run_query "Query 3: Range covering job processing, text 'job'" \
  "start=1685427010&end=1685427030&text=job"

run_query "Query 4: Range within first hour, empty text (all logs)" \
  "start=1685426700&end=1685427100&text="

run_query "Query 5: Range crossing hour boundary (06:59 to 07:00), text 'task'" \
  "start=1685429990&end=1685430010&text=task"

run_query "Query 6: Specific error message" \
  "start=1685430000&end=1685430010&text=ERROR"

run_query "Query 7: Wider range, text 'database'" \
  "start=1685430000&end=1685431000&text=database"

run_query "Query 8: Time range with no ingested logs" \
  "start=1685500000&end=1685500100&text=test"

run_query "Query 9: Text search that won't match anything" \
  "start=1685426700&end=1685431000&text=NoSuchWordHereDefinitely"

run_query "Query 10: Exact timestamp range matching the single log batch" \
  "start=1685428200&end=1685428200&text=DEBUG"

run_query "Query 11: Query covering multiple hours, empty text" \
  "start=1685426700&end=1685431000&text="

echo "--- Test Script Finished ---"