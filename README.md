# Hulykvs

Hulypulse is a service that enables clients to share information on a “whiteboard”. Clients connected to the same “whiteboard” see data provided by other clients to the whiteboard.

The service is exposed as REST and WebSocket API.

**Usage scenarios:**

- user presence in a document
- user is “typing” event
- user cursor position in editor or drawing board
- service posts a process status

## Key
Key is a string that consists of one or multiple segments separated by some separator.
Example: foo/bar/baz.

It is possible to use wildcard keys to list or subscribe to values with this prefix.

Key may contain a special section (guard) $that separates public and private data. “Private” data is available when querying or subscribing by exact key.
Example foo/bar/$/private, this value can be queried by foo/bar/$/private or foo/bar/$/but not by foo/bar/

## Data
“Data” is an arbitrary JSON document.
Size of data is limited to some reasonable size

## API
Methods

GET - returns values of one key

LIST - returns values with given prefix until the “sentinel”

PUT - put value to the key
- Support CAS
- Support If-* headers

DELETE - delete value of the key

SUB - subscribe to key data + get initial state
Behavior identical to LIST

UNSUB - unsubscribe to key data


## HTTP API

```PUT /{workspace}/{key}```
- Input
        Body - data
        Content-Type: application/json (do we need something else?)
        Content-Length: optional
        Headers: TTL or absolute expiration time
            HULY-TTL
            HULY-EXPIRE-AT
        Conditional Headers
            If-*
- Output
        Status: 201
        No body

```PATCH /{workspace}/{key}```
- TODO (not in v1)

```DELETE /{workspace}/{key}```
- Output
    Status: 204

```GET /{workspace}/{key}```
- Output
    Status 200
    Content-type: application/json
    Body:
	- key
        - user
        - data
        - expiresAt ?

```GET /{workspace}?prefix={key}```
- Output
    Status 200
    Content-type: application/json
    Body (array):
        - key
        - user
        - data
        - expiresAt ?



## API v2
Create a key-value pair api

```PUT /api2/{workspace}/{namespace}/{key}```
Stores request payload as the value for the given key in the given namespace. Existing keys will be overwritten. Returns 204 (NoContent) on success.

**Optional headers:**

- `If-Match: *` — update only if the key exists
- `If-Match: <md5>` — update only if current value's MD5 matches
- `If-None-Match: *` — insert only if the key does not exist

Returns:
- `204` on successful insert or update
- `201` if inserted with `If-None-Match: *`
- `412` if the condition is not met
- `400` if headers are invalid


```GET /api2/{workspace}/{namespace}/{key}```
Retrieves the value for the given key in the given namespace. Returns 404 if the key does not exist.


```DELETE /api2/{workspace}/{namespace}/{key}```
Deletes the key-value pair for the given key in the given namespace. Returns 404 if the key does not exist, 204 (NoContent) on success, 404 if the key does not exist.


```GET /api2/{workspace}/{namespace}?[prefix=<prefix>]```
Retrieves all key-value pairs in the given namespace. Optionally, a prefix can be provided to filter the results. The following structure is returned:
```json
{
  "workspace": "workspace",
  "namespace": "namespace",
  "count": 3,
  "keys": ["key1", "key2", "keyN"]
}
```
## API (old)
workspace = "defaultspace"

Create a key-value pair

```POST /api/{namespace}/{key}```
Stores request payload as the value for the given key in the given namespace. Existing keys will be overwritten. Returs 204 (NoContent) on sucesss.

```GET /api/{namespace}/{key}```
Retrieves the value for the given key in the given namespace. Returns 404 if the key does not exist.

```DELETE /api/{namespace}/{key}```
Deletes the key-value pair for the given key in the given namespace. Returns 404 if the key does not exist, 204 (NoContent) on success, 404 if the key does not exist.

```GET /api/{namespace}?[prefix=<prefix>]```
Retrieves all key-value pairs in the given namespace. Optionally, a prefix can be provided to filter the results. The following structure is returned:
```json
{
  "namespace": "namespace",
  "count": 3,
  "keys": ["key1", "key2", "keyN"]
}
```

## Running
Pre-build docker images is available at: hardcoreeng/service_hulykvs:{tag}.

You can use the following command to run the image locally:
```bash
docker run -p 8094:8094 -it --rm hardcoreeng/service_hulykvs:{tag}"
```

If you want to run the service as a part of local huly development environment use the following command:
```bash
 export HULY_DB_CONNECTION="postgresql://root@huly.local:26257/defaultdb?sslmode=disable"
 docker run --rm -it --network dev_default -p 8094:8094 hardcoreeng/service_hulykvs:{tag}
```
This will run Hulykvs in the same network as the rest of huly services, and set the coackroach connection string to the one matching the local dev cockroach instance. 

You can then access hulykvs at http://localhost:8094.

## Authetication
Hulykvs uses bearer JWT token authetication. At the moment, it will accept any token signed by the hulykvs secret. The secret is set in the environment variable HULY_TOKEN_SECRET variable. 

## Configuration
The following environment variables are used to configure hulykvs:
   - ```HULY_DB_CONNECTION```: cockroachdb (postgres) connection string (default: postgresql://root@huly.local:26257/defaultdb?sslmode=disable)
   - ```HULY_DB_SCHEME```: database schema for the key-value store (default: hulykvs)
   - ```HULY_TOKEN_SECRET```: secret used to sign JWT tokens (default: secret)
   - ```HULY_BIND_HOST```: host to bind the server to (default: 0.0.0.0)
   - ```HULY_BIND_PORT```: port to bind the server to (default: 8094)
   - ```HULY_PAYLOAD_SIZE_LIMIT```: maximum size of the payload (default: 2Mb)

## Databse DDL
Database schema is created automatically on startup. Database objects are also created or migrated automatically on startup. 

## Todo (in no particular order)
- [ ] Optional value encryption
- [ ] HEAD request
- [ ] Conditional update (optimistic locking)
- [ ] Support for open telemetry
- [ ] Concurrency control for database migration (several instances of hulykvs are updated at the same time)
- [ ] TLS support
- [ ] Namespacee based access control
- [ ] Liveness/readiness probe endpoint 

## Contributing
Contributions are welcome! Please open an issue or a pull request if you have any suggestions or improvements.

## License
This project is licensed under EPL-2.0






