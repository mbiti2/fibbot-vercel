# FibBot Vercel API (Rust, Vercel Advanced Functions)

This project provides a serverless API for calculating Fibonacci numbers, built with Rust and deployed using Vercel Advanced Functions.

## API Usage

**Endpoint:** `/api/fib?number=10`

**Example Request:**
```
GET /api/fib?number=10
```

**Example Response:**
```json
{
  "number": 10,
  "fibonacci": 55
}
```

## Local Development

1. Install [Vercel CLI](https://vercel.com/docs/cli):
   ```sh
   npm i -g vercel
   ```
2. Run the local dev server:
   ```sh
   vercel dev
   ```
3. Visit: [http://localhost:3000/api/fib?number=10](http://localhost:3000/api/fib?number=10)

## Deployment

1. Push your code to GitHub.
2. Connect your repository to Vercel and deploy.

## Project Structure
- `api/fib.rs` — The Rust function handler for the Fibonacci API.
- `Cargo.toml` — Rust dependencies.
- `vercel.json` — Vercel function configuration.

---

Built with ❤️ using Rust and Vercel Advanced Functions. 