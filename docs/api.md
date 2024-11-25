# API Documentation

## Overview

This document describes the REST API endpoints available in the Rust Web Application.

## Base URL

```
http://localhost:9000
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

### Authentication

#### Register
```http
POST /api/auth/register
Content-Type: application/json

{
    "email": "string",
    "password": "string"
}
```

Returns:
```json
{
    "message": "Registration successful",
    "token": "jwt-token"
}
```

#### Login
```http
POST /api/auth/login
Content-Type: application/json

{
    "email": "string",
    "password": "string"
}
```

Returns:
```json
{
    "message": "Login successful",
    "token": "jwt-token"
}
```

### Protected Endpoints

#### Hello (Protected)
```http
GET /api/hello
Authorization: Bearer <your-jwt-token>
```

Returns:
```json
{
    "message": "Hello from the protected API!"
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

### Authentication Errors
```json
{
    "message": "No authorization token provided"
}
```
or
```json
{
    "message": "Invalid token"
}
```

### Rate Limiting
```json
{
    "message": "Too many failed attempts. Please try again later.",
    "retry_after_seconds": 900
}
```

## Response Headers

All responses include the following headers:
- `Content-Type`: Specifies the response format
- `X-Request-ID`: Unique request identifier (if enabled)
- `Server`: Actix-web

## Rate Limiting

Currently, there are no rate limits implemented.

## Authentication

All protected endpoints require a JWT token in the Authorization header:

```
Authorization: Bearer <your-jwt-token>
```

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
curl http://localhost:9000/health

# Hello world
curl http://localhost:9000/hello

# Echo
curl -X POST http://localhost:9000/echo \
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
