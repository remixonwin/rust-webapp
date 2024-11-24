# API Documentation

## Overview

This document describes the REST API endpoints available in the Rust Web Application.

## Base URL

```
http://localhost:8080
```

## Endpoints

### Welcome Page

```http
GET /
```

Returns the welcome page HTML.

**Response**
- Status: 200 OK
- Content-Type: text/html

### Health Check

```http
GET /health
```

Check the health status of the service.

**Response**
- Status: 200 OK
- Content-Type: application/json

```json
{
    "content": "Service is healthy"
}
```

### Hello World

```http
GET /hello
```

Returns a simple hello world message.

**Response**
- Status: 200 OK
- Content-Type: application/json

```json
{
    "content": "Hello, World!"
}
```

### Echo

```http
POST /echo
```

Echoes back the received message.

**Request Body**
- Content-Type: application/json

```json
{
    "content": "Your message here"
}
```

**Response**
- Status: 200 OK
- Content-Type: application/json

```json
{
    "content": "Your message here"
}
```

## Error Responses

### Method Not Allowed

When an endpoint is accessed with an unsupported HTTP method.

**Response**
- Status: 405 Method Not Allowed
- Headers:
  - Allow: Allowed HTTP methods

### Not Found

When accessing a non-existent endpoint.

**Response**
- Status: 404 Not Found

## Response Headers

All responses include the following headers:
- `Content-Type`: Specifies the response format
- `X-Request-ID`: Unique request identifier (if enabled)
- `Server`: Actix-web

## Rate Limiting

Currently, there are no rate limits implemented.

## Authentication

Currently, there is no authentication required for any endpoints.

## Future Endpoints

Planned future endpoints include:
- User authentication
- Data persistence
- File uploads
- WebSocket support

## Testing the API

You can test the API using curl:

```bash
# Health check
curl http://localhost:8080/health

# Hello world
curl http://localhost:8080/hello

# Echo
curl -X POST http://localhost:8080/echo \
    -H "Content-Type: application/json" \
    -d '{"content":"test message"}'
```

## Best Practices

1. **Error Handling**
   - Always check response status codes
   - Handle errors gracefully
   - Log errors appropriately

2. **Request Format**
   - Use proper Content-Type headers
   - Validate request bodies
   - Keep requests concise

3. **Response Format**
   - Consistent JSON structure
   - Meaningful error messages
   - Appropriate status codes

## Versioning

The API is currently in version 1.0. Future versions will be handled through:
- URL versioning (e.g., /v2/endpoint)
- Accept header versioning

## Support

For API support:
- Create an issue in the GitHub repository
- Contact the development team
- Check the documentation

## Changes and Updates

All API changes will be documented in the release notes.
