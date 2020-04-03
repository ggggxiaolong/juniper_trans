curl -X POST \
  --url http://34.221.26.139:4000/graphql \
  -H "content-type: application/json" \
  -H "token:eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyIjp7ImlkIjoxLCJ1c2VybmFtZSI6ImFkbWluIiwibWFpbCI6ImFkbWluQGdtYWlsLmNvbSIsInBhc3N3b3JkIjoiIn0sImlzX3JlZnJlc2giOmZhbHNlLCJleHAiOjE1ODU4OTMzMTh9.UMdXdq_RuIlu5pbF0aN2NxDHRi7kjY0uuDirtIStRgk" \
  --data-binary "@./web.txt" \
  >> web.json