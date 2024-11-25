# Release Notes

## v0.2.0 (Current)

### Features
- Implemented JWT-based authentication system
- Added user registration and login endpoints
- Implemented rate limiting for authentication endpoints
- Added protected API endpoints
- Cross-Origin Resource Sharing (CORS) support

### Security
- Secure password hashing using bcrypt
- JWT token-based authentication
- Rate limiting to prevent brute force attacks
- Proper error handling and secure error messages

### Technical Improvements
- Modular code structure with separate auth module
- Improved testing with both unit and UI tests
- Docker support for containerized deployment
- Nginx configuration for production deployment

## v0.1.1

### What's New
- Improved SLSA provenance workflow configuration
- Updated to latest SLSA generator (v1.10.0)
- Enhanced permissions handling for secure artifact uploads
- Streamlined build and provenance process

### Security Improvements
- Added proper artifact handling with SHA256 verification
- Implemented secure provenance generation
- Enhanced GitHub Actions permissions model

### Technical Details
- Build artifacts are now properly organized
- Automated provenance generation on release
- Improved CI/CD pipeline reliability
