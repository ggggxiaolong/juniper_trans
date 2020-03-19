curl -X POST \
  --url http://34.221.26.139:4000/graphql \
  -H "content-type: application/json" \
  -H "token:eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyIjp7ImlkIjoxLCJ1c2VybmFtZSI6ImFkbWluIiwibWFpbCI6ImFkbWluQGdtYWlsLmNvbSIsInBhc3N3b3JkIjoiIn0sImlzX3JlZnJlc2giOmZhbHNlLCJleHAiOjE1ODQ1MjU4ODF9.n7leCHOtWQM409LeaH9H7sRMCQQEDxORhExWD7S6oaE" \
  --data-binary "@./web.txt" \
  >> web.json