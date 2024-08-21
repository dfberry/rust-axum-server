curl -X POST http://localhost:3000/github/repo \
    -H "Content-Type: application/json" \
    -d '{
        "token": "",
        "org_or_user": "dfberry",
        "repo": "azure-notes"
    }'