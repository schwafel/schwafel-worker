# schwafel-worker

worker api for text generation

## Usage

This template starts you off with a `src/lib.rs` file, acting as an entrypoint for requests hitting
your Worker. Feel free to add more code in this file, or create Rust modules anywhere else for this
project to use.

With `wrangler`, you can build, test, and deploy your Worker with the following commands:

```bash
# compiles your project to WebAssembly and will warn of any issues
wrangler build 

# set cors secret to https://www.example.com,http://127.0.0.1:3000,http://localhost:3000

wrangler secret put CORS_ORIGIN

# put api keys

wrangler secret put HF_TOKEN
wrangler secret put OPENAI_TOKEN

# run your Worker in an ideal development workflow (with a local server, file watcher & more)
wrangler dev

# deploy your Worker globally to the Cloudflare network (update your wrangler.toml file for configuration)
wrangler publish
```
