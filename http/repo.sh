curl -X POST http://localhost:4000/github/repo \
    -H "Content-Type: application/json" \
    -d '{
        "token": "",
        "org_or_user": "dfberry",
        "repo": "azure-notes"
    }'