curl -X POST {{$dotenv %BACKEND_URL}}/github/repo \
    -H "Content-Type: application/json" \
    -d '{
        "token": "{{$dotenv %PAT}}",
        "org_or_user": "dfberry",
        "repo": "azure-notes"
    }'