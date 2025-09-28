#!/bin/bash

# Simple API test script
# This would normally be run after starting the server

echo "Testing API endpoints (requires running server)..."

# Test health endpoint
echo "Testing health endpoint..."
curl -s http://localhost:3000/health | jq . || echo "Health endpoint test - server not running"

# Test API documentation
echo "Testing Swagger UI availability..."
curl -s -o /dev/null -w "%{http_code}" http://localhost:3000/swagger-ui || echo "Swagger UI test - server not running"

echo "To test the API:"
echo "1. Start services: docker-compose up -d"
echo "2. Wait for services to be ready"
echo "3. Run: ./test_api.sh"
echo ""
echo "Or start manually:"
echo "1. Set up .env file from .env.example"
echo "2. Start PostgreSQL and Redis"
echo "3. Run: cargo run"
echo "4. Visit http://localhost:3000/swagger-ui for API documentation"