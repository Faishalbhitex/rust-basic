#!/bin/bash

# REST API Testing Script
# Make sure the server is running on localhost:3000

API_URL="http://127.0.0.1:3000"

echo "🧪 Testing REST API..."
echo "=================="

echo -e "\n1. GET all todos (should be empty initially):"
curl -s -X GET $API_URL/todos | jq '.'

echo -e "\n\n2. CREATE first todo:"
TODO1=$(curl -s -X POST $API_URL/todos \
  -H "Content-Type: application/json" \
  -d '{"title": "Learn Rust"}')
echo $TODO1 | jq '.'

echo -e "\n\n3. CREATE second todo:"
TODO2=$(curl -s -X POST $API_URL/todos \
  -H "Content-Type: application/json" \
  -d '{"title": "Build REST API"}')
echo $TODO2 | jq '.'

echo -e "\n\n4. CREATE third todo:"
TODO3=$(curl -s -X POST $API_URL/todos \
  -H "Content-Type: application/json" \
  -d '{"title": "Deploy to production"}')
echo $TODO3 | jq '.'

echo -e "\n\n5. GET all todos (should show all 3):"
curl -s -X GET $API_URL/todos | jq '.'

echo -e "\n\n6. GET specific todo (ID: 1):"
curl -s -X GET $API_URL/todos/1 | jq '.'

echo -e "\n\n7. UPDATE todo (mark as completed):"
curl -s -X PUT $API_URL/todos/1 \
  -H "Content-Type: application/json" \
  -d '{"completed": true}' | jq '.'

echo -e "\n\n8. UPDATE todo (change title and status):"
curl -s -X PUT $API_URL/todos/2 \
  -H "Content-Type: application/json" \
  -d '{"title": "Build Awesome REST API", "completed": true}' | jq '.'

echo -e "\n\n9. GET all todos (should show updates):"
curl -s -X GET $API_URL/todos | jq '.'

echo -e "\n\n10. DELETE a todo (ID: 3):"
curl -s -X DELETE $API_URL/todos/3 -w "HTTP Status: %{http_code}\n"

echo -e "\n\n11. GET all todos (should show 2 remaining):"
curl -s -X GET $API_URL/todos | jq '.'

echo -e "\n\n12. Try to GET deleted todo (should return 404):"
curl -s -X GET $API_URL/todos/3 -w "HTTP Status: %{http_code}\n"

echo -e "\n\n✅ Testing completed!"
echo "📁 Check todos.json file for persistent data"
