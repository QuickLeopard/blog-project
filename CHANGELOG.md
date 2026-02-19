# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- HTTP API endpoints for user authentication (register, login)
- HTTP API endpoints for posts (get by id, list with pagination)
- RegisterUserRequest and LoginUserResponse DTOs
- InMemoryPostRepository for testing
- Pagination query parameters support
- API routes under /api scope

### Changed
- BlogService now uses repository pattern with dependency injection
- User model: password_hash field now skipped in serialization

## [0.1.0] - 2024-01-18

### Added
- Cargo workspace with 4 crates: blog-server, blog-client, blog-cli, blog-wasm
- gRPC proto definitions (blog.proto) with BlogService
- Clean architecture structure: domain, application, data, infrastructure, presentation
- Domain models: User, Post with request DTOs and custom errors (thiserror)
- SQL migrations for users and posts tables
- PostgreSQL database integration with sqlx
- Connection pooling (min 5, max 20 connections)
- Automatic migrations runner
- JWT infrastructure module
- Logging infrastructure with tracing
- Docker Compose setup with PostgreSQL 16
- Multi-stage Dockerfile for blog-server
- Environment variables configuration (.env.example)
- Health check endpoint (GET /api/health)
- actix-web HTTP server on port 8080
- Database connection and migration on startup

### Infrastructure
- Docker support with docker-compose.yml
- PostgreSQL 16 in Docker container
- Environment variable parameterization
- .gitignore configuration

## [0.0.1] - 2024-01-01

### Added
- Initial project setup
- Repository structure
