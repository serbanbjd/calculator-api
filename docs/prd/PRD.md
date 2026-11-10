# PRD: Calculator API

- Status: Draft, awaiting owner review
- Owner: Serban
- Date: 2026-11-10

## 1. Problem and goal

Clients that need arithmetic often get it wrong with binary floating point (`0.1 + 0.2 = 0.30000000000000004`).
The Calculator API offers a small set of arithmetic operations over HTTP with **exact decimal results**,
predictable rounding and clear errors, so any client can delegate calculations without precision surprises.

## 2. Users and use cases

- **Any HTTP client**: a UI, `curl`, scripts, other services or an AI agent calling it as a tool. The API makes no assumption about who calls it.
- **Developers and AI agents exploring the API** through interactive documentation in the browser.
- **The hosting platform** checking that the service is alive.

Use cases:
- UC-1: Compute one operation on one or two decimal numbers and get an exact result.
- UC-2: Get a machine-readable error explaining why a calculation was rejected.
- UC-3: Discover and try the API without reading the source code.

## 3. System boundary

**Inside:** an HTTP service that validates a single calculation request, computes it and returns the result or an error.
It also serves its own API documentation and a health check.

**Outside:**
- Callers and their UIs, retries and caching.
- Authentication, user accounts and rate limiting (the service is open; if needed, they belong in front of it, at a gateway).
- Persistence: the service keeps no state between requests.
- Hosting: TLS, domain and scaling are provided by the deployment platform.

**Depends on:** nothing at runtime (no database, no external services).

## 4. Functional requirements

All numbers in requests and responses are **JSON strings** containing a plain decimal number (for example `"0.1"`, `"-42"`).
Strings only, so no client or parser converts them to binary floats on the way. Scientific notation (`"1e3"`) is rejected with `INVALID_NUMBER`.

### FR-1 Calculate endpoint

`POST /v1/calculate` takes one operation and its operands and returns the result.

```json
// request
{ "op": "add", "a": "0.1", "b": "0.2" }
// 200 response
{ "result": "0.3" }
```

### FR-2 Binary operations

| `op` | Meaning | Example (`a`, `b` → result) |
|---|---|---|
| `add` | a + b | `"0.1"`, `"0.2"` → `"0.3"` |
| `subtract` | a − b | `"5"`, `"7.5"` → `"-2.5"` |
| `multiply` | a × b | `"1.5"`, `"4"` → `"6"` |
| `divide` | a ÷ b | `"1"`, `"3"` → `"0.3333333333333333333333333333"` |
| `modulo` | remainder of a ÷ b, sign of `a` | `"10"`, `"3"` → `"1"` |
| `power` | a raised to b; b must be an integer (`"2"`, `"0.5"` → `UNSUPPORTED_EXPONENT`) | `"2"`, `"10"` → `"1024"` |
| `percent` | a percent of b, i.e. a × b ÷ 100 | `"15"`, `"200"` → `"30"` |

Both `a` and `b` are required.

### FR-3 Unary operation

| `op` | Meaning | Example (`a` → result) |
|---|---|---|
| `sqrt` | square root of a | `"16"` → `"4"`, `"2"` → `"1.41421356…"` (rounded per FR-4) |

Only `a` is required; sending `b` is a validation error.

### FR-4 Precision and rounding

- Results are exact whenever the exact result fits in 28 significant digits.
- When it does not (for example `1 ÷ 3`, `sqrt(2)`), the result is rounded **half-to-even** to the available precision.
- Results are returned without trailing zeros (`"2.50"` + `"0"` → `"2.5"`). *(ASSUMPTION)*
- Input numbers with more precision than the service can represent are rejected, not silently rounded.

### FR-5 Errors

Every error uses one shape:

```json
{ "error": { "code": "DIVISION_BY_ZERO", "message": "Cannot divide by zero" } }
```

| Situation | HTTP status | `code` |
|---|---|---|
| Body is not valid JSON, or a field is missing or unexpected | 400 | `INVALID_REQUEST` |
| Unknown `op` | 400 | `UNKNOWN_OPERATION` |
| An operand is not a valid decimal number, or too precise | 400 | `INVALID_NUMBER` |
| Divide or modulo by zero | 422 | `DIVISION_BY_ZERO` |
| Square root of a negative number | 422 | `NEGATIVE_SQRT` |
| `power` with a non-integer exponent | 422 | `UNSUPPORTED_EXPONENT` |
| Result too large to represent | 422 | `OVERFLOW` |
| Body larger than 16 KB | 413 | `PAYLOAD_TOO_LARGE` |

Error responses never expose internal details.

### FR-6 Interactive API documentation

- `GET /docs` serves Swagger UI where every endpoint can be tried from the browser.
- `GET /openapi.json` serves the OpenAPI specification it is built from.
- The specification documents every endpoint, request field, response and error code.

### FR-7 Health check

`GET /health` returns `200` with `{ "status": "ok" }` while the service can serve requests.

## 5. Non-functional requirements

- **NFR-1 Stateless**: no data is stored between requests; any instance can serve any request.
- **NFR-2 Payload limit**: request bodies above 16 KB are rejected (FR-5).
- **NFR-3 Performance**: p95 latency under 50 ms for `/v1/calculate`, measured inside the service. *(ASSUMPTION)*
- **NFR-4 Portability**: runs as a single container image configured only through environment variables (`PORT`).
- **NFR-5 Deployability**: deployable on Railway, using `/health` as its health check.
- **NFR-6 No crashes on input**: no request, however malformed, can crash the service.

## 6. Non-goals

- Expression strings such as `"2*(3+4)"`: one operation per call only.
- History, memory or any server-side state.
- Authentication, accounts, rate limiting.
- Trigonometry, logarithms, complex numbers, units, variables.
- A user interface beyond Swagger UI.
- Batch requests (many operations in one call).

## 7. Assumptions and decisions

Assumptions are marked *(ASSUMPTION)* above. Owner decisions on earlier open questions (2026-11-10):

1. **Power with fractional exponents** (`2 ^ 0.5`) is rejected with `UNSUPPORTED_EXPONENT`, so every result is exact or rounded only per FR-4.
2. **Percent** means "a percent of b" (`a × b ÷ 100`).
3. **Scientific notation** (`"1e3"`) is not accepted.

## 8. Acceptance criteria

- AC-1 (FR-1, FR-2): `{"op":"add","a":"0.1","b":"0.2"}` returns `200 {"result":"0.3"}`.
- AC-2 (FR-2): every operation in the FR-2 table returns the listed example result.
- AC-3 (FR-3): `sqrt` of `"16"` returns `"4"`; `sqrt` with a `b` field returns `400 INVALID_REQUEST`.
- AC-4 (FR-4): `1 ÷ 3` returns 28 significant digits, rounded half-to-even.
- AC-5 (FR-5): each row of the error table is reproducible with a request and returns that status and code.
- AC-6 (FR-6): `/docs` loads Swagger UI and `/openapi.json` lists `/v1/calculate` and `/health`.
- AC-7 (FR-7, NFR-5): `/health` returns `200`, and the service runs on Railway using it.
- AC-8 (NFR-4): `docker run -e PORT=9000` serves on port 9000.
- AC-10 (FR-2, FR-5): `power` with `b = "0.5"` returns `422 UNSUPPORTED_EXPONENT`; operand `"1e3"` returns `400 INVALID_NUMBER`.
- AC-9 (NFR-6): fuzzed or malformed bodies never produce a `5xx` or a crash.
