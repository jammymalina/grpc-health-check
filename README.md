# grpc-health-check

A lightweight, standalone gRPC health check binary for your containerized applications. This utility is designed to be used as a health probe in container orchestration systems like Kubernetes or AWS ECS.

It connects to a gRPC server, performs a health check, and exits with a status code indicating the health of the service. This makes it a suitable tool for `livenessProbe` and `readinessProbe` in your Kubernetes deployments or `healthCheck` in AWS ECS.

## How It Works

The application connects to a specified gRPC server and sends a `HealthCheckRequest`. It then checks the `ServingStatus` in the response.

*   If the status is `SERVING`, the application exits with a success code (`0`).
*   For any other status, it prints an error message and exits with a failure code (`1`), signaling to the container orchestrator that the service is unhealthy.

It is using the standard health check protobuf spec:

```proto
syntax = "proto3";

package grpc.health.v1;

message HealthCheckRequest { string service = 1; }

message HealthCheckResponse {
  enum ServingStatus {
    UNKNOWN = 0;
    SERVING = 1;
    NOT_SERVING = 2;
    SERVICE_UNKNOWN = 3;
  }
  ServingStatus status = 1;
}

service Health {
  rpc Check(HealthCheckRequest) returns (HealthCheckResponse);
  rpc Watch(HealthCheckRequest) returns (stream HealthCheckResponse);
}
```

## Using with Docker

`Dockerfile` to building a small, multi-stage container image:

```Dockerfile
# --- Builder Stage ---
FROM rust:1.91.0-slim-bookworm AS builder
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y protobuf-compiler
RUN cargo install grpc-health-check --root /usr/local/


# --- Main Stage ---
FROM gcr.io/distroless/cc-debian12
# Copy the health check binary from the builder stage
COPY --from=builder /usr/local/bin/grpc-health-check /usr/local/bin/grpc-health-check
```

## Configuration

You can configure the gRPC server's host and port using the following methods:

**1. Command-Line Arguments:**

| Argument | Short | Environment Variable | Description |
|---|---|---|---|
| `--host` | `-h` | `GRPC_HEALTH_CHECK_HOST` | The hostname or IP address of the gRPC server. |
| `--port` | `-p` | `GRPC_HEALTH_CHECK_PORT` | The port number of the gRPC server. |

**Example:**
```bash
./grpc-health-check --host http://localhost --port 50051
```

**2. Environment Variables:**

This is the recommended approach for use within containers.

```bash
export GRPC_HEALTH_CHECK_HOST=http://localhost
export GRPC_HEALTH_CHECK_PORT=50051
./grpc-health-check
```
